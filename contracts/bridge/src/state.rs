use cw_storage_plus::Map;

use crate::{data::Proposal, Bytes};

pub const DEPOSIT_COUNTS: Map<u8, u64> = Map::new("deposit_counts");
pub const PROPOSALS: Map<u128, Map<Bytes, Proposal>> = Map::new("proposals");
