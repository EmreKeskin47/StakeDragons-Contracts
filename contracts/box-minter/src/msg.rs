use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;
use cw721_base::msg::MintMsg as Cw721MintMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub base_price: Uint128,
    pub open_price: Uint128,
    pub random_key: i32,
    pub allowed_cw20: Addr,
    pub multisig: String,
    pub juno_recipient: String,
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
pub struct BaseMintMsg {
    pub base: Cw721MintMsg<Extension>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NftMintMsg {
    pub mint: BaseMintMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExtensionMintMsg {
    pub mint: ExtMintMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExtMintMsg {
    pub base: Cw721MintMsg<Extension>,
    pub extension: Vec<Trait>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintBoxCrystal {
    pub id: String,
    pub box_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClaimMessage {
    pub recepient: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MintBox {},
    GenesisMint {},
    OpenBox(MintBoxCrystal),
    EditState {
        new_owner: String,
        base_price: Uint128,
        random_key: i32,
        open_price: Uint128,
        allowed_cw20: Addr,
    },
    EditContracts {
        dragon_box: String,
        crystal: String,
        multisig: String,
        juno_recipient: String,
    },
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    Hatch { id: String, box_id: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetState {},
    GetBoxListInfo {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetStateResponse {
    pub owner: String,
    pub base_price: Uint128,
    pub open_price: Uint128,
    pub random_key: i32,
    pub allowed_cw20: Addr,
    pub dragon_box: String,
    pub crystal: String,
    pub multisig: String,
    pub juno_recipient: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetBoxResponse {
    pub opened: Uint128,
    pub total: Uint128,
    pub base_price: Uint128,
    pub open_price: Uint128,
}
