use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Sender is not relayer or admin")]
    SenderNeitherRelayerNorAdmin {},

    #[error("Sender doesn't have admin role")]
    SenderDoesNotHaveAdminRole {},

    #[error("Sender doesn't have relayer role")]
    SenderDoesNotHaveRelayerRole {},

    #[error("Address not found")]
    AddressNotFound {},
}
