use std::{cell::RefCell, time::Duration};
use crate::{
    services::chain::ChainService, 
    state::{self, State}, 
    types::chain::BLOCK_TIME, 
    utils::{hasher::DblHasher, rng}
};

pub mod init;
pub mod post_upgrade;
pub mod pre_upgrade;

const READER_WRITER_BUFFER_SIZE: usize = 10 * 1024 * 1024; // 10MB

thread_local! {
    static HASHER: RefCell<DblHasher> = RefCell::new(DblHasher::new());
}

pub(crate) fn setup(
    s: State
) -> Result<(), String> {
    let state = s.clone();

    // init random
    rng::init(state.rng_seed());

    state::init(state);

    ic_cdk_timers::set_timer(
        Duration::from_secs(BLOCK_TIME), 
        || ic_cdk::spawn(ChainService::process())
    );    

    Ok(())
}
