use near_sdk::Timestamp;

use crate::*;

pub type OrderId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Order {
    owner_id: AccountId,
    nft_id: TokenId,
    price: u128,
    create_at: Timestamp,
    is_sold: bool,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn sell_nft(&mut self, nft_id: TokenId, price: u128) {
        let account_id = env::predecessor_account_id();
        let owner_id = self
            .tokens
            .owner_by_id
            .get(&nft_id)
            .expect("Can't find owner of this NFT");

        assert!(
            account_id == owner_id,
            "Require owner of NFT to call sell it!"
        );
        assert!(self.orders.get(&nft_id).is_none(), "NFT are selling.");

        let block_timestamp = env::block_timestamp() / 1_000_000_000;
        let order_id = block_timestamp.to_string() + "_" + &nft_id;

        let order = Order {
            owner_id,
            nft_id: nft_id.clone(),
            price,
            create_at: env::block_timestamp().into(),
            is_sold: false,
        };

        self.orders.insert(&order_id, &order);
        self.order_by_nft.insert(&nft_id, &order_id);
        self.marketplace.insert(&order_id);
    }
}
