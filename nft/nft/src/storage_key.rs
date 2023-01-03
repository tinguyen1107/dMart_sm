use crate::*;
use near_sdk::BorshStorageKey;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Roles,
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    TokenIds,
}
