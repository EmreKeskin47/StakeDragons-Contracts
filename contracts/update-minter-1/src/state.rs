use cosmwasm_std::{Addr, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: String,
    pub drgn_contract: Addr,
    pub allowed_cw20: Addr,
    pub allowed_operators: Vec<String>,
    pub random_key: i32,
    pub drgn_rac: Uint128,
    pub season: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Contracts {
    pub dragon: Addr,
    pub updated_dragon: String,
    pub egg_minter: String,
    pub drgn_recipient: String,
    pub cw20_recipient: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UpdatedStats {
    pub common_reward: Uint64,
    pub common_ovulation: Uint64,
    pub uncommon_reward: Uint64,
    pub uncommon_ovulation: Uint64,
    pub rare_reward: Uint64,
    pub rare_ovulation: Uint64,
    pub epic_reward: Uint64,
    pub epic_ovulation: Uint64,
    pub legendary_reward: Uint64,
    pub legendary_ovulation: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequiredMinMax {
    pub common_min: Uint128,
    pub common_max: Uint128,
    pub uncommon_min: Uint128,
    pub uncommon_max: Uint128,
    pub rare_min: Uint128,
    pub rare_max: Uint128,
    pub epic_min: Uint128,
    pub epic_max: Uint128,
    pub legendary_min: Uint128,
    pub legendary_max: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
pub const CONTRACTS: Item<Contracts> = Item::new("contracts");
pub const UPDATED_STATS: Item<UpdatedStats> = Item::new("stats");
pub const REQUIRED_MIN_MAX: Item<RequiredMinMax> = Item::new("min_max");
pub const UPDATED_DRAGON_COUNT: Item<Uint64> = Item::new("updated_dragon_count");
