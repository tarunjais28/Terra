use crate::{
    data::Proposal,
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
use access_control::{
    contract::{has_both_admin_relayer_role, has_role},
    state::{DEFAULT_ADMIN_ROLE, RELAYER_ROLE},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "bridge";
const CONTRACT_VERSION: &str = "1.0.0";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("action", "intantiated"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {}
}

fn only_admin_or_relayer(deps: DepsMut, info: MessageInfo) -> Result<(), ContractError> {
    if !has_both_admin_relayer_role(deps, info.sender)? {
        return Err(Err(
            access_control::ContractError::SenderNeitherRelayerNorAdmin {},
        )?);
    }
    Ok(())
}

fn only_admin(deps: DepsMut, info: MessageInfo) -> Result<(), ContractError> {
    if !has_role(deps, DEFAULT_ADMIN_ROLE, info.sender)? {
        return Err(Err(
            access_control::ContractError::SenderDoesNotHaveAdminRole {},
        )?);
    }
    Ok(())
}

fn only_relayers(deps: DepsMut, info: MessageInfo) -> Result<(), ContractError> {
    if !has_role(deps, RELAYER_ROLE.into(), info.sender)? {
        return Err(Err(
            access_control::ContractError::SenderDoesNotHaveRelayerRole {},
        )?);
    }
    Ok(())
}

fn has_voted(proposal: Proposal) -> bool {
    proposal.yes_votes > 0
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}
