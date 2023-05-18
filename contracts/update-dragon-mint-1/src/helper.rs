use crate::ContractError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DragonBirthMsg {
    pub id: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MinterMsg {
    pub dragon_birth: DragonBirthMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DragonBirthWrapper {
    pub old_minter_dragon_birth: MinterMsg,
}

pub fn generate_dragon_birth_msg(
    id: String,
    owner: String,
) -> Result<DragonBirthWrapper, ContractError> {
    let msg = DragonBirthWrapper {
        old_minter_dragon_birth: MinterMsg {
            dragon_birth: DragonBirthMsg {
                id: "0000".to_string() + id.as_str(),
                owner,
            },
        },
    };
    Ok(msg)
}
