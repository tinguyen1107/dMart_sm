use super::*;
use near_sdk::serde_json::json;
use near_sdk::{ext_contract, AccountId, Gas, PromiseResult};

const DEFAULT_GAS_FEE: Gas = Gas(20_000_000_000_000);

#[ext_contract(ext_self)]
pub trait ExtContract {
    fn on_minted_nft(&mut self, owner_id: AccountId, nft_id: String) -> bool;
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn on_minted_nft(&mut self, owner_id: AccountId, nft_id: String) -> bool {
        env::log(format!("promise_result_count = {}", env::promise_results_count()).as_bytes());
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                let owner = self
                    .accounts
                    .get(&owner_id.clone())
                    .expect("Account not found");
                let mut user_nfts = match self.user_nfts.get(&owner_id.clone()) {
                    Some(user_nfts) => user_nfts,
                    None => UnorderedSet::<NftId>::new(StorageKey::UserNFTsInner {
                        id: owner_id.clone().to_string(),
                    }),
                };
                user_nfts.insert(&nft_id);
                self.user_nfts.insert(&owner_id, &user_nfts);

                true
            }
            _ => {
                panic!("Mint chest failed")
            }
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn set_nft_contract(&mut self, nft_contract: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "You can't set new nft contract id"
        );
        self.nft_contract = nft_contract
    }

    #[payable]
    pub fn mint_nft(
        &mut self,
        account_id: Option<AccountId>,
        title: String,
        description: String,
        media: String,
        nft_type: String,
    ) -> Promise {
        let receiver_id = account_id.unwrap_or(env::predecessor_account_id());
        let amount = env::attached_deposit();

        let _ = self.accounts.get(&receiver_id);

        assert!(
            amount == 1_000_000_000_000_000_000_000_000,
            "Must attach exacts 1 NEAR to mint NFT"
        );

        let block_timestamp = env::block_timestamp() / 1_000_000_000;
        let token_id = block_timestamp.to_string() + "_" + &receiver_id.as_str();

        return Promise::new(self.nft_contract.clone())
            .function_call(
                "nft_mint".to_string(),
                json!({
                    "token_id": token_id,
                    "receiver_id": receiver_id,
                    "token_metadata": {
                        "title": title,
                        "description": description,
                        "media": media,
                        "copies": 1,
                    }
                })
                .to_string()
                .as_bytes()
                .to_vec(),
                20_000_000_000_000_000_000_000,
                DEFAULT_GAS_FEE,
            )
            .then(Self::ext(env::current_account_id()).on_minted_nft(receiver_id, token_id));
        // .on_minted_nft(
        //     receiver_id,
        //     token_id,
        //     // &env::current_account_id(),
        //     // 0,
        //     // DEFAULT_GAS_FEE,
        // );
    }
}
