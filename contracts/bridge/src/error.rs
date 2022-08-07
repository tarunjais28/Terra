use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
}

impl From<handler::ContractError> for ContractError {
    fn from(err: handler::ContractError) -> Self {
        match err {
            _ => ContractError::Std(StdError::generic_err(err.to_string())),
        }
    }
}

impl From<access_control::ContractError> for ContractError {
    fn from(err: access_control::ContractError) -> Self {
        match err {
            _ => ContractError::Std(StdError::generic_err(err.to_string())),
        }
    }
}
