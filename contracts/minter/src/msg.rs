use cosmwasm_std::{Addr, Uint128, Uint64};
use cw20::Cw20ReceiveMsg;
use cw721_base::msg::MintMsg as Cw721MintMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub base_price: Uint128,
    pub hatch_price: Uint128,
    pub random_key: i32,
    pub egg_sale_size: Uint64,
    pub allowed_cw20: Addr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}
//Change some to mandatory some to optional
// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub external_url: Option<String>,
    pub attributes: Vec<Trait>,
    pub image_data: Option<String>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

pub type Extension = Option<Metadata>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CustomMintMsg {
    pub base: Cw721MintMsg<Extension>,
    pub extension: Vec<Trait>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BaseMintMsg {
    pub base: Cw721MintMsg<Extension>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DragonMintMsg {
    pub mint: CustomMintMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EggMintMsg {
    pub mint: BaseMintMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintEggDragon {
    pub id: String,
    pub egg_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClaimMessage {
    pub recepient: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MintEgg {},
    GenesisHatch(MintEggDragon),
    DragonBirth {
        id: String,
        owner: String,
    },
    EditState {
        new_owner: String,
        base_price: Uint128,
        random_key: i32,
        hatch_price: Uint128,
        egg_sale_size: Uint64,
        allowed_cw20: Addr,
    },
    EditContracts {
        egg: String,
        dragon: String,
        recipient: String,
        multisig: String,
    },
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    Hatch { id: String, egg_id: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetState {},
    GetEggsaleOwnedCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetStateResponse {
    pub owner: String,
    pub base_price: Uint128,
    pub random_key: i32,
    pub hatch_price: Uint128,
    pub total_eggs: Uint64,
    pub egg_sale_size: Uint64,
    pub dragon_contract: String,
    pub egg_contract: String,
    pub recipient_contract: String,
    pub multisig_contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetEggsaleInfoResponse {
    pub owned_eggsale: Uint64,
    pub size: Uint64,
    pub base_price: Uint128,
}
