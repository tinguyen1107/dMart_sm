use crate::*;
use near_sdk::collections::UnorderedSet;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub avatar: String,
    pub thumbnail: String,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountStats {
    pub avatar: String,
    pub thumbnail: String,
}

impl From<Account> for AccountStats {
    fn from(account: Account) -> Self {
        Self {
            avatar: account.avatar,
            thumbnail: account.thumbnail,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn set_avatar(&mut self, avatar: String) {
        let account_id = env::predecessor_account_id();

        let storage_update = self.new_storage_update(account_id.clone());
        let mut account = self.internal_get_account(&account_id);
        account.avatar = avatar;
        self.internal_set_account(&account_id, account);
        self.finalize_storage_update(storage_update);
    }

    pub fn set_thumbnail(&mut self, thumbnail: String) {
        let account_id = env::predecessor_account_id();

        let storage_update = self.new_storage_update(account_id.clone());
        let mut account = self.internal_get_account(&account_id);
        account.thumbnail = thumbnail;
        self.internal_set_account(&account_id, account);
        self.finalize_storage_update(storage_update);
    }

    pub fn get_account(&self, account_id: AccountId) -> Option<AccountStats> {
        self.internal_get_account_optional(&account_id)
            .map(|a| a.into())
    }

    pub fn get_accounts(&self, from_index: u64, limit: u64) -> Vec<(AccountId, AccountStats)> {
        let account_ids = self.accounts.keys_as_vector();
        let accounts = self.accounts.values_as_vector();
        (from_index..std::cmp::min(from_index + limit, account_ids.len()))
            .map(|index| {
                let account_id = account_ids.get(index).unwrap();
                let account: Account = accounts.get(index).unwrap().into();
                (account_id, account.into())
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
                let account: Account = self.accounts.get(id).unwrap().into();
                account.into()
            })
            .collect()
    }
}
