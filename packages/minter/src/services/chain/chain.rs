use std::time::Duration;
use candid::{Nat, Principal};
use icrc7_types::icrc3_types::{
    GetBlocksArgs, GetBlocksResult, TransactionRange
};
use icrc_ledger_types::icrc1::{
    account::Account, transfer::{TransferArg, TransferError}
};
use futures::{future, FutureExt};
use crate::{
    state, 
    storage::{
        block::BlockStorage, 
        transaction::TransactionStorage
    }, 
    types::{
        block::{Block, BlockHeader, BLOCK_V1_00}, 
        chain::{
            Chain, 
            BLOCKS_PER_HALVING, BLOCK_TIME, INIT_BLOCK_REWARD
        },
        transaction::{
            MintReason, Transaction, TransactionOp, TX_V1_00
        }
    }, 
    utils::{
        hasher::DblHasher, 
        icrc1::{
            get_meta_reactions, get_tx_id, get_tx_meta, 
            get_tx_to, str_to_subaccount
        }, 
        rng
    }
};

const TREASURY_SUBACCOUNT: &str = "TREASURY_SUBACCOUNT";

const MAX_LOG_ITEMS_PER_CALL: usize = 1024;
const MAX_UPDATE_CALLS: usize = 4;

pub struct ChainService;

impl ChainService {
    pub async fn process(
    ) {
        let (
                meme_nft, 
                meme_coin,
                admin,
                team_fee_p,
                treasury_fee_p,
                mut chain
            ) = state::read(|s| {
                let config = s.config();
            
                (
                    config.meme_nft.clone(),
                    config.meme_coin.clone(),
                    config.administrator.clone(),
                    config.team_fee,
                    config.treasury_fee,
                    s.chain().clone()
                )
        });

        let block_ts = ic_cdk::api::time();
        let rewards_tiers = state::read(|s| s.config().block_reward_tiers.clone());

        // 1st: collect all MEME NFTs minted since last block
        let mut tokens = Self::get_minted_nfts(
            meme_nft.canister_id, 
            block_ts, 
            &mut chain
        ).await;

        let block_reward = Self::calc_reward(chain.height);
        
        // any NFT minted since last block? 
        let transactions = if tokens.len() > 0 {
            // 2nd: sort by reactions (desc), token_id (asc)
            tokens.sort_by(|a, b| a.1.cmp(&b.1));
            tokens.sort_by(|a, b| b.2.cmp(&a.2));

            // 3rd: select the ones at the top reward tiers
            let accounts = tokens.iter()
                .take(rewards_tiers.len())
                .map(|m| m.0.clone())
                .collect();

            // 4th: create transactions
            let block_reward = chain.accumulated_reward + block_reward;
            chain.accumulated_reward = 0;

            Self::build_transactions(
                &accounts, 
                block_reward, 
                rewards_tiers,
                team_fee_p,
                treasury_fee_p,
                admin,
                block_ts
            )
        }
        // no NFT minted, accumulate the reward for the next block..
        else {
            chain.accumulated_reward += block_reward;

            vec![]
        };

        let mut hasher = DblHasher::new();

        let tx_ids = transactions.iter()
            .map(|tx| tx.calc_id(&mut hasher))
            .collect();

        // 5th: add a new block, and the transactions, to our chain
        let block = Block {
            height: chain.height,
            header: BlockHeader {
                version: BLOCK_V1_00,
                prev: chain.last_block_id.clone(),
                merkle_root: Block::calc_merkle_root(&tx_ids),
                timestamp: (block_ts / 1_000_000_000) as u32,
                bits: 0xffffffff, // difficulty = 0
                nonce: rng::gen(),
            },
            txs: tx_ids.clone(),
        };

        let block_id = block.calc_id(&mut hasher);
        ic_cdk::println!("Block {} added at height {} with {} txs", hex::encode(&block_id.inner), chain.height, block.txs.len());

        BlockStorage::save(block_id.clone(), block);

        for (id, tx) in tx_ids.iter().zip(&transactions) {
            TransactionStorage::save(id.clone(), tx.clone());
        }

        // 6th: update state
        chain.height += 1;
        chain.last_block_id = block_id;

        state::mutate(|s| 
            s.set_chain(chain.clone())
        );

        // 7th: mint the MEME coins
        Self::distribute_rewards(
            meme_coin.canister_id, 
            &transactions
        ).await;

        // 8th: reschedule
        ic_cdk_timers::set_timer(
            Duration::from_secs(BLOCK_TIME), 
            || ic_cdk::spawn(Self::process())
        );
    }
    
    fn calc_reward(
        height: u32
    ) -> u64 {
        INIT_BLOCK_REWARD >> (height as u64 / BLOCKS_PER_HALVING)
    }

