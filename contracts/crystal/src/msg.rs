use cosmwasm_std::{Addr, Binary, Uint128, Uint64};
use cw20::Cw20ReceiveMsg;
use cw721_base::msg::InstantiateMsg as Cw721InstantiateMsg;
use cw721_base::msg::MintMsg as Cw721MintMsg;
use cw721_base::msg::QueryMsg as Cw721QueryMsg;
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub base: Cw721InstantiateMsg,
    pub size: Uint64,
    pub cosmic_contract: String,
    pub drgn_recipient: String,
    pub allowed_cw20: Addr,
    pub attune_price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}
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
pub enum ReceiveMsg {
    GenerateCosmic {
        fire_id: Uint64,
        ice_id: Uint64,
        storm_id: Uint64,
        divine_id: Uint64,
        udin_id: Uint64,
        owner: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateOwner {
        new_owner: String,
    },
    TransferNft {
        recipient: String,
        token_id: Uint64,
    },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: String,
        token_id: String,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: String,
    },
    Burn {
        token_id: String,
    },
    Mint(CustomMintMsg),
    GenerateCosmic {
        fire_id: Uint64,
        ice_id: Uint64,
        storm_id: Uint64,
        divine_id: Uint64,
        udin_id: Uint64,
    },
    UpdateState {
        cosmic_contract: String,
        drgn_recipient: String,
        allowed_cw20: Addr,
        attune_price: Uint128,
    },
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    Minter {},
    CollectionInfo {},
    CrystalInfoList {},
    CrystalInfo {
        id: Uint64,
    },
    QueryUserCrystal {
        owner: String,
    },
    RangeCrystals {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    RangeUserCrystals {
        start_after: Option<u64>,
        limit: Option<u32>,
        owner: String,
    },
    GetState {},
}

impl From<QueryMsg> for Cw721QueryMsg {
    fn from(msg: QueryMsg) -> Cw721QueryMsg {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721QueryMsg::AllTokens { start_after, limit }
            }
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            _ => unreachable!("cannot convert {:?} to Cw721QueryMsg", msg),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionInfoResponse {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub size: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub owner: String,
    pub cosmic_contract: String,
    pub drgn_recipient: String,
    pub allowed_cw20: Addr,
    pub attune_price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NftMintMsg {
    pub mint: CustomMintMsg,
}
