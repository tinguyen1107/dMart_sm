use crate::*;
use near_sdk::BorshStorageKey;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    StorageAccount,
    Accounts,
    UserNFT,
    UserNFTsInner { id: String },

    Marketplace,
    Auction,
}
