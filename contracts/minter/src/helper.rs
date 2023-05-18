use crate::msg::{
    BaseMintMsg, CustomMintMsg, DragonMintMsg, EggMintMsg, Extension, Metadata, Trait,
};
use crate::ContractError;
use cw721_base::MintMsg;

pub fn generate_dragon_mint_msg(
    id: &str,
    kind: String,
    owner: String,
) -> Result<DragonMintMsg, ContractError> {
    match &kind.to_lowercase()[..] {
        "common" => {
            let attributes = vec![
                Trait {
                    display_type: None,
                    trait_type: "kind".to_string(),
                    value: "common".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "ovulation_period".to_string(),
                    value: "60".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "daily_income".to_string(),
                    value: "0.5".to_string(),
                },
            ];
            let metadata = Metadata {
                name: Option::from("common_stake_dragon".to_string()),
                description: Option::from("This is the common type of the stake dragon. It has a 60 day ovulation period and 0.5 DRGN daily income when staked".to_string()),
                image: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link/1.png".to_string()),
                external_url: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link".to_string()),
                attributes: attributes.clone(),
                image_data: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link".to_string()),
                background_color: Option::from("".to_string()),
                animation_url: Option::from("".to_string()),
                youtube_url: Option::from("".to_string())
            };
            let msg = DragonMintMsg {
                mint: CustomMintMsg {
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
        "uncommon" => {
            let attributes = vec![
                Trait {
                    display_type: None,
                    trait_type: "kind".to_string(),
                    value: "uncommon".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "ovulation_period".to_string(),
                    value: "45".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "daily_income".to_string(),
                    value: "1".to_string(),
                },
            ];
            let metadata = Metadata {
                name: Option::from("uncommon_stake_dragon".to_string()),
                description: Option::from("This is the uncommon type of the stake dragon. It has a 45 day ovulation period and 1 DRGN daily income when staked".to_string()),
                image: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link/2.png".to_string()),
                external_url: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link".to_string()),
                attributes: attributes.clone(),
                image_data: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link".to_string()),
                background_color: Option::from("".to_string()),
                animation_url: Option::from("".to_string()),
                youtube_url: Option::from("".to_string())
            };
            let msg = DragonMintMsg {
                mint: CustomMintMsg {
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
        "rare" => {
            let attributes = vec![
                Trait {
                    display_type: None,
                    trait_type: "kind".to_string(),
                    value: "rare".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "ovulation_period".to_string(),
                    value: "30".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "daily_income".to_string(),
                    value: "2".to_string(),
                },
            ];
            let metadata = Metadata {
                name: Option::from("rare_stake_dragon".to_string()),
                description: Option::from("This is the rare type of the stake dragon. It has a 30 day ovulation period and 2 DRGN daily income when staked".to_string()),
                image: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link/3.png".to_string()),
                external_url: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link".to_string()),
                attributes: attributes.clone(),
                image_data: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link".to_string()),
                background_color: Option::from("".to_string()),
                animation_url: Option::from("".to_string()),
                youtube_url: Option::from("".to_string()),
            };
            let msg = DragonMintMsg {
                mint: CustomMintMsg {
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
        "epic" => {
            let attributes = vec![
                Trait {
                    display_type: None,
                    trait_type: "kind".to_string(),
                    value: "epic".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "ovulation_period".to_string(),
                    value: "20".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "daily_income".to_string(),
                    value: "5".to_string(),
                },
            ];
            let metadata = Metadata {
                name: Option::from("epic_stake_dragon".to_string()),
                description: Option::from("This is the epic type of the stake dragon. It has a 20 day ovulation period and 5 DRGN daily income when staked".to_string()),
                image: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link/4.png".to_string()),
                external_url: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link".to_string()),
                attributes: attributes.clone(),
                image_data: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link".to_string()),
                background_color: Option::from("".to_string()),
                animation_url: Option::from("".to_string()),
                youtube_url: Option::from("".to_string()),
            };
            let msg = DragonMintMsg {
                mint: CustomMintMsg {
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
        "legendary" => {
            let attributes = vec![
                Trait {
                    display_type: None,
                    trait_type: "kind".to_string(),
                    value: "legendary".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "ovulation_period".to_string(),
                    value: "15".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "daily_income".to_string(),
                    value: "10".to_string(),
                },
            ];
            let metadata = Metadata {
                name: Option::from("common_stake_dragon".to_string()),
                description: Option::from("This is the legendary type of the stake dragon. It has a 15 day ovulation period and 10 DRGN daily income when staked".to_string()),
                image: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link/5.png".to_string()),
                external_url: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link".to_string()),
                attributes: attributes.clone(),
                image_data: Option::from("https://bafybeiacxf7hsoqkyhg6fqa6ktnxvtoni32tyuqd6w2mgpk7vosxg4y2ve.ipfs.nftstorage.link".to_string()),
                background_color: Option::from("".to_string()),
                animation_url: Option::from("".to_string()),
                youtube_url: Option::from("".to_string()),
            };

            let msg = DragonMintMsg {
                mint: CustomMintMsg {
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

pub fn generate_egg_mint_msg(id: &str, owner: String) -> Result<EggMintMsg, ContractError> {
    let a: i32 = id.parse().unwrap();
    let img_id = a % 17;

    let image_url =
        "https://bafybeihuaoctl3lhtnzg26swjud742i4kwxlrtm63n6r57353oi7sqyohy.ipfs.nftstorage.link/"
            .to_string()
            + &*(img_id + 1).to_string()
            + &*".png".to_string();

    let metadata = Metadata {
        name: Option::from("Dragon Egg NFT".to_string()),
        description: Option::from("Dragon Egg NFT".to_string()),
        image: Option::from(image_url.clone()),
        external_url: Option::from("https://bafkreiecnllo77z64a52z5ogfyrx5lhhybp5n57bscf2tgxq7qyespjnre.ipfs.nftstorage.link".to_string()),
        attributes: vec![],
        image_data: Option::from(image_url),
        background_color: Option::from("".to_string()),
        animation_url: Option::from("".to_string()),
        youtube_url: Option::from("".to_string()),
    };
    let msg = EggMintMsg {
        mint: BaseMintMsg {
            base: MintMsg {
                token_id: id.to_string(),
                owner: owner.to_string(),
                token_uri: Option::from("https://bafkreiecnllo77z64a52z5ogfyrx5lhhybp5n57bscf2tgxq7qyespjnre.ipfs.nftstorage.link".to_string()),
                extension: Extension::from(metadata),
            },
        },
    };
    Ok(msg)
}
