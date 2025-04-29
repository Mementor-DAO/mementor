use std::cell::RefCell;
use candid::Principal;
use oc_bots_sdk_canister::env;
use serde::{Deserialize, Serialize};
use crate::types::coin::Coin;

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::default();
}

#[derive(Clone, Serialize, Deserialize)]
pub struct State {
    oc_public_key: String,
    administrator: Principal,
    meme_coin: Coin,
    rng_seed: [u8; 32],
    temp_bobs_index: usize,
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
        oc_public_key: String,
        administrator: Principal,
        meme_coin: Coin
    ) -> Self {
        Self {
            oc_public_key,
            administrator,
            meme_coin,
            temp_bobs_index: 0,
            rng_seed: env::entropy(),
        }
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

    pub fn oc_public_key(
        &self
    ) -> &str {
        &self.oc_public_key
    }
    
    pub fn set_oc_public_key(
        &mut self, 
        oc_public_key: String
    ) {
        self.oc_public_key = oc_public_key;
    }
    
    pub fn administrator(
        &self
    ) -> Principal {
        self.administrator
    }

    pub fn set_administrator(
        &mut self, 
        administrator: Principal
    ) {
        self.administrator = administrator;
    }

    pub fn temp_bobs_index(
        &mut self
    ) -> &mut usize {
        &mut self.temp_bobs_index
    }
}