#![allow(unused_variables)]
use super::*;
use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg},
    state::{RoleData, DEFAULT_ADMIN_ROLE, RELAYER_ROLE, ROLES},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Addr, DepsMut, Env, MessageInfo, Response, StdError};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "access-control";
const CONTRACT_VERSION: &str = "1.0.0";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ROLES.save(
        deps.storage,
        msg.role_type,
        &RoleData {
            members: msg.members,
            admin_role: msg.admin_role,
        },
    )?;

    Ok(Response::new().add_attribute("action", "intantiated"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::GrantRole { role, account } => Ok(grant_role(deps, info, role, account)?),
        ExecuteMsg::RevokeRole { role, account } => Ok(revoke_role(deps, info, role, account)?),
        ExecuteMsg::RenounceRole { role, account } => Ok(renounce_role(deps, info, role, account)?),
        ExecuteMsg::SetupRole { role, account } => Ok(setup_role(deps, role, account)?),
        ExecuteMsg::SetRoleAdmin { role, admin_role } => {
            Ok(set_role_admin(deps, role, admin_role)?)
        }
    }
}

/// Returns `true` if `account` has been granted both `admin` as well as `relayer` role.
pub fn has_both_admin_relayer_role(deps: DepsMut, account: Addr) -> Result<bool, ContractError> {
    Ok(ROLES
        .load(deps.storage, DEFAULT_ADMIN_ROLE)?
        .members
        .contains(&account)
        || ROLES
            .load(deps.storage, RELAYER_ROLE.into())?
            .members
            .contains(&account))
}

/// Returns `true` if `account` has been granted `role`.
pub fn has_role(deps: DepsMut, role: Bytes, account: Addr) -> Result<bool, ContractError> {
    Ok(ROLES.load(deps.storage, role)?.members.contains(&account))
}

/// Returns the number of accounts that have `role`. Can be used
/// together with {get_role_member} to enumerate all bearers of a role.
pub fn get_role_member_count(deps: DepsMut, role: Bytes) -> Result<usize, ContractError> {
    Ok(ROLES.load(deps.storage, role)?.members.len())
}

/// Returns one of the accounts that have `role`. `index` must be a
/// value between 0 and {get_role_member_count}, non-inclusive.
///
/// Role bearers are not sorted in any particular way, and their ordering may
/// change at any point.
pub fn get_role_member(deps: DepsMut, role: Bytes, index: usize) -> Result<Addr, ContractError> {
    Ok(ROLES.load(deps.storage, role)?.members[index].clone())
}

/// Returns the index of the account that have `role`.
pub fn get_role_member_index(
    deps: DepsMut,
    role: Bytes,
    account: Addr,
) -> Result<usize, ContractError> {
    let index = match ROLES
        .load(deps.storage, role)?
        .members
        .iter()
        .position(|addr| addr == &account)
    {
        Some(index) => index,
        None => return Err(ContractError::AddressNotFound {}),
    };
    Ok(index)
}

/// Returns the admin role that controls `role`. See {grant_role} and
/// {revoke_role}.
///
/// To change a role's admin, use {set_role_admin}.
pub fn get_role_admin(deps: DepsMut, role: Bytes) -> Result<Bytes, ContractError> {
    Ok(ROLES.load(deps.storage, role)?.admin_role)
}

/// Grants `role` to `account`.
///
/// If `account` had not been already granted `role`, emits a {grant_role}
/// event.
///
/// Requirements:
///
/// - the caller must have ``role``'s admin role.
pub fn grant_role(
    deps: DepsMut,
    info: MessageInfo,
    role: Bytes,
    account: Addr,
) -> Result<Response, ContractError> {
    let admin_role = ROLES.load(deps.storage, role.clone())?.admin_role;
    let has_role = ROLES
        .load(deps.storage, admin_role)?
        .members
        .contains(&info.sender);

    if !has_role {
        return Err(
            StdError::generic_err("AccessControl: sender must be an admin to grant").into(),
        );
    }

    ROLES
        .load(deps.storage, role)?
        .members
        .push(account.clone());

    Ok(Response::new()
        .add_attribute("action", "grant_role")
        .add_attribute("account", account))
}

/// Revokes `role` from `account`.
///
/// If `account` had been granted `role`, emits a {revoke_role} event.
///
/// Requirements:
///
/// - the caller must have ``role``'s admin role.
pub fn revoke_role(
    deps: DepsMut,
    info: MessageInfo,
    role: Bytes,
    account: Addr,
) -> Result<Response, ContractError> {
    let admin_role = ROLES.load(deps.storage, role.clone())?.admin_role;
    let has_role = ROLES
        .load(deps.storage, admin_role)?
        .members
        .contains(&info.sender);

    if !has_role {
        return Err(
            StdError::generic_err("AccessControl: sender must be an admin to revoke").into(),
        );
    }

    let mut data = ROLES.load(deps.storage, role.clone())?;
    data.members.retain(|addr| addr != &account);

    ROLES.save(deps.storage, role, &data)?;

    Ok(Response::new()
        .add_attribute("action", "revoke_role")
        .add_attribute("account", account))
}

/// Revokes `role` from `account`.
///
/// Roles are often managed via {grant_role} and {revoke_role}: this function's
/// purpose is to provide a mechanism for accounts to lose their privileges
/// if they are compromised (such as when a trusted device is misplaced).
///
/// If the calling account had been granted `role`, emits a {renounce_role}
/// event.
///
/// Requirements:
///
/// - the caller must be `account`.
pub fn renounce_role(
    deps: DepsMut,
    info: MessageInfo,
    role: Bytes,
    account: Addr,
) -> Result<Response, ContractError> {
    if account != info.sender {
        return Err(
            StdError::generic_err("AccessControl: can only renounce roles for self").into(),
        );
    }

    let mut data = ROLES.load(deps.storage, role.clone())?;
    data.members.retain(|addr| addr != &account);

    ROLES.save(deps.storage, role, &data)?;

    Ok(Response::new()
        .add_attribute("action", "renounce_role")
        .add_attribute("account", account))
}

/// Grants `role` to `account`.
///
/// If `account` had not been already granted `role`, emits a {setup_role}
/// event. Note that unlike {grant_role}, this function doesn't perform any
/// checks on the calling account.
pub fn setup_role(deps: DepsMut, role: Bytes, account: Addr) -> Result<Response, ContractError> {
    ROLES
        .load(deps.storage, role)?
        .members
        .push(account.clone());

    Ok(Response::new()
        .add_attribute("action", "setup_role")
        .add_attribute("account", account))
}

/// Sets `admin_role` as ``role``'s admin role.
pub fn set_role_admin(
    deps: DepsMut,
    role: Bytes,
    admin_role: Bytes,
) -> Result<Response, ContractError> {
    let members = match ROLES.load(deps.storage, role.clone()) {
        Ok(role) => role.members,
        Err(_) => Vec::new(),
    };
    let data = RoleData {
        members,
        admin_role,
    };
    ROLES.save(deps.storage, role, &data)?;

    Ok(Response::new().add_attribute("action", "set_role_admin"))
}
