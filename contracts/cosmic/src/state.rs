use crate::ContractError;
use cosmwasm_std::{Response, Uint64};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionInfo {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub description: String,
    pub size: Uint64,
    pub base_price: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: String,
    pub reward_contract_address: String,
    pub daily_income: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cosmic {
    pub owner: String,
    pub token_id: String,
    pub daily_income: String,
    pub is_staked: bool,
    pub stake_start_time: Uint64,
    pub reward_start_time: Uint64,
    pub unstaking_start_time: Uint64,
    pub unstaking_process: bool,
    pub reward_end_time: Uint64,
}

impl Cosmic {
    pub fn is_owner(&self, owner: String) -> Result<Response, ContractError> {
        if self.owner != owner {
            Err(ContractError::Unauthorized {})
        } else {
            Ok(Response::new())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CosmicResponse {
    pub owner: String,
    pub token_id: String,
    pub daily_income: String,
    pub is_staked: bool,
    pub stake_start_time: Uint64,
    pub reward_start_time: Uint64,
    pub unstaking_start_time: Uint64,
    pub unstaking_process: bool,
    pub reward_end_time: Uint64,
}

impl Into<CosmicResponse> for Cosmic {
    fn into(self) -> CosmicResponse {
        CosmicResponse {
            owner: self.owner,
            token_id: self.token_id,
            daily_income: self.daily_income,
            is_staked: self.is_staked,
            stake_start_time: self.stake_start_time,
            reward_start_time: self.reward_start_time,
            unstaking_start_time: self.unstaking_start_time,
            unstaking_process: self.unstaking_process,
            reward_end_time: self.reward_end_time,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CosmicListResponse {
    pub cosmics: Vec<CosmicResponse>,
}

pub const STATE: Item<State> = Item::new("state");
pub const COLLECTION_INFO: Item<CollectionInfo> = Item::new("collection_info");
pub const COSMIC_INFO: Map<u64, Cosmic> = Map::new("cosmics_list");
pub const COSMIC_INFO_SEQ: Item<Uint64> = Item::new("cosmic_list_length");
pub const MIN_STAKE_TIME: Item<Uint64> = Item::new("minimum_stake_time");
