use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"Contract should be initialized before usage");
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let this = Self {};
        this
    }
}
