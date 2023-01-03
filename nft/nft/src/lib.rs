use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, PromiseOrValue};

pub use crate::constant::*;
pub use crate::storage_key::*;

mod constant;
mod storage_key;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    roles: LookupMap<AccountId, String>,
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    token_ids: UnorderedSet<TokenId>,
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
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            roles: LookupMap::new(StorageKey::Roles),
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            token_ids: UnorderedSet::new(StorageKey::TokenIds),
        }
    }

    /// Mint a new token with ID=`token_id` belonging to `receiver_id`.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. `self.tokens.mint` will also require it to be Some, since
    /// `StorageKey::TokenMetadata` was provided at initialization.
    ///
    /// `self.tokens.mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
    /// initialization call to `new`.
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        let role = self
            .roles
            .get(&env::predecessor_account_id())
            .expect("Account doesn't have any special role to mint NFT");
        assert!(
            role == "contract_mint",
            "This account doesn't have permission to mint contract"
        );
        let token = self
            .tokens
            .internal_mint(token_id, receiver_id, Some(token_metadata));
        self.token_ids.insert(&token.token_id);
        return token;
    }

    #[payable]
    pub fn set_role(&mut self, account_id: AccountId) -> bool {
        assert_eq!(
            env::predecessor_account_id(),
            self.tokens.owner_id,
            "You don't have permission to set role for other people"
        );
        self.roles
            .insert(&account_id, &"contract_mint".to_string())
            .is_none()
    }

    pub fn tokens_of_owner(&self, owner_id: AccountId) -> Vec<TokenId> {
        self.tokens
            .tokens_per_owner
            .as_ref()
            .unwrap()
            .get(&owner_id)
            .unwrap_or(UnorderedSet::new(b"x"))
            .to_vec()
    }

    pub fn tokens_metadata_of_owner(&self, owner_id: AccountId) -> Vec<DMartToken> {
        self.tokens
            .tokens_per_owner
            .as_ref()
            .unwrap()
            .get(&owner_id)
            .unwrap_or(UnorderedSet::new(b"t"))
            .iter()
            .map(|v| DMartToken {
                token_id: v.clone(),
                metadata: self
                    .tokens
                    .token_metadata_by_id
                    .as_ref()
                    .unwrap()
                    .get(&v)
                    .unwrap(),
            })
            .collect()
    }

    pub fn token_metadata(&self, token_id: TokenId) -> TokenMetadata {
        self.tokens
            .token_metadata_by_id
            .as_ref()
            .unwrap()
            .get(&token_id.to_string())
            .unwrap()
    }

    pub fn get_nfts(&self) -> Vec<TokenMetadata> {
        self.token_ids
            .iter()
            .map(|id| self.token_metadata(id))
            .collect()
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
