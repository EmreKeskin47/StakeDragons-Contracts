use cosmwasm_std::{DivideByZeroError, OverflowError, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized {msg}")]
    Unauthorized { msg: String },

    #[error("RarityNotSupported ")]
    RarityNotSupported {},

    #[error("This CW20 token is not allowed: (current: {sent}, allowed: {need}")]
    CW20TokenNotAllowed { sent: String, need: String },

    #[error("All dragon rarities do not match")]
    DragonRarityMismatch {},

    #[error("Given dragons are staked")]
    DragonIsStaked {},

    #[error("Invalid amount: (max: {max}, min: {min}, sent: {sent}")]
    InvalidAmountSent {
        max: Uint128,
        min: Uint128,
        sent: Uint128,
    },
    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),
}
