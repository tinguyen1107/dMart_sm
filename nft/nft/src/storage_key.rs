use crate::*;
use near_sdk::BorshStorageKey;

#[derive(BorshSerialize, BorshStorageKey)]
pub(crate) enum StorageKey {
    StorageAccount,
    Accounts,

    Orders,
    OrderByNft,
    OrderByAccount,
    OrderByAccountInner { id: String },
    Marketplace,

    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    TokenIds,
}
