use cosmwasm_std::{OverflowError, StdError, Uint128};
use cw721_base::ContractError as Cw721ContractError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("721 error : {method}")]
    NftContractError { method: String },

    #[error("This CW20 token is not allowed: (current: {sent}, allowed: {need}")]
    CW20TokenNotAllowed { sent: String, need: String },

    #[error("Invalid size")]
    InvalidSize {},

    #[error("InvalidCreationFee")]
    InvalidCreationFee {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },

    #[error("Unexpected trait type: {trait_type}")]
    UnexpectedTraitType { trait_type: String },

    #[error("Kind not found")]
    KindNotFound {},

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Sent wrong amount of funds, need: {need} sent: {sent}")]
    SentWrongFundsAmount { need: Uint128, sent: Uint128 },
}

impl From<ContractError> for Cw721ContractError {
    fn from(err: ContractError) -> Cw721ContractError {
        match err {
            ContractError::Unauthorized {} => Cw721ContractError::Unauthorized {},
            _ => unreachable!("cannot connect {:?} to cw721ContractError", err),
        }
    }
}
