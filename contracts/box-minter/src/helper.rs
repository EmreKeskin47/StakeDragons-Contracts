use crate::msg::{
    BaseMintMsg, ExtMintMsg, Extension, ExtensionMintMsg, Metadata, NftMintMsg, Trait,
};
use crate::ContractError;
use cw721_base::MintMsg;

pub fn generate_crystal_mint_msg(
    id: &str,
    kind: String,
    owner: String,
) -> Result<ExtensionMintMsg, ContractError> {
    match &kind.to_lowercase()[..] {
        "fire" => {
            let attributes = vec![Trait {
                display_type: None,
                trait_type: "kind".to_string(),
                value: "fire".to_string(),
            }];
            let metadata = Metadata {
                name: Option::from("fire_crystal".to_string()),
                description: Option::from("".to_string()),
                image: Option::from("".to_string()),
                external_url: Option::from("".to_string()),
                attributes: attributes.clone(),
                image_data: Option::from("".to_string()),
                background_color: Option::from("".to_string()),
                animation_url: Option::from("".to_string()),
                youtube_url: Option::from("".to_string()),
            };
            let msg = ExtensionMintMsg {
                mint: ExtMintMsg {
                    base: MintMsg {
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
        "ice" => {
            let attributes = vec![Trait {
                display_type: None,
                trait_type: "kind".to_string(),
                value: "ice".to_string(),
            }];
            let metadata = Metadata {
                name: Option::from("ice_crystal".to_string()),
                description: Option::from("".to_string()),
                image: Option::from("".to_string()),
                external_url: Option::from("".to_string()),
                attributes: attributes.clone(),
                image_data: Option::from("".to_string()),
                background_color: Option::from("".to_string()),
                animation_url: Option::from("".to_string()),
                youtube_url: Option::from("".to_string()),
            };
            let msg = ExtensionMintMsg {
                mint: ExtMintMsg {
                    base: MintMsg {
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
        "storm" => {
            let attributes = vec![Trait {
                display_type: None,
                trait_type: "kind".to_string(),
                value: "storm".to_string(),
            }];
            let metadata = Metadata {
                name: Option::from("storm_crystal".to_string()),
                description: Option::from("".to_string()),
                image: Option::from("".to_string()),
                external_url: Option::from("".to_string()),
                attributes: attributes.clone(),
                image_data: Option::from("".to_string()),
                background_color: Option::from("".to_string()),
                animation_url: Option::from("".to_string()),
                youtube_url: Option::from("".to_string()),
            };
            let msg = ExtensionMintMsg {
                mint: ExtMintMsg {
                    base: MintMsg {
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
        "divine" => {
            let attributes = vec![Trait {
                display_type: None,
                trait_type: "kind".to_string(),
                value: "divine".to_string(),
            }];
            let metadata = Metadata {
                name: Option::from("divine_crystal".to_string()),
                description: Option::from("".to_string()),
                image: Option::from("".to_string()),
                external_url: Option::from("".to_string()),
                attributes: attributes.clone(),
                image_data: Option::from("".to_string()),
                background_color: Option::from("".to_string()),
                animation_url: Option::from("".to_string()),
                youtube_url: Option::from("".to_string()),
            };
            let msg = ExtensionMintMsg {
                mint: ExtMintMsg {
                    base: MintMsg {
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
        "udin" => {
            let attributes = vec![Trait {
                display_type: None,
                trait_type: "kind".to_string(),
                value: "udin".to_string(),
            }];
            let metadata = Metadata {
                name: Option::from("udin_crystal".to_string()),
                description: Option::from("".to_string()),
                image: Option::from("".to_string()),
                external_url: Option::from("".to_string()),
                attributes: attributes.clone(),
                image_data: Option::from("".to_string()),
                background_color: Option::from("".to_string()),
                animation_url: Option::from("".to_string()),
                youtube_url: Option::from("".to_string()),
            };

            let msg = ExtensionMintMsg {
                mint: ExtMintMsg {
                    base: MintMsg {
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
        _ => Err(ContractError::MintError {}),
    }
}

pub fn generate_box_mint_msg(id: &str, owner: String) -> Result<NftMintMsg, ContractError> {
    let metadata = Metadata {
        name: Option::from("Dragon Box NFT".to_string()),
        description: Option::from("Dragon Box NFT".to_string()),
        image: Option::from("".to_string()),
        external_url: Option::from("".to_string()),
        attributes: vec![],
        image_data: Option::from("".to_string()),
        background_color: Option::from("".to_string()),
        animation_url: Option::from("".to_string()),
        youtube_url: Option::from("".to_string()),
    };
    let msg = NftMintMsg {
        mint: BaseMintMsg {
            base: MintMsg {
                token_id: id.to_string(),
                owner: owner.to_string(),
                token_uri: Option::from("".to_string()),
                extension: Extension::from(metadata),
            },
        },
    };
    Ok(msg)
}
