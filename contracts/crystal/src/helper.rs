use crate::msg::{CustomMintMsg, Extension, Metadata, NftMintMsg};
use crate::ContractError;
use cw721_base::MintMsg;

pub fn generate_cosmic_mint_msg(id: String, owner: String) -> Result<NftMintMsg, ContractError> {
    let metadata = Metadata {
        name: Option::from("Cosmic Crystal NFT".to_string()),
        description: Option::from("Cosmic Crystal NFT".to_string()),
        image: Option::from("".to_string()),
        external_url: Option::from("".to_string()),
        attributes: vec![],
        image_data: Option::from("".to_string()),
        background_color: Option::from("".to_string()),
        animation_url: Option::from("".to_string()),
        youtube_url: Option::from("".to_string()),
    };
    let msg = NftMintMsg {
        mint: CustomMintMsg {
            base: MintMsg {
                token_id: id.to_string(),
                owner: owner.to_string(),
                token_uri: Option::from("".to_string()),
                extension: Extension::from(metadata),
            },
            extension: vec![],
        },
    };
    Ok(msg)
}
