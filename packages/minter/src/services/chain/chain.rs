use std::{collections::{BTreeMap, HashSet}, time::Duration, u64};
use candid::{Nat, Principal};
use icrc7_types::icrc3_types::{
    GetBlocksArgs, GetBlocksResult, TransactionRange
};
use icrc_ledger_types::{icrc::generic_value::Value, icrc1::{
    account::Account, transfer::{TransferArg, TransferError}
}};
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
const MAX_IDS_PER_CALL: usize = 128;
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
        let rewards_tiers = state::read(|s| s.config().block_reward_tiers.clone());

        // 1st: collect all MEME NFTs minted since last block
        let owners = Self::get_minted_nfts(
            meme_nft.canister_id, 
            block_ts, 
            &mut chain
        ).await;

        // if no NFT was minted in the interval, divide the block 
        // rewards to a number of randomly selected NFT owners
        let (mut owners, is_minted) = if owners.len() == 0 {
            let tokens = Self::randomly_select_nfts(
                meme_nft.canister_id
            ).await;

            let owners = Self::get_nft_owners(
                meme_nft.canister_id, 
                &tokens
            ).await;

            ic_cdk::println!("info: block rewards will be divided between {} lucky NFT owners", owners.len());
            
            (
                owners,
                false
            )
        }
        else {
            ic_cdk::println!("info: block rewards will be divided between {} NFT minters", owners.len());

            (
                owners,
                true
            )
        };

        // 2nd: sort by reactions (desc), token_id (asc)
        owners.sort_by(|a, b| a.1.cmp(&b.1));
        owners.sort_by(|a, b| b.2.cmp(&a.2));

        // 3rd: select the ones at the top reward tiers
        let owners = owners.iter()
            .take(rewards_tiers.len())
            .map(|m| m.0.clone())
            .collect();

        // 4th: create transactions
        let block_reward = Self::calc_reward(chain.height);
        let transactions = Self::build_transactions(
            &owners, 
            block_reward, 
            rewards_tiers,
            team_fee_p,
            treasury_fee_p,
            admin,
            is_minted,
            block_ts
        );

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
                                                        if let Some(token_id) = Self::get_tx_id(&tx) {
                                                            ic_cdk::println!("info: 7mint block found: {}", token_id);

                                                            if let Some(account) = Self::get_tx_to(&tx) {
                                                                let num_reactions = if let Some(meta) = Self::get_tx_meta(&tx) {
                                                                    if let Some(reactions) = Self::get_meta_reactions(&meta) {
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

    async fn get_nft_owners(
        nft_canister_id: Principal,
        tokens: &Vec<u128>
    ) -> Vec<(Account, u128, u32)> {

        let mut owner_of_calls = vec![];
        let mut metadata_calls = vec![];
        let mut args = vec![];

        for ids_chunk in tokens.chunks(MAX_IDS_PER_CALL) {
            owner_of_calls.push(
                ic_cdk::call::<(Vec<u128>, ), (Vec<Option<Account>>, )>(
                    nft_canister_id, 
                    "icrc7_owner_of", 
                    (ids_chunk.to_vec(), )
                ).boxed()
            );

            metadata_calls.push(
                ic_cdk::call::<(Vec<u128>, ), (Vec<Option<Value>>, )>(
                    nft_canister_id, 
                    "icrc7_token_metadata", 
                    (ids_chunk.to_vec(), )
                ).boxed()
            );

            args.push((ids_chunk, ));
        }

        let mut owners : Vec<(Account, u128, u32)> = Vec::new();

        for (c, chunk) in owner_of_calls.chunks_mut(MAX_QUERY_CALLS).enumerate() {
            let metadata_res = future::join_all(
                metadata_calls.drain(0..chunk.len())
            ).await;
            let owner_of_res = future::join_all(
                chunk
            ).await;
        
            for (i, res) in owner_of_res.iter().zip(metadata_res).enumerate() {
                match res {
                    (Ok(owner_res), Ok(metadata_res)) => {
                        let args = args[c * MAX_QUERY_CALLS + i];
                        for (j, account) in owner_res.0.iter().enumerate() {
                            let token_id = args.0[j];
                            let num_reactions = if let Some(metadata) = &metadata_res.0[j] {
                                if let Ok(metadata) = metadata.clone().as_map() {
                                    Self::get_meta_reactions(&metadata).unwrap_or(0)
                                }
                                else {
                                    0
                                }
                            }
                            else {
                                0
                            };

                            if let Some(account) = account {
                                owners.push((
                                    account.clone(), 
                                    token_id,
                                    num_reactions
                                ));
                            }
                        }
                    }
                    (Err(err0), Err(err1)) => {
                        ic_cdk::println!("error: calling icrc7_owner_of and icrc7_token_metadata: {} and {}", err0.1, err1.1);
                    }
                    (Err(err), _) => {
                        ic_cdk::println!("error: calling icrc7_owner_of: {}", err.1);
                    }
                    (_, Err(err)) => {
                        ic_cdk::println!("error: calling icrc7_token_metadata: {}", err.1);
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
        owners: &Vec<Account>,
        block_reward: u64,
        rewards_tiers: Vec<u64>,
        team_fee_p: u64,
        treasury_fee_p: u64,
        admin: Principal,
        is_minter: bool,
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
                        reason: if is_minter {
                            MintReason::TopNftMinter
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

        let tiers = state::read(|s| s.config().block_reward_tiers.clone());
        let total = tiers.len().min(total_supply);
        let mut token_ids = HashSet::new();
        while token_ids.len() < total {
            let token_id = rng::gen_range(0..total_supply) as u128;
            token_ids.insert(token_id);
        }

        token_ids.iter()
            .cloned()
            .collect()
    }

    fn get_tx_id(
        tx: &BTreeMap<String, Value>
    ) -> Option<u128> {
        if let Some(tid) = tx.get("tid") {
            if let Ok(tid) = tid.clone().as_nat() {
                Some(nat_to_u128(tid))
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    #[allow(unused)]
    fn get_tx_ts(
        tx: &BTreeMap<String, Value>
    ) -> Option<u64> {
        if let Some(ts) = tx.get("ts") {
            if let Ok(ts) = ts.clone().as_nat() {
                Some(nat_to_u128(ts) as u64)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    fn get_tx_meta(
        tx: &BTreeMap<String, Value>
    ) -> Option<BTreeMap<String, Value>> {
        if let Some(meta) = tx.get("meta") {
            if let Ok(meta) = meta.clone().as_map() {
                Some(meta)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    fn get_meta_reactions(
        meta: &BTreeMap<String, Value>
    ) -> Option<u32> {
        if let Some(reactions) = meta.get("Reactions") {
            if let Ok(num) = reactions.clone().as_nat() {
                Some(nat_to_u128(num) as u32)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    fn get_tx_to(
        tx: &BTreeMap<String, Value>
    ) -> Option<Account> {
        if let Some(to) = tx.get("to") {
            if let Ok(to) = to.clone().as_array() {
                if let Ok(owner) = to[0].clone().as_blob() {
                    if let Ok(subaccount) = to[1].clone().as_blob() {
                        let owner = Principal::from_slice(owner.as_slice());
                        let subaccount = subaccount.first_chunk::<32>()
                            .unwrap().clone();
                        return Some(Account {
                            owner, 
                            subaccount: Some(subaccount)
                        });
                    }
                }
            }
        }

        None
    }
}

