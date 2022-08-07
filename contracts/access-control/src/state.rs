use super::Bytes;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_RELAYERS: u128 = 200;
pub const RELAYER_ROLE: &str = "RELAYER_ROLE";
pub const DEFAULT_ADMIN_ROLE: Bytes = Vec::new();

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoleData {
    pub members: Vec<Addr>,
    pub admin_role: Bytes,
}

pub const ROLES: Map<Bytes, RoleData> = Map::new("roles");
