use crate::*;

impl Contract {
    pub(crate) fn get_account_range(
        &self,
        account_ids: &Vector<AccountId>,
        from_index: u64,
        limit: u64,
    ) -> Vec<(AccountId, AccountStats)> {
        (from_index..std::cmp::min(from_index + limit, account_ids.len()))
            .filter_map(|index| {
                account_ids.get(index).and_then(|account_id| {
                    self.internal_get_account_optional(&account_id)
                        .map(|account| (account_id, account.into()))
                })
            })
            .collect()
    }

    pub(crate) fn internal_create_account(&mut self, account_id: &AccountId) -> Account {
        let hash = env::sha256(account_id.as_bytes());
        let mut following_key = vec![b'o'];
        following_key.extend_from_slice(&hash);
        let mut followers_key = vec![b'i'];
        followers_key.extend(&hash);

        let account = Account {
            account_info: "".to_string(),
            bookmarks: Vec::new(),
            following: UnorderedSet::new(following_key),
            followers: UnorderedSet::new(followers_key),
        };
        let v_account = account.into();
        assert!(
            self.accounts.insert(account_id, &v_account).is_none(),
            "Account already exists"
        );
        v_account.into()
    }

    pub(crate) fn internal_get_account_optional(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts.get(account_id).map(|a| a.into())
    }

    pub(crate) fn internal_get_account(&self, account_id: &AccountId) -> Account {
        self.internal_get_account_optional(account_id)
            .expect("Account doesn't exist")
    }

    pub(crate) fn internal_set_account(&mut self, account_id: &AccountId, account: Account) {
        self.accounts.insert(account_id, &account.into());
    }
}
