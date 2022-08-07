use super::Bytes;
use super::*;
use cosmwasm_std::Addr;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Role type can be either of Relayer or Admin for declaring key of storage
    pub role_type: Bytes,
    /// Contains the addresses of various members
    pub members: Vec<Addr>,
    /// The role of Admin
    pub admin_role: Bytes,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    GrantRole { role: Bytes, account: Addr },
    RevokeRole { role: Bytes, account: Addr },
    RenounceRole { role: Bytes, account: Addr },
    SetupRole { role: Bytes, account: Addr },
    SetRoleAdmin { role: Bytes, admin_role: Bytes },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}
