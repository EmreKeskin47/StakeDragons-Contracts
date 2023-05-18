use crate::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;
use cw721_base::msg::MintMsg as Cw721MintMsg;

#[cw_serde]
pub struct DragonMintMsg {
    pub mint: CustomMintMsg,
}

#[cw_serde]
pub struct CustomMintMsg {
    pub base: Cw721MintMsg<Extension>,
    pub extension: Vec<Trait>,
}

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}
//Change some to mandatory some to optional
// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
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

pub fn generate_updated_dragon_mint_msg(
    id: Uint64,
    owner: String,
    kind: String,
    ovulation: String,
    daily_reward: String,
    season: String,
) -> Result<DragonMintMsg, ContractError> {
    let attributes = vec![
        Trait {
            display_type: None,
            trait_type: "kind".to_string(),
            value: kind + &*"+".to_string() + &*season,
        },
        Trait {
            display_type: None,
            trait_type: "ovulation_period".to_string(),
            value: ovulation.to_string(),
        },
        Trait {
            display_type: None,
            trait_type: "daily_income".to_string(),
            value: daily_reward.to_string(),
        },
    ];
    let metadata = Metadata {
        name: Option::from("Updated stake dragon".to_string()),
        description: Option::from("Updated stake dragon".to_string()),
        image: Option::from("".to_string()),
        external_url: Option::from("".to_string()),
        attributes: attributes.clone(),
        image_data: Option::from("".to_string()),
        background_color: Option::from("".to_string()),
        animation_url: Option::from("".to_string()),
        youtube_url: Option::from("".to_string()),
    };
    let msg = DragonMintMsg {
        mint: CustomMintMsg {
            base: Cw721MintMsg {
                token_id: id.to_string(),
                owner: owner.to_string(),
                token_uri: None,
                extension: Extension::from(metadata),
            },
            extension: attributes,
        },
    };
    Ok(msg)
}
