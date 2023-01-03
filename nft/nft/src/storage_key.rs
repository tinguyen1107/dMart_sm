use crate::*;
use near_sdk::BorshStorageKey;

#[derive(BorshSerialize, BorshStorageKey)]
pub(crate) enum StorageKey {
    StorageAccount,
    Accounts,

    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    TokenIds,
}
