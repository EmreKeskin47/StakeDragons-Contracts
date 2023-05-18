use cosmwasm_std::{Addr, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: String,
    pub base_price: Uint128,
    pub hatch_price: Uint128,
    pub random_key: i32,
    pub egg_sale_size: Uint64,
    pub allowed_cw20: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractAddressList {
    pub egg: String,
    pub dragon: String,
    pub recipient: String,
    pub multisig: String,
}

pub const STATE: Item<State> = Item::new("state");
pub const CONTRACTS: Item<ContractAddressList> = Item::new("contract_list");
pub const EGG_SALE_COUNT: Item<Uint64> = Item::new("eggsale_size");
pub const TOTAL_EGGS: Item<Uint64> = Item::new("total_eggs");
