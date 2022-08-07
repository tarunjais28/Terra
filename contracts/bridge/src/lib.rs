pub mod contract;
mod data;
mod error;
pub mod msg;
pub mod state;

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub type Bytes = Vec<u8>;

#[cfg(test)]
mod tests;

pub use crate::error::ContractError;
