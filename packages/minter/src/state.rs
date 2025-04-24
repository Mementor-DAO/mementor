use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use crate::{types::{chain::Chain, config::Config}, utils::env};

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::default();
}

#[derive(Clone, Serialize, Deserialize)]
pub struct State {
    config: Config,
    chain: Chain,
    rng_seed: [u8; 32],
}

const STATE_ALREADY_INITIALIZED: &str = "State has already been initialized";
const STATE_NOT_INITIALIZED: &str = "State has not been initialized";

pub fn init(
    state: State
) {
    STATE.with_borrow_mut(|s| {
        if s.is_some() {
            panic!("{}", STATE_ALREADY_INITIALIZED);
        } else {
            *s = Some(state);
        }
    })
}

#[allow(unused)]
pub fn read<F, R>(
    f: F
) -> R 
    where 
        F: FnOnce(&State) -> R {
    STATE.with_borrow(|s| 
        f(s.as_ref().expect(STATE_NOT_INITIALIZED))
    )
}

#[allow(unused)]
pub fn mutate<F, R>(
    f: F
) -> R 
    where 
        F: FnOnce(&mut State) -> R {
    STATE.with_borrow_mut(|s| 
        f(s.as_mut().expect(STATE_NOT_INITIALIZED))
    )
}

pub fn take(
) -> State {
    STATE.take().expect(STATE_NOT_INITIALIZED)
}

impl State {
    pub fn new(
        config: Config,
    ) -> Self {
        Self {
            config,
            chain: Chain::new(),
            rng_seed: env::entropy(),
        }
    }

    pub fn config(
        &self,
    ) -> &Config {
        &self.config
    }

    pub fn set_config(
        &mut self,
        config: Config
    ) {
        self.config = config;
    }

    pub fn rng_seed(
        &self
    ) -> [u8; 32] {
        self.rng_seed
    }
    
    pub fn set_rng_seed(
        &mut self, 
        rng_seed: [u8; 32]
    ) {
        self.rng_seed = rng_seed;
    }

    pub fn chain(
        &self
    ) -> &Chain {
        &self.chain
    }
    
    pub fn set_chain(
        &mut self, 
        chain: Chain
    ) {
        self.chain = chain;
    }
}