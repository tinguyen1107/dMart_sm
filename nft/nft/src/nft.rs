use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DMartToken {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn mint_art_nft(&mut self, receiver_id: Option<AccountId>, metadata: TokenMetadata) {
        let caller = env::predecessor_account_id();
        let _ = self.internal_get_account(&caller);

        let storage_update = self.new_storage_update(caller.clone());

        let receiver_id = receiver_id.unwrap_or(caller);
        let block_timestamp = env::block_timestamp() / 1_000_000_000;
        let token_id = block_timestamp.to_string() + "_invite_" + &receiver_id.to_string();

        self.nft_mint(token_id, receiver_id, metadata);
        self.finalize_storage_update(storage_update);
    }

    pub(crate) fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        let token = self
            .tokens
            .internal_mint(token_id, receiver_id, Some(token_metadata));
        self.token_ids.insert(&token.token_id);
        return token;
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
                owner_id: owner_id.clone(),
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

    pub fn get_nfts(&self) -> Vec<DMartToken> {
        self.token_ids
            .iter()
            .map(|id| DMartToken {
                token_id: id.clone(),
                owner_id: self.tokens.owner_by_id.get(&id).unwrap(),
                metadata: self.token_metadata(id),
            })
            .collect()
    }
}
