use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Addr, Attribute, DepsMut,
};

use crate::{contract::*, msg::*, state::*, Bytes};

fn do_instantiate(mut deps: DepsMut, role_type: Bytes, members: Vec<Addr>, admin_role: Bytes) {
    let instantiate_msg = InstantiateMsg {
        role_type,
        members,
        admin_role,
    };
    let info = mock_info("creator", &[]);
    let env = mock_env();
    let res = instantiate(deps.branch(), env, info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn test_grant_role() {
    let mut deps = mock_dependencies(&[]);

    let addr01 = Addr::unchecked("addr01");
    do_instantiate(
        deps.as_mut(),
        DEFAULT_ADMIN_ROLE.into(),
        vec![addr01.clone()],
        DEFAULT_ADMIN_ROLE.into(),
    );

    let msg = ExecuteMsg::GrantRole {
        role: DEFAULT_ADMIN_ROLE,
        account: addr01.clone(),
    };

    let info = mock_info(addr01.as_str(), &[]);
    let env = mock_env();
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    assert_eq!(
        res.attributes,
        vec![
            Attribute {
                key: String::from("action"),
                value: String::from("grant_role")
            },
            Attribute {
                key: String::from("account"),
                value: String::from("addr01")
            },
        ]
    );

    assert!(ROLES
        .load(&deps.storage, DEFAULT_ADMIN_ROLE)
        .unwrap()
        .members
        .contains(&addr01));
}

#[test]
fn test_revoke_role() {
    let mut deps = mock_dependencies(&[]);

    let addr01 = Addr::unchecked("addr01");
    do_instantiate(
        deps.as_mut(),
        DEFAULT_ADMIN_ROLE.into(),
        vec![addr01.clone()],
        DEFAULT_ADMIN_ROLE.into(),
    );

    let msg = ExecuteMsg::RevokeRole {
        role: DEFAULT_ADMIN_ROLE,
        account: addr01.clone(),
    };
    let info = mock_info(addr01.as_str(), &[]);
    let env = mock_env();
    let res = execute(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(0, res.messages.len());
    assert_eq!(
        res.attributes,
        vec![
            Attribute {
                key: String::from("action"),
                value: String::from("revoke_role")
            },
            Attribute {
                key: String::from("account"),
                value: String::from("addr01")
            },
        ]
    );
    assert!(!ROLES
        .load(&deps.storage, DEFAULT_ADMIN_ROLE)
        .unwrap()
        .members
        .contains(&addr01));
}

#[test]
fn test_renounce_role() {
    let mut deps = mock_dependencies(&[]);

    let addr01 = Addr::unchecked("addr01");
    do_instantiate(
        deps.as_mut(),
        DEFAULT_ADMIN_ROLE.into(),
        vec![addr01.clone()],
        DEFAULT_ADMIN_ROLE.into(),
    );

    let msg = ExecuteMsg::RenounceRole {
        role: DEFAULT_ADMIN_ROLE,
        account: addr01.clone(),
    };
    let info = mock_info(addr01.as_str(), &[]);
    let env = mock_env();
    let res = execute(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(0, res.messages.len());
    assert_eq!(
        res.attributes,
        vec![
            Attribute {
                key: String::from("action"),
                value: String::from("renounce_role")
            },
            Attribute {
                key: String::from("account"),
                value: String::from("addr01")
            },
        ]
    );
    assert!(!ROLES
        .load(&deps.storage, DEFAULT_ADMIN_ROLE)
        .unwrap()
        .members
        .contains(&addr01));
}

#[test]
fn test_setup_role() {
    let mut deps = mock_dependencies(&[]);

    let addr01 = Addr::unchecked("addr01");
    do_instantiate(
        deps.as_mut(),
        DEFAULT_ADMIN_ROLE.into(),
        vec![addr01.clone()],
        DEFAULT_ADMIN_ROLE.into(),
    );

    let msg = ExecuteMsg::SetupRole {
        role: DEFAULT_ADMIN_ROLE,
        account: addr01.clone(),
    };
    let info = mock_info(addr01.as_str(), &[]);
    let env = mock_env();
    let res = execute(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(0, res.messages.len());
    assert_eq!(
        res.attributes,
        vec![
            Attribute {
                key: String::from("action"),
                value: String::from("setup_role")
            },
            Attribute {
                key: String::from("account"),
                value: String::from("addr01")
            },
        ]
    );
    assert!(ROLES
        .load(&deps.storage, DEFAULT_ADMIN_ROLE)
        .unwrap()
        .members
        .contains(&addr01));
}

#[test]
fn test_set_role_admin() {
    let mut deps = mock_dependencies(&[]);

    let addr01 = Addr::unchecked("addr01");
    do_instantiate(
        deps.as_mut(),
        DEFAULT_ADMIN_ROLE.into(),
        vec![addr01.clone()],
        DEFAULT_ADMIN_ROLE.into(),
    );

    let msg = ExecuteMsg::SetRoleAdmin {
        role: DEFAULT_ADMIN_ROLE,
        admin_role: RELAYER_ROLE.into(),
    };
    let info = mock_info(addr01.as_str(), &[]);
    let env = mock_env();
    let res = execute(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(0, res.messages.len());
    assert_eq!(
        res.attributes,
        vec![Attribute {
            key: String::from("action"),
            value: String::from("set_role_admin")
        }]
    );
    assert_eq!(
        ROLES
            .load(&deps.storage, DEFAULT_ADMIN_ROLE)
            .unwrap()
            .admin_role,
        RELAYER_ROLE.as_bytes()
    );
}
