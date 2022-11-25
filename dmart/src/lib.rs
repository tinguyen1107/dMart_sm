use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId};

pub use crate::storage_key::*;

mod storage_key;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub nft_contract: AccountId,

    pub storage_accounts: LookupMap<AccountId, StorageAccount>,
    pub accounts: UnorderedMap<AccountId, Account>,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"Contract should be initialized before usage");
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(nft_contract: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let this = Self {
            nft_contract,
            storage_accounts: LookupMap::new(StorageKey::StorageAccount),
            accounts: UnorderedMap::new(StorageKey::Accounts),
        };
        this
    }
}
