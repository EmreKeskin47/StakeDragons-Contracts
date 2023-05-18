use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: String,
    pub base_price: Uint128,
    pub open_price: Uint128,
    pub random_key: i32,
    pub allowed_cw20: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractAddressList {
    pub dragon_box: String,
    pub crystal: String,
    pub multisig: String,
    pub juno_recipient: String,
}

pub const STATE: Item<State> = Item::new("state");
pub const CONTRACTS: Item<ContractAddressList> = Item::new("contract_list");
pub const BOX_COUNT: Item<Uint128> = Item::new("box_count");
pub const OPENED_BOX_COUNT: Item<Uint128> = Item::new("opened_box_count");
