use crate::*;
use near_sdk::collections::UnorderedSet;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub account_info: String,

    pub bookmarks: Vec<TokenId>,
    pub following: UnorderedSet<AccountId>,
    pub followers: UnorderedSet<AccountId>,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountStats {
    pub account_info: String,

    pub num_followers: u64,
    pub num_following: u64,
    pub num_nfts: u64,
}

impl From<Account> for AccountStats {
    fn from(account: Account) -> Self {
        Self {
            account_info: account.account_info,

            num_followers: account.followers.len(),
            num_following: account.following.len(),
            num_nfts: 0,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_num_nfts(&self, account_id: &AccountId) -> u64 {
        self.tokens
            .tokens_per_owner
            .as_ref()
            .unwrap()
            .get(account_id)
            .unwrap_or(UnorderedSet::new(b"t"))
            .len()
    }

    pub fn set_account_info(&mut self, account_info: String) {
        let account_id = env::predecessor_account_id();
        let storage_update = self.new_storage_update(account_id.clone());

        let mut account = self.internal_get_account(&account_id);
        account.account_info = validate_account_data(account_info.clone());
        self.internal_set_account(&account_id, account);

        self.finalize_storage_update(storage_update);
    }

    pub fn get_account(&self, account_id: AccountId) -> Option<AccountStats> {
        let account_stats: Option<AccountStats> = self
            .internal_get_account_optional(&account_id)
            .map(|a| a.into());
        if let Some(mut account_stats_unwrapped) = account_stats {
            account_stats_unwrapped.num_nfts = self.get_num_nfts(&account_id);
            return Some(account_stats_unwrapped);
        }
        None
    }

    pub fn get_accounts(&self, from_index: u64, limit: u64) -> Vec<(AccountId, AccountStats)> {
        let account_ids = self.accounts.keys_as_vector();
        let accounts = self.accounts.values_as_vector();
        (from_index..std::cmp::min(from_index + limit, account_ids.len()))
            .map(|index| {
                let account_id = account_ids.get(index).unwrap();
                let mut account_stats: AccountStats = accounts.get(index).unwrap().into();
                account_stats.num_nfts = self.get_num_nfts(&account_id);
                (account_id, account_stats)
            })
            .collect()
    }

    pub fn get_num_accounts(&self) -> u64 {
        self.accounts.len()
    }

    pub fn get_accounts_with_ids(&self, account_ids: Vec<AccountId>) -> Vec<AccountStats> {
        account_ids
            .iter()
            .map(|id| {
                let mut account_stats: AccountStats = self.accounts.get(id).unwrap().into();
                account_stats.num_nfts = self.get_num_nfts(id);
                account_stats
            })
            .collect()
    }
}
