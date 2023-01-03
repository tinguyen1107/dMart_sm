use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        // let role = self
        //     .roles
        //     .get(&env::predecessor_account_id())
        //     .expect("Account doesn't have any special role to mint NFT");
        // assert!(
        //     role == "contract_mint",
        //     "This account doesn't have permission to mint contract"
        // );
        let token = self
            .tokens
            .internal_mint(token_id, receiver_id, Some(token_metadata));
        self.token_ids.insert(&token.token_id);
        return token;
    }

    // #[payable]
    // pub fn set_role(&mut self, account_id: AccountId) -> bool {
    //     assert_eq!(
    //         env::predecessor_account_id(),
    //         self.tokens.owner_id,
    //         "You don't have permission to set role for other people"
    //     );
    //     self.roles
    //         .insert(&account_id, &"contract_mint".to_string())
    //         .is_none()
    // }

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
