use crate::ContractError;
use cosmwasm_std::{Addr, Response, Uint128, Uint64};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionInfo {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub size: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: String,
    pub cosmic_contract: String,
    pub drgn_recipient: String,
    pub allowed_cw20: Addr,
    pub attune_price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Crystal {
    pub owner: String,
    pub token_id: String,
    pub kind: String,
}

impl Crystal {
    pub fn is_owner(&self, owner: String) -> Result<Response, ContractError> {
        if self.owner != owner {
            Err(ContractError::Unauthorized {})
        } else {
            Ok(Response::new())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CrystalResponse {
    pub owner: String,
    pub token_id: String,
    pub kind: String,
}

impl Into<CrystalResponse> for Crystal {
    fn into(self) -> CrystalResponse {
        CrystalResponse {
            owner: self.owner,
            token_id: self.token_id,
            kind: self.kind,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CrystalListResponse {
    pub crystals: Vec<CrystalResponse>,
}

pub const STATE: Item<State> = Item::new("state");
pub const COLLECTION_INFO: Item<CollectionInfo> = Item::new("collection_info");
pub const CRYSTAL_INFO: Map<u64, Crystal> = Map::new("crystals_list");
pub const CRYSTAL_INFO_SEQ: Item<Uint64> = Item::new("crystal_list_length");
pub const COSMIC_LENGTH: Item<Uint64> = Item::new("cosmic_id");
