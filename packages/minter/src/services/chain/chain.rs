use std::{collections::{HashMap, HashSet}, time::Duration};
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
        icrc1::str_to_subaccount, 
        nat::nat_to_u128, 
        rng
    }
};

const TREASURY_SUBACCOUNT: &str = "TREASURY_SUBACCOUNT";

const MAX_LOG_ITEMS_PER_CALL: usize = 1024;
const MAX_IDS_PER_CALL: usize = 64;
const MAX_QUERY_CALLS: usize = 16;
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

        // 1st: collect all MEME NFTs minted since last block
        let (minted_tokens, owners) = Self::get_minted_nfts(
            meme_nft.canister_id, 
            block_ts, 
            &mut chain
        ).await;

        // if no NFT was minted in the interval, divide the block 
        // rewards to a number of randomly selected NFT owners
        let (tokens, owners, is_minted) = if minted_tokens.len() == 0 {
            let tokens = Self::randomly_select_nfts(
                meme_nft.canister_id
            ).await;

            let owners = Self::get_nft_owners(
                meme_nft.canister_id, 
                &tokens
            ).await;

            ic_cdk::println!("info: block rewards will be divided between {} lucky NFT owners", owners.len());
            
            (
                tokens,
                owners,
                false
            )
        }
        else {
            ic_cdk::println!("info: block rewards will be divided between {} NFT minters", owners.len());

            (
                minted_tokens,
                owners,
                true
            )
        };

        // 2nd: create transactions
        let block_reward = Self::calc_reward(chain.height);
        let transactions = Self::build_transactions(
            &owners, 
            block_reward, 
            team_fee_p,
            treasury_fee_p,
            admin,
            tokens.len() as u64, 
            is_minted,
            block_ts
        );

        let mut hasher = DblHasher::new();

        let tx_ids = transactions.iter()
            .map(|tx| tx.calc_id(&mut hasher))
            .collect();

        // 3rd: add a new block, and the transactions, to our chain
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
        BlockStorage::save(block_id.clone(), block);

        for (id, tx) in tx_ids.iter().zip(&transactions) {
            TransactionStorage::save(id.clone(), tx.clone());
        }

        // 4th: update state
        chain.height += 1;
        chain.last_block_id = block_id;

        state::mutate(|s| 
            s.set_chain(chain.clone())
        );

        // 5th: mint the MEME coins
        Self::distribute_rewards(
            meme_coin.canister_id, 
            &transactions
        ).await;

        // 6th: reschedule
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
    ) -> (Vec<u128>, HashMap<Account, Vec<u128>>) {
        let mut tokens: Vec<u128> = vec![];
        let mut minters: HashMap<Account, Vec<u128>> = HashMap::new();

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
                                                        let token_id = if let Some(tid) = tx.get("tid") {
                                                            if let Ok(tid) = tid.clone().as_nat() {
                                                                let token_id = nat_to_u128(tid);
                                                                ic_cdk::println!("info: 7mint block found: {}", token_id);
                                                                
                                                                tokens.push(
                                                                    token_id
                                                                );

                                                                token_id
                                                            }
                                                            else {
                                                                u128::MAX
                                                            }
                                                        }
                                                        else {
                                                            u128::MAX
                                                        };

                                                        if token_id != u128::MAX {
                                                            if let Some(to) = tx.get("to") {
                                                                if let Ok(to) = to.clone().as_array() {
                                                                    if let Ok(owner) = to[0].clone().as_blob() {
                                                                        if let Ok(subaccount) = to[1].clone().as_blob() {
                                                                            let owner = Principal::from_slice(owner.as_slice());
                                                                            let subaccount = subaccount.first_chunk::<32>()
                                                                                .unwrap().clone();
                                                                            let account = Account {
                                                                                owner, 
                                                                                subaccount: Some(subaccount)
                                                                            };
                                                                            if let Some(tokens) = minters.get_mut(&account) {
                                                                                tokens.push(token_id);
                                                                            }
                                                                            else {
                                                                                minters.insert(account, vec![token_id]);
                                                                            }
                                                                        }
                                                                    }
                                                                }
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

        (tokens, minters)
    }

    async fn get_nft_owners(
        nft_canister_id: Principal,
        token_ids: &Vec<u128>
    ) -> HashMap<Account, Vec<u128>> {

        let mut calls = vec![];
        let mut args = vec![];

        for ids in token_ids.chunks(MAX_IDS_PER_CALL) {
            calls.push(ic_cdk::call::<(Vec<u128>, ), (Vec<Option<Account>>, )>(
                    nft_canister_id, 
                    "icrc7_owner_of", 
                    (ids.to_vec(), )
                ).boxed(),
            );
            args.push((ids, ));
        }

        let mut owners : HashMap<Account, Vec<u128>> = HashMap::new();

        for (c, chunk) in calls.chunks_mut(MAX_QUERY_CALLS).enumerate() {
            let calls_res = future::join_all(chunk).await;
        
            for (i, res) in calls_res.iter().enumerate() {
                match res {
                    Ok(res) => {
                        let args = args[c * MAX_QUERY_CALLS + i];
                        for (j, owner) in res.0.iter().enumerate() {
                            let token_id = args.0[j];
                            if let Some(account) = owner {
                                if let Some(tokens) = owners.get_mut(account) {
                                    tokens.push(token_id);
                                }
                                else {
                                    owners.insert(account.clone(), vec![token_id]);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        ic_cdk::println!("error: calling icrc7_owner_of: {}", err.1);
                    }
                }
            }
        }

        owners
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
        owners: &HashMap<Account, Vec<u128>>,
        block_reward: u64,
        team_fee_p: u64,
        treasury_fee_p: u64,
        admin: Principal,
        num_tokens: u64,
        is_minted: bool,
        block_ts: u64
    ) -> Vec<Transaction> {
        let timestamp = (block_ts / 1_000_000_000) as u32;

        let team_fee = (block_reward * team_fee_p) / (100_000_000);

        let treasury_fee = if owners.len() > 0 {
            (block_reward * treasury_fee_p) / (100_000_000)
        }
        else {
            block_reward - team_fee
        };

        let mut txs = vec![
            Transaction{
                version: TX_V1_00,
                op: TransactionOp::Mint { 
                    to: admin.into(), 
                    amount: team_fee,
                    reason: MintReason::TeamFee
                },
                timestamp,
            },
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

        for (owner, tokens) in owners {
            let amount = (block_reward * tokens.len() as u64) / num_tokens;
            if amount > 0 {
                txs.push(Transaction{
                    version: TX_V1_00,
                    op: TransactionOp::Mint { 
                        to: owner.clone(), 
                        amount,
                        reason: if is_minted {
                            MintReason::NftMinter(tokens.clone())
                        }
                        else {
                            MintReason::RaffleWinner
                        },
                    },
                    timestamp,
                });
            }
        }

        txs
    }
    
    async fn randomly_select_nfts(
        canister_id: Principal
    ) -> Vec<u128> {
        let total_supply = match ic_cdk::call::<((), ), (u128, )>(
            canister_id, 
            "icrc7_total_supply", 
            ((), )
        ).await {
            Ok(total) => {
                total.0 as usize
            },
            Err(err) => {
                ic_cdk::println!("error: calling icrc7_total_supply: {}", err.1);
                0
            },
        };

        let total = total_supply.min(
            state::read(|s| s.config().max_owners_raffle as usize)
        );
        let mut token_ids = HashSet::new();
        while token_ids.len() < total {
            let token_id = rng::gen_range(0..total_supply) as u128;
            token_ids.insert(token_id);
        }

        token_ids.iter()
            .cloned()
            .collect()
    }
}