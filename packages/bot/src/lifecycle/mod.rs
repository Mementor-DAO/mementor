use std::cell::RefCell;
use ic_ledger_types::DEFAULT_SUBACCOUNT;
use crate::{
    services::{
        fund::{FundCanisterConfig, FundService}, meme, nft::{self}
    }, 
    state::{self, State}, 
    types::init::InitOrUpgradeArgs, 
    utils::rng
};

pub mod init;
pub mod post_upgrade;
pub mod pre_upgrade;

const READER_WRITER_BUFFER_SIZE: usize = 10 * 1024 * 1024; // 10MB

const MIN_CYCLES: u128 = 1_000_000_000_000;
const FUND_CYCLES: u128 =  500_000_000_000;

thread_local! {
    static FUND_SERVICE: RefCell<FundService> = RefCell::new(FundService::new());
}

pub(crate) fn setup(
    state: State,
    args: InitOrUpgradeArgs
) -> Result<(), String> {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);

    // fund service (to auto top-up our canisters)
    FUND_SERVICE.with_borrow_mut(|service| {
        service.start(
            vec![
                FundCanisterConfig { 
                    canister_id: ic_cdk::id(), 
                    from_subaccount: DEFAULT_SUBACCOUNT, 
                    min_cycles: MIN_CYCLES * 5, 
                    fund_cycles: FUND_CYCLES * 2,
                },
                FundCanisterConfig { 
                    canister_id: args.meme_nft.canister_id, 
                    from_subaccount: DEFAULT_SUBACCOUNT, 
                    min_cycles: MIN_CYCLES, 
                    fund_cycles: FUND_CYCLES,
                },
                FundCanisterConfig { 
                    canister_id: args.meme_coin.canister_id, 
                    from_subaccount: DEFAULT_SUBACCOUNT, 
                    min_cycles: MIN_CYCLES, 
                    fund_cycles: FUND_CYCLES,
                },
                FundCanisterConfig { 
                    canister_id: args.meme_coin_config.minter_canister_id, 
                    from_subaccount: DEFAULT_SUBACCOUNT, 
                    min_cycles: MIN_CYCLES, 
                    fund_cycles: FUND_CYCLES,
                },
            ],
            15 * 60 // every 15 minutes
        );
    });

    // start the meme service
    meme::init(
        args.memes_json_bytes, 
        args.index_tar_bytes
    );

    // create the nft service
    nft::mutate(|s| s.update(
        args.meme_nft_config,
        args.meme_nft
    ));

    // init random
    rng::init(state.rng_seed());
    
    state::init(state);

    Ok(())
}
