use cosmwasm_std::StdError;
use cw721_base::ContractError as Cw721ContractError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },

    #[error("721 error : {method}")]
    NftContractError { method: String },

    #[error("token_id already claimed")]
    Claimed {},
}

impl From<ContractError> for Cw721ContractError {
    fn from(err: ContractError) -> Cw721ContractError {
        match err {
            ContractError::Unauthorized {} => Cw721ContractError::Unauthorized {},
            ContractError::Claimed {} => Cw721ContractError::Claimed {},
            _ => unreachable!("cannot connect {:?} to cw721ContractError", err),
        }
    }
}
