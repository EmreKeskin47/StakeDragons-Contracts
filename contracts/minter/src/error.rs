use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized {msg}")]
    Unauthorized { msg: String },

    #[error("Generate metadata failed")]
    MintError {},

    #[error("Egg sale limit reached")]
    EggSaleLimit {},

    #[error("Insufficient Fee {0} < {1}")]
    InsufficientFee(u128, u128),

    #[error("Native token not in allowed list: {denom}")]
    NativeDenomNotAllowed { denom: String },

    #[error("No support for CW20 and native tokens simultaneously")]
    InvalidTokenType {},

    #[error("The marketplace does not support CW20 tokens")]
    CW20TokenNotSupported {},

    #[error("This CW20 token is not allowed: (current: {sent}, allowed: {need}")]
    CW20TokenNotAllowed { sent: String, need: String },

    #[error("Send single native token type")]
    SendSingleNativeToken {},

    #[error("Sent wrong amount of funds, need: {need} sent: {sent}")]
    SentWrongFundsAmount { need: Uint128, sent: Uint128 },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