    async fn get_minted_nfts(
        nft_canister_id: Principal,
        block_ts: u64,
        chain: &mut Chain
    ) -> Vec<(Account, u128, u32)> {
        let mut minters: Vec<(Account, u128, u32)> = Vec::new();

        loop {
            match ic_cdk::call::<(GetBlocksArgs, ), (GetBlocksResult, )>(
                nft_canister_id, 
                "icrc3_get_blocks", 
                (vec![TransactionRange { 
                    start: chain.next_nft_block_log_id, 
                    length: MAX_LOG_ITEMS_PER_CALL as _
                }], )
            ).await {
                Ok(res) => {
                    let blocks = &res.0.blocks;
                    
                    if blocks.len() > 0 {
                        ic_cdk::println!("info: analizying {} blocks", blocks.len());
                    }
                    
                    let mut ts = 0;
                    for log in blocks {
                        if let Ok(block) = log.block.clone().as_map() {
                            if let Some(timestamp) = block.get("ts") {
                                if let Ok(timestamp) = timestamp.clone().as_nat() {
                                    ts = timestamp.0.to_u64_digits()[0];
                                }
                            }
                            
                            if ts < block_ts {
                                if let Some(btype) = block.get("btype") {
                                    if let Ok(btype) = btype.clone().as_text() {
                                        match btype.as_str() {
                                            "7mint" => {
                                                if let Some(tx) = block.get("tx") {
                                                    if let Ok(tx) = tx.clone().as_map() {
                                                        if let Some(token_id) = get_tx_id(&tx) {
                                                            ic_cdk::println!("info: 7mint block found: {}", token_id);

                                                            if let Some(account) = get_tx_to(&tx) {
                                                                let num_reactions = if let Some(meta) = get_tx_meta(&tx) {
                                                                    if let Some(reactions) = get_meta_reactions(&meta) {
                                                                        reactions
                                                                    }
                                                                    else {
                                                                        0
                                                                    }
                                                                }
                                                                else {
                                                                    0
                                                                };

                                                                minters.push((account, token_id, num_reactions));
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            _ => {
                                            }
                                        }
                                    }
                                }

                                chain.next_nft_block_log_id = log.id + 1;
                            }
                            else {
                                break;
                            }
                        }
                    }

                    if blocks.len() < MAX_LOG_ITEMS_PER_CALL || ts >= block_ts {
                        break;
                    }
                },
                Err(err) => {
                    ic_cdk::println!("error: calling icrc3_get_blocks: {}", err.1);
                    break;
                }
            }
        }

        minters
    }

    async fn distribute_rewards(
        coin_canister_id: Principal,
        transactions: &Vec<Transaction>
    ) {
        let mut calls = vec![];
        for tx in transactions {
            match tx.op {
                TransactionOp::Mint { to, amount, .. } => {
                    if amount > 0 {
                        let timestamp = (tx.timestamp as u64) * 1_000_000_000;
                        calls.push(ic_cdk::call::<(TransferArg, ), (Result<Nat, TransferError>, )>(
                            coin_canister_id, 
                            "icrc1_transfer", 
                            (TransferArg {
                                amount: amount.into(),
                                from_subaccount: None,
                                to: to.clone(),
                                memo: None,
                                fee: None,
                                created_at_time: Some(timestamp),
                            },)
                        ).boxed());
                    }
                },
            }
        }

        for chunk in calls.chunks_mut(MAX_UPDATE_CALLS) {
            let calls_res = future::join_all(chunk).await;

            for res in calls_res {
                match res {
                    Ok(res) => {
                        match res.0 {
                            Ok(tx_id) => {
                                ic_cdk::println!("info: MEME coin minted: {}", tx_id);
                            },
                            Err(err) => {
                                ic_cdk::println!("error: calling icrc1_transfer: {}", err.to_string());        
                            }
                        }
                    },
                    Err(err) => {
                        ic_cdk::println!("error: calling icrc1_transfer: {}", err.1);
                    },
                }
            }
        }
    }
    
    fn build_transactions(
        owners: &Vec<Account>,
        block_reward: u64,
        rewards_tiers: Vec<u64>,
        team_fee_p: u64,
        treasury_fee_p: u64,
        admin: Principal,
        block_ts: u64
    ) -> Vec<Transaction> {
        let timestamp = (block_ts / 1_000_000_000) as u32;

        let team_fee = (block_reward * team_fee_p) / (100_000_000);
        let treasury_fee = (block_reward * treasury_fee_p) / (100_000_000);

        let mut txs = vec![
            // team fee
            Transaction{
                version: TX_V1_00,
                op: TransactionOp::Mint { 
                    to: admin.into(), 
                    amount: team_fee,
                    reason: MintReason::TeamFee
                },
                timestamp,
            },
            // treasury fee
            Transaction{
                version: TX_V1_00,
                op: TransactionOp::Mint { 
                    to: Account{ 
                        owner: ic_cdk::id(), 
                        subaccount: Some(str_to_subaccount(TREASURY_SUBACCOUNT)) 
                    }, 
                    amount: treasury_fee,
                    reason: MintReason::TreasuryFee
                },
                timestamp,
            }
        ];

        let block_reward = block_reward - (team_fee + treasury_fee);

        for (i, owner) in owners.iter().enumerate() {
            let tier_p = rewards_tiers.get(i).cloned().unwrap_or(0);
            let amount = (block_reward * tier_p) / 1_00000000;
            if amount > 0 {
                txs.push(Transaction{
                    version: TX_V1_00,
                    op: TransactionOp::Mint { 
                        to: owner.clone(), 
                        amount,
                        reason: MintReason::TopNftMinter,
                    },
                    timestamp,
                });
            }
        }

        txs
    }
}

