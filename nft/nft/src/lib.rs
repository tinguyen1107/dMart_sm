use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PromiseOrValue, StorageUsage,
};

pub use crate::account::*;
pub use crate::constant::*;
pub use crate::internal_account::*;
pub use crate::nft::*;
pub use crate::storage::*;
pub use crate::storage_key::*;
pub use crate::validator::*;

mod account;
mod constant;
mod internal_account;
mod nft;
mod storage;
mod storage_key;
mod validator;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // roles: LookupMap<AccountId, String>,
    pub tokens: NonFungibleToken,
    pub metadata: LazyOption<NFTContractMetadata>,
    pub token_ids: UnorderedSet<TokenId>,

    pub storage_accounts: LookupMap<AccountId, StorageAccount>,
    pub storage_account_in_bytes: StorageUsage,

    pub accounts: UnorderedMap<AccountId, Account>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DMartToken {
    token_id: TokenId,
    metadata: TokenMetadata,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "DMart".to_string(),
                symbol: "DMART".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            // roles: LookupMap::new(StorageKey::Roles),
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            token_ids: UnorderedSet::new(StorageKey::TokenIds),

            storage_accounts: LookupMap::new(StorageKey::StorageAccount),
            storage_account_in_bytes: 0,

            accounts: UnorderedMap::new(StorageKey::Accounts),
        }
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
