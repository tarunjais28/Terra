pub mod contract;
mod error;
pub mod msg;
pub mod state;

use serde::{Deserialize, Serialize};

pub type Bytes = Vec<u8>;
pub type Bytes32 = [u8; 32];

#[cfg(test)]
mod tests;

pub use crate::{error::ContractError, state::*};
