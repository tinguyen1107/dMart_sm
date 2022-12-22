use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::collections::Vector;
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, setup_alloc, AccountId, Balance, BlockHeight, BorshStorageKey, Promise,
    StorageUsage,
};

pub use crate::account::*;
pub use crate::internal_account::*;
pub use crate::nft::*;
pub use crate::storage::*;
pub use crate::storage_key::*;

mod account;
mod internal_account;
mod nft;
mod storage;
mod storage_key;

type NftId = String;
type Auction = String;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub nft_contract: AccountId,

    pub storage_accounts: LookupMap<AccountId, StorageAccount>,
    pub storage_account_in_bytes: StorageUsage,

    pub accounts: UnorderedMap<AccountId, Account>,
    pub user_nfts: LookupMap<AccountId, UnorderedSet<NftId>>,

    pub marketplace: UnorderedSet<NftId>,
    pub auction: UnorderedSet<Auction>,
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
            storage_account_in_bytes: 0,
            accounts: UnorderedMap::new(StorageKey::Accounts),
            user_nfts: LookupMap::new(StorageKey::UserNFT),

            marketplace: UnorderedSet::new(StorageKey::Marketplace),
            auction: UnorderedSet::new(StorageKey::Auction),
        };
        this
    }
}
