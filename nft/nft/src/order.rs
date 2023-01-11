use near_sdk::serde_json::json;
use near_sdk::Promise;
use near_sdk::{env::attached_deposit, Timestamp};
use near_sdk::{ext_contract, Gas, PromiseResult};

use crate::*;
const DEFAULT_GAS_FEE: Gas = Gas(20_000_000_000_000);

pub type OrderId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Order {
    owner_id: AccountId,
    nft_id: TokenId,
    price: Balance, // Yocto near
    create_at: Timestamp,
    is_sold: bool,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OrderDto {
    order_id: String,
    owner_id: AccountId,
    nft_id: TokenId,
    price: U128, // Yocto near
    create_at: U64,
    is_sold: bool,
    metadata: TokenMetadata,
}

#[near_bindgen]
impl Contract {
    pub fn get_orders(&self, //from_index: u64, limit: u64
    ) -> Vec<OrderDto> {
        self.marketplace
            .iter()
            .map(|id| {
                let order = self.orders.get(&id).expect("Contain invalid order(s)");
                let metadata = self
                    .tokens
                    .token_metadata_by_id
                    .as_ref()
                    .unwrap()
                    .get(&order.nft_id)
                    .expect("Not exist NFT");
                OrderDto {
                    order_id: id,
                    owner_id: order.owner_id,
                    nft_id: order.nft_id,
                    price: U128::from(order.price), // Yocto near
                    create_at: U64::from(order.create_at),
                    is_sold: order.is_sold,
                    metadata,
                }
            })
            .collect()
    }

    pub fn get_account_orders(
        &self,
        account_id: AccountId, //from_index: u64, limit: u64
    ) -> Vec<OrderDto> {
        self.order_by_user
            .get(&account_id)
            .unwrap_or(UnorderedSet::new(StorageKey::OrderByAccountInner {
                id: account_id.to_string(),
            }))
            .iter()
            .map(|id| {
                let order = self.orders.get(&id).expect("Contain invalid order(s)");
                let metadata = self
                    .tokens
                    .token_metadata_by_id
                    .as_ref()
                    .unwrap()
                    .get(&order.nft_id)
                    .expect("Not exist NFT");
                OrderDto {
                    order_id: id,
                    owner_id: order.owner_id,
                    nft_id: order.nft_id,
                    price: U128::from(order.price), // Yocto near
                    create_at: U64::from(order.create_at),
                    is_sold: order.is_sold,
                    metadata,
                }
            })
            .collect()
    }

    #[payable]
    pub fn sell_nft(&mut self, nft_id: TokenId, price: U128) {
        let account_id = env::predecessor_account_id();
        let owner_id = self
            .tokens
            .owner_by_id
            .get(&nft_id)
            .expect("Can't find owner of this NFT");
        let attached_deposit = env::attached_deposit();

        assert!(
            account_id == owner_id,
            "Require owner of NFT to call sell it!"
        );
        assert!(self.orders.get(&nft_id).is_none(), "NFT are selling.");

        let block_timestamp = env::block_timestamp() / 1_000_000_000;
        let order_id = block_timestamp.to_string() + "_" + &nft_id;

        let order = Order {
            owner_id: owner_id.clone(),
            nft_id: nft_id.clone(),
            price: price.into(),
            create_at: env::block_timestamp().into(),
            is_sold: false,
        };

        self.nft_transfer(env::current_account_id(), nft_id.clone(), None, None);

        self.orders.insert(&order_id, &order);
        self.order_by_nft.insert(&nft_id, &order_id);
        let mut orders = match self.order_by_user.get(&owner_id) {
            Some(orders) => orders,
            None => UnorderedSet::<OrderId>::new(StorageKey::OrderByAccountInner {
                id: owner_id.clone().to_string(),
            }),
        };
        orders.insert(&order_id);
        self.order_by_user.insert(&owner_id, &orders);
        self.marketplace.insert(&order_id);
    }

    // #[payable]
    // pub fn cancel_sell_nft(&mut self, nft_id: TokenId, price: U128) {
    //     let account_id = env::predecessor_account_id();
    //     let owner_id = self
    //         .tokens
    //         .owner_by_id
    //         .get(&nft_id)
    //         .expect("Can't find owner of this NFT");
    //     let attached_deposit = env::attached_deposit();
    //
    //     assert!(
    //         account_id == owner_id,
    //         "Require owner of NFT to call sell it!"
    //     );
    //     assert!(self.orders.get(&nft_id).is_none(), "NFT are selling.");
    //
    //     let block_timestamp = env::block_timestamp() / 1_000_000_000;
    //     let order_id = block_timestamp.to_string() + "_" + &nft_id;
    //
    //     let order = Order {
    //         owner_id: owner_id.clone(),
    //         nft_id: nft_id.clone(),
    //         price: price.into(),
    //         create_at: env::block_timestamp().into(),
    //         is_sold: false,
    //     };
    //
    //     self.nft_transfer(env::current_account_id(), nft_id.clone(), None, None);
    //
    //     self.orders.insert(&order_id, &order);
    //     self.order_by_nft.insert(&nft_id, &order_id);
    //     let mut orders = match self.order_by_user.get(&owner_id) {
    //         Some(orders) => orders,
    //         None => UnorderedSet::<OrderId>::new(StorageKey::OrderByAccountInner {
    //             id: owner_id.clone().to_string(),
    //         }),
    //     };
    //     orders.insert(&order_id);
    //     self.order_by_user.insert(&owner_id, &orders);
    //     self.marketplace.insert(&order_id);
    // }

    #[payable]
    pub fn buy_nft(&mut self, order_id: OrderId) {
        let account_id = env::predecessor_account_id();
        let mut order = self.orders.get(&order_id).expect("Not exist Order");
        let attached_deposit = env::attached_deposit();

        assert!(!order.is_sold, "Nft is already been sold");
        assert!(account_id != order.owner_id, "Can't buy Nft of yourself");
        assert!(
            attached_deposit == order.price,
            "Need to attach exactly the price of order"
        );

        order.is_sold = false;
        self.orders.insert(&order_id, &order);

        // self.nft_transfer(account_id, order.nft_id, None, None);
        let mut storage_account = self
            .storage_accounts
            .get(&order.owner_id)
            .expect("Cant't found owner of this order");
        storage_account.balance += order.price;
        self.storage_accounts
            .insert(&order.owner_id, &storage_account);

        // Remove this order
        self.orders.remove(&order_id);
        self.order_by_nft.remove(&order.nft_id);
        let mut orders = match self.order_by_user.get(&order.owner_id) {
            Some(orders) => orders,
            None => UnorderedSet::<OrderId>::new(StorageKey::OrderByAccountInner {
                id: order.owner_id.clone().to_string(),
            }),
        };
        orders.remove(&order_id);
        self.order_by_user.insert(&order.owner_id, &orders);
        self.marketplace.remove(&order_id);

        Promise::new(env::current_account_id()).function_call(
            "nft_transfer".to_string(),
            json!({
                "receiver_id": account_id,
                "token_id": order.nft_id,
            })
            .to_string()
            .as_bytes()
            .to_vec(),
            1,
            DEFAULT_GAS_FEE,
        );
    }
}
