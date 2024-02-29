#![cfg(test)]

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

use cosmwasm_std::{
    from_json, to_json_binary, Addr, CosmosMsg, DepsMut, Response, StdError, WasmMsg,
};

use cw721::{
    Approval, ApprovalResponse, CollectionInfo, Cw721ReceiveMsg, Expiration, NftInfoResponse,
    OperatorResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use cw_ownable::OwnershipError;

use crate::state::Cw721ExpirationContract;
use crate::{
    error::ContractError, msg::ExecuteMsg, msg::InstantiateMsg, msg::QueryMsg, Extension,
    MinterResponse,
};
use cw721_base::ContractError as Cw721ContractError;

const MINTER: &str = "merlin";
const CONTRACT_NAME: &str = "Magic Power";
const SYMBOL: &str = "MGK";

fn setup_contract(deps: DepsMut<'_>, expiration_days: u16) -> Cw721ExpirationContract<'static> {
    let contract = Cw721ExpirationContract::default();
    let msg = InstantiateMsg {
        expiration_days,
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: Some(String::from(MINTER)),
        withdraw_address: None,
    };
    let info = mock_info("creator", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let contract = Cw721ExpirationContract::default();

    let msg = InstantiateMsg {
        expiration_days: 1,
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: Some(String::from(MINTER)),
        withdraw_address: None,
    };
    let info = mock_info("creator", &[]);

    // we can just call .unwrap() to assert this was a success
    let res = contract
        .instantiate(deps.as_mut(), mock_env(), info, msg)
        .unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = contract.minter(deps.as_ref()).unwrap();
    assert_eq!(Some(MINTER.to_string()), res.minter);
    let info = contract.contract_info(deps.as_ref()).unwrap();
    assert_eq!(
        info,
        CollectionInfo {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
        }
    );

    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(0, count.count);

    // list the token_ids
    let tokens = contract
        .all_tokens(deps.as_ref(), mock_env(), None, None, false)
        .unwrap();
    assert_eq!(0, tokens.tokens.len());
}

#[test]
fn test_mint() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);

    let token_id = "atomize".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/atomize".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("medusa"),
        token_uri: Some(token_uri.clone()),
        extension: None,
    };

    // random cannot mint
    let random = mock_info("random", &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, mint_msg.clone())
        .unwrap_err();
    assert_eq!(
        err,
        ContractError::Cw721(Cw721ContractError::Ownership(OwnershipError::NotOwner))
    );

    // minter can mint
    let allowed = mock_info(MINTER, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), allowed, mint_msg)
        .unwrap();

    // ensure num tokens increases
    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(1, count.count);

    // unknown nft returns error
    let _ = contract
        .nft_info(deps.as_ref(), mock_env(), "unknown".to_string(), false)
        .unwrap_err();

    // this nft info is correct
    let info = contract
        .nft_info(deps.as_ref(), mock_env(), token_id.clone(), false)
        .unwrap();
    assert_eq!(
        info,
        NftInfoResponse::<Extension> {
            token_uri: Some(token_uri),
            extension: None,
        }
    );

    // owner info is correct
    let owner = contract
        .owner_of(deps.as_ref(), mock_env(), token_id.clone(), true, false)
        .unwrap();
    assert_eq!(
        owner,
        OwnerOfResponse {
            owner: String::from("medusa"),
            approvals: vec![],
        }
    );

    // assert mint timestamp is set
    let mint_timestamp = contract
        .mint_timestamps
        .load(deps.as_ref().storage, token_id.as_str())
        .unwrap();
    assert_eq!(mint_timestamp, mock_env().block.time);

    // Cannot mint same token_id again
    let mint_msg2 = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("hercules"),
        token_uri: None,
        extension: None,
    };

    let allowed = mock_info(MINTER, &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), allowed, mint_msg2)
        .unwrap_err();
    assert_eq!(err, ContractError::Cw721(Cw721ContractError::Claimed {}));

    // list the token_ids
    let tokens = contract
        .all_tokens(deps.as_ref(), mock_env(), None, None, false)
        .unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id], tokens.tokens);
}

#[test]
fn test_update_minter() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id,
        owner: String::from("medusa"),
        token_uri: Some(token_uri.clone()),
        extension: None,
    };

    // Minter can mint
    let minter_info = mock_info(MINTER, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), minter_info.clone(), mint_msg)
        .unwrap();

    // Update the owner to "random". The new owner should be able to
    // mint new tokens, the old one should not.
    contract
        .execute(
            deps.as_mut(),
            mock_env(),
            minter_info.clone(),
            ExecuteMsg::UpdateOwnership(cw_ownable::Action::TransferOwnership {
                new_owner: "random".to_string(),
                expiry: None,
            }),
        )
        .unwrap();

    // Minter does not change until ownership transfer completes.
    let minter: MinterResponse = from_json(
        contract
            .query(deps.as_ref(), mock_env(), QueryMsg::Minter {})
            .unwrap(),
    )
    .unwrap();
    assert_eq!(minter.minter, Some(MINTER.to_string()));

    // Pending ownership transfer should be discoverable via query.
    let ownership: cw_ownable::Ownership<Addr> = from_json(
        contract
            .query(deps.as_ref(), mock_env(), QueryMsg::Ownership {})
            .unwrap(),
    )
    .unwrap();

    assert_eq!(
        ownership,
        cw_ownable::Ownership::<Addr> {
            owner: Some(Addr::unchecked(MINTER)),
            pending_owner: Some(Addr::unchecked("random")),
            pending_expiry: None,
        }
    );

    // Accept the ownership transfer.
    let random_info = mock_info("random", &[]);
    contract
        .execute(
            deps.as_mut(),
            mock_env(),
            random_info.clone(),
            ExecuteMsg::UpdateOwnership(cw_ownable::Action::AcceptOwnership),
        )
        .unwrap();

    // Minter changes after ownership transfer is accepted.
    let minter: MinterResponse = from_json(
        contract
            .query(deps.as_ref(), mock_env(), QueryMsg::Minter {})
            .unwrap(),
    )
    .unwrap();
    assert_eq!(minter.minter, Some("random".to_string()));

    let mint_msg = ExecuteMsg::Mint {
        token_id: "randoms_token".to_string(),
        owner: String::from("medusa"),
        token_uri: Some(token_uri),
        extension: None,
    };

    // Old owner can not mint.
    let err: ContractError = contract
        .execute(deps.as_mut(), mock_env(), minter_info, mint_msg.clone())
        .unwrap_err();
    assert_eq!(
        err,
        ContractError::Cw721(Cw721ContractError::Ownership(OwnershipError::NotOwner))
    );

    // New owner can mint.
    let _ = contract
        .execute(deps.as_mut(), mock_env(), random_info, mint_msg)
        .unwrap();
}

#[test]
fn test_burn() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: MINTER.to_string(),
        token_uri: Some(token_uri),
        extension: None,
    };

    let burn_msg = ExecuteMsg::Burn {
        token_id: token_id.clone(),
    };

    // mint some NFT
    let mut env = mock_env();
    let minter = mock_info(MINTER, &[]);
    let _ = contract
        .execute(deps.as_mut(), env.clone(), minter.clone(), mint_msg.clone())
        .unwrap();

    // random not allowed to burn
    let random = mock_info("random", &[]);
    let err = contract
        .execute(deps.as_mut(), env.clone(), random, burn_msg.clone())
        .unwrap_err();

    assert_eq!(
        err,
        ContractError::Cw721(Cw721ContractError::Ownership(OwnershipError::NotOwner))
    );

    let _ = contract
        .execute(deps.as_mut(), env.clone(), minter.clone(), burn_msg.clone())
        .unwrap();

    // ensure num tokens decreases
    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(0, count.count);

    // trying to get nft returns error
    let _ = contract
        .nft_info(deps.as_ref(), env.clone(), "petrify".to_string(), false)
        .unwrap_err();

    // list the token_ids
    let tokens = contract
        .all_tokens(deps.as_ref(), env.clone(), None, None, false)
        .unwrap();
    assert!(tokens.tokens.is_empty());

    // assert invalid nft throws error
    // - mint again
    contract
        .execute(deps.as_mut(), env.clone(), minter.clone(), mint_msg)
        .unwrap();
    // - burn
    let mint_date = env.block.time;
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let error = contract
        .execute(deps.as_mut(), env, minter, burn_msg)
        .unwrap_err();
    assert_eq!(
        error,
        ContractError::NftExpired {
            token_id,
            mint_date,
            expiration
        }
    );
}

#[test]
fn test_transfer_nft() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);

    // Mint a token
    let token_id = "melt".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

    let owner = "owner";
    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from(owner),
        token_uri: Some(token_uri),
        extension: None,
    };

    let mut env = mock_env();
    let minter = mock_info(MINTER, &[]);
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    // random cannot transfer
    let random = mock_info("random", &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: String::from("random"),
        token_id: token_id.clone(),
    };

    let err = contract
        .execute(deps.as_mut(), env.clone(), random, transfer_msg)
        .unwrap_err();
    assert_eq!(
        err,
        ContractError::Cw721(Cw721ContractError::Ownership(OwnershipError::NotOwner))
    );

    // owner can
    let owner_info = mock_info(owner, &[]);
    let new_owner = "random";
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: String::from(new_owner),
        token_id: token_id.clone(),
    };

    let res = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            owner_info.clone(),
            transfer_msg.clone(),
        )
        .unwrap();

    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "transfer_nft")
            .add_attribute("sender", owner)
            .add_attribute("recipient", new_owner)
            .add_attribute("token_id", token_id.clone())
    );

    // assert invalid nft throws error
    let mint_date = env.block.time;
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let error = contract
        .execute(deps.as_mut(), env, owner_info, transfer_msg)
        .unwrap_err();
    assert_eq!(
        error,
        ContractError::NftExpired {
            token_id,
            mint_date,
            expiration
        }
    );
}

#[test]
fn test_send_nft() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);

    // Mint a token
    let token_id = "melt".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("venus"),
        token_uri: Some(token_uri),
        extension: None,
    };

    let mut env = mock_env();
    let minter = mock_info(MINTER, &[]);
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    let msg = to_json_binary("You now have the melting power").unwrap();
    let target = String::from("another_contract");
    let send_msg = ExecuteMsg::SendNft {
        contract: target.clone(),
        token_id: token_id.clone(),
        msg: msg.clone(),
    };

    let random = mock_info("random", &[]);
    let err = contract
        .execute(deps.as_mut(), env.clone(), random, send_msg.clone())
        .unwrap_err();
    assert_eq!(
        err,
        ContractError::Cw721(Cw721ContractError::Ownership(OwnershipError::NotOwner))
    );

    // but owner can
    let random = mock_info("venus", &[]);
    let res = contract
        .execute(deps.as_mut(), env.clone(), random.clone(), send_msg.clone())
        .unwrap();

    let payload = Cw721ReceiveMsg {
        sender: String::from("venus"),
        token_id: token_id.clone(),
        msg,
    };
    let expected = payload.into_cosmos_msg(target.clone()).unwrap();
    // ensure expected serializes as we think it should
    match &expected {
        CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, .. }) => {
            assert_eq!(contract_addr, &target)
        }
        m => panic!("Unexpected message type: {m:?}"),
    }
    // and make sure this is the request sent by the contract
    assert_eq!(
        res,
        Response::new()
            .add_message(expected)
            .add_attribute("action", "send_nft")
            .add_attribute("sender", "venus")
            .add_attribute("recipient", "another_contract")
            .add_attribute("token_id", token_id.clone())
    );

    // assert invalid nft throws error
    let mint_date = env.block.time;
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let error = contract
        .execute(deps.as_mut(), env, random, send_msg)
        .unwrap_err();
    assert_eq!(
        error,
        ContractError::NftExpired {
            token_id,
            mint_date,
            expiration
        }
    );
}

#[test]
fn test_approve_revoke() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);

    // Mint a token
    let token_id = "grow".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/grow".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("demeter"),
        token_uri: Some(token_uri),
        extension: None,
    };

    let mut env = mock_env();
    let minter = mock_info(MINTER, &[]);
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    // token owner shows in approval query
    let res = contract
        .approval(
            deps.as_ref(),
            env.clone(),
            token_id.clone(),
            String::from("demeter"),
            false,
            false,
        )
        .unwrap();
    assert_eq!(
        res,
        ApprovalResponse {
            approval: Approval {
                spender: String::from("demeter"),
                expires: Expiration::Never {}
            }
        }
    );

    // Give random transferring power
    let approve_msg = ExecuteMsg::Approve {
        spender: String::from("random"),
        token_id: token_id.clone(),
        expires: None,
    };
    let owner = mock_info("demeter", &[]);
    let res = contract
        .execute(deps.as_mut(), env.clone(), owner, approve_msg)
        .unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "approve")
            .add_attribute("sender", "demeter")
            .add_attribute("spender", "random")
            .add_attribute("token_id", token_id.clone())
    );

    // test approval query
    let res = contract
        .approval(
            deps.as_ref(),
            env.clone(),
            token_id.clone(),
            String::from("random"),
            true,
            false,
        )
        .unwrap();
    assert_eq!(
        res,
        ApprovalResponse {
            approval: Approval {
                spender: String::from("random"),
                expires: Expiration::Never {}
            }
        }
    );

    // random can now transfer
    let random = mock_info("random", &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: String::from("person"),
        token_id: token_id.clone(),
    };
    contract
        .execute(deps.as_mut(), env.clone(), random, transfer_msg)
        .unwrap();

    // Approvals are removed / cleared
    let query_msg = QueryMsg::OwnerOf {
        token_id: token_id.clone(),
        include_expired: None,
        include_invalid: None,
    };
    let res: OwnerOfResponse = from_json(
        contract
            .query(deps.as_ref(), env.clone(), query_msg.clone())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        res,
        OwnerOfResponse {
            owner: String::from("person"),
            approvals: vec![],
        }
    );

    // Approve, revoke, and check for empty, to test revoke
    let approve_msg = ExecuteMsg::Approve {
        spender: String::from("random"),
        token_id: token_id.clone(),
        expires: None,
    };
    let owner = mock_info("person", &[]);
    contract
        .execute(
            deps.as_mut(),
            env.clone(),
            owner.clone(),
            approve_msg.clone(),
        )
        .unwrap();

    let revoke_msg = ExecuteMsg::Revoke {
        spender: String::from("random"),
        token_id: token_id.clone(),
    };
    contract
        .execute(
            deps.as_mut(),
            env.clone(),
            owner.clone(),
            revoke_msg.clone(),
        )
        .unwrap();

    // Approvals are now removed / cleared
    let res: OwnerOfResponse = from_json(
        contract
            .query(deps.as_ref(), env.clone(), query_msg)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        res,
        OwnerOfResponse {
            owner: String::from("person"),
            approvals: vec![],
        }
    );

    // assert approval of invalid nft throws error
    let mint_date = env.block.time;
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let error = contract
        .execute(deps.as_mut(), env.clone(), owner.clone(), approve_msg)
        .unwrap_err();
    assert_eq!(
        error,
        ContractError::NftExpired {
            token_id: token_id.clone(),
            mint_date,
            expiration
        }
    );

    // assert revoke of invalid nft throws error
    let error = contract
        .execute(deps.as_mut(), env, owner, revoke_msg)
        .unwrap_err();
    assert_eq!(
        error,
        ContractError::NftExpired {
            token_id,
            mint_date,
            expiration
        }
    );
}

#[test]
fn test_approve_all_revoke_all() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);

    // Mint a couple tokens (from the same owner)
    let token_id1 = "grow1".to_string();
    let token_uri1 = "https://www.merriam-webster.com/dictionary/grow1".to_string();

    let token_id2 = "grow2".to_string();
    let token_uri2 = "https://www.merriam-webster.com/dictionary/grow2".to_string();

    let mint_msg1 = ExecuteMsg::Mint {
        token_id: token_id1.clone(),
        owner: String::from("demeter"),
        token_uri: Some(token_uri1),
        extension: None,
    };

    let minter = mock_info(MINTER, &[]);
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg1)
        .unwrap();

    let mint_msg2 = ExecuteMsg::Mint {
        token_id: token_id2.clone(),
        owner: String::from("demeter"),
        token_uri: Some(token_uri2),
        extension: None,
    };

    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg2)
        .unwrap();

    // paginate the token_ids
    let tokens = contract
        .all_tokens(deps.as_ref(), mock_env(), None, Some(1), false)
        .unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id1.clone()], tokens.tokens);
    let tokens = contract
        .all_tokens(
            deps.as_ref(),
            mock_env(),
            Some(token_id1.clone()),
            Some(3),
            false,
        )
        .unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id2.clone()], tokens.tokens);

    // demeter gives random full (operator) power over her tokens
    let approve_all_msg = ExecuteMsg::ApproveAll {
        operator: String::from("random"),
        expires: None,
    };
    let owner = mock_info("demeter", &[]);
    let res = contract
        .execute(deps.as_mut(), mock_env(), owner, approve_all_msg)
        .unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "approve_all")
            .add_attribute("sender", "demeter")
            .add_attribute("operator", "random")
    );

    // random can now transfer
    let random = mock_info("random", &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: String::from("person"),
        token_id: token_id1,
    };
    contract
        .execute(deps.as_mut(), mock_env(), random.clone(), transfer_msg)
        .unwrap();

    // random can now send
    let inner_msg = WasmMsg::Execute {
        contract_addr: "another_contract".into(),
        msg: to_json_binary("You now also have the growing power").unwrap(),
        funds: vec![],
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(inner_msg);

    let send_msg = ExecuteMsg::SendNft {
        contract: String::from("another_contract"),
        token_id: token_id2,
        msg: to_json_binary(&msg).unwrap(),
    };
    contract
        .execute(deps.as_mut(), mock_env(), random, send_msg)
        .unwrap();

    // Approve_all, revoke_all, and check for empty, to test revoke_all
    let approve_all_msg = ExecuteMsg::ApproveAll {
        operator: String::from("operator"),
        expires: None,
    };
    // person is now the owner of the tokens
    let owner = mock_info("person", &[]);
    contract
        .execute(deps.as_mut(), mock_env(), owner, approve_all_msg)
        .unwrap();

    // query for operator should return approval
    let res = contract
        .operator(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            String::from("operator"),
            true,
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorResponse {
            approval: Approval {
                spender: String::from("operator"),
                expires: Expiration::Never {}
            }
        }
    );

    // query for other should throw error
    let res = contract.operator(
        deps.as_ref(),
        mock_env(),
        String::from("person"),
        String::from("other"),
        true,
    );
    match res {
        Err(StdError::NotFound { kind }) => assert_eq!(kind, "Approval not found"),
        _ => panic!("Unexpected error"),
    }

    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            true,
            None,
            None,
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: String::from("operator"),
                expires: Expiration::Never {}
            }]
        }
    );

    // second approval
    let buddy_expires = Expiration::AtHeight(1234567);
    let approve_all_msg = ExecuteMsg::ApproveAll {
        operator: String::from("buddy"),
        expires: Some(buddy_expires),
    };
    let owner = mock_info("person", &[]);
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), approve_all_msg)
        .unwrap();

    // and paginate queries
    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            true,
            None,
            Some(1),
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: String::from("buddy"),
                expires: buddy_expires,
            }]
        }
    );
    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            true,
            Some(String::from("buddy")),
            Some(2),
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: String::from("operator"),
                expires: Expiration::Never {}
            }]
        }
    );

    let revoke_all_msg = ExecuteMsg::RevokeAll {
        operator: String::from("operator"),
    };
    contract
        .execute(deps.as_mut(), mock_env(), owner, revoke_all_msg)
        .unwrap();

    // query for operator should return error
    let res = contract.operator(
        deps.as_ref(),
        mock_env(),
        String::from("person"),
        String::from("operator"),
        true,
    );
    match res {
        Err(StdError::NotFound { kind }) => assert_eq!(kind, "Approval not found"),
        _ => panic!("Unexpected error"),
    }

    // Approvals are removed / cleared without affecting others
    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            false,
            None,
            None,
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: String::from("buddy"),
                expires: buddy_expires,
            }]
        }
    );

    // ensure the filter works (nothing should be here
    let mut late_env = mock_env();
    late_env.block.height = 1234568; //expired
    let res = contract
        .operators(
            deps.as_ref(),
            late_env.clone(),
            String::from("person"),
            false,
            None,
            None,
        )
        .unwrap();
    assert_eq!(0, res.operators.len());

    // query operator should also return error
    let res = contract.operator(
        deps.as_ref(),
        late_env,
        String::from("person"),
        String::from("buddy"),
        false,
    );

    match res {
        Err(StdError::NotFound { kind }) => assert_eq!(kind, "Approval not found"),
        _ => panic!("Unexpected error"),
    }
}

#[test]
fn test_tokens_by_owner() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);
    let minter = mock_info(MINTER, &[]);

    // Mint a couple tokens (from the same owner)
    let token_id1 = "grow1".to_string();
    let demeter = String::from("demeter");
    let token_id2 = "grow2".to_string();
    let ceres = String::from("ceres");
    let token_id3 = "sing".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id1.clone(),
        owner: demeter.clone(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg)
        .unwrap();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id2.clone(),
        owner: ceres.clone(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg)
        .unwrap();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id3.clone(),
        owner: demeter.clone(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    // get all tokens in order:
    let expected = vec![token_id1.clone(), token_id2.clone(), token_id3.clone()];
    let tokens = contract
        .all_tokens(deps.as_ref(), mock_env(), None, None, false)
        .unwrap();
    assert_eq!(&expected, &tokens.tokens);
    // paginate
    let tokens = contract
        .all_tokens(deps.as_ref(), mock_env(), None, Some(2), false)
        .unwrap();
    assert_eq!(&expected[..2], &tokens.tokens[..]);
    let tokens = contract
        .all_tokens(
            deps.as_ref(),
            mock_env(),
            Some(expected[1].clone()),
            None,
            false,
        )
        .unwrap();
    assert_eq!(&expected[2..], &tokens.tokens[..]);

    // get by owner
    let by_ceres = vec![token_id2];
    let by_demeter = vec![token_id1, token_id3];
    // all tokens by owner
    let tokens = contract
        .tokens(
            deps.as_ref(),
            mock_env(),
            demeter.clone(),
            None,
            None,
            false,
        )
        .unwrap();
    assert_eq!(&by_demeter, &tokens.tokens);
    let tokens = contract
        .tokens(deps.as_ref(), mock_env(), ceres, None, None, false)
        .unwrap();
    assert_eq!(&by_ceres, &tokens.tokens);

    // paginate for demeter
    let tokens = contract
        .tokens(
            deps.as_ref(),
            mock_env(),
            demeter.clone(),
            None,
            Some(1),
            false,
        )
        .unwrap();
    assert_eq!(&by_demeter[..1], &tokens.tokens[..]);
    let tokens = contract
        .tokens(
            deps.as_ref(),
            mock_env(),
            demeter,
            Some(by_demeter[0].clone()),
            Some(3),
            false,
        )
        .unwrap();
    assert_eq!(&by_demeter[1..], &tokens.tokens[..]);
}

#[test]
fn test_nft_info() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);
    let minter = mock_info(MINTER, &[]);

    let token_id = "grow1".to_string();
    let owner = String::from("ark");

    let mut env = mock_env();
    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner,
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    // assert valid nft is returned
    contract
        .nft_info(deps.as_ref(), env.clone(), token_id.clone(), false)
        .unwrap();

    // assert invalid nft throws error
    let mint_date = env.block.time;
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let error = contract
        .nft_info(deps.as_ref(), env, token_id.clone(), false)
        .unwrap_err();
    assert_eq!(
        error,
        ContractError::NftExpired {
            token_id,
            mint_date,
            expiration
        }
    );
}

#[test]
fn test_all_nft_info() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);
    let minter = mock_info(MINTER, &[]);

    let token_id = "grow1".to_string();
    let owner = String::from("ark");

    let mut env = mock_env();
    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner,
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    // assert valid nft is returned
    contract
        .all_nft_info(deps.as_ref(), env.clone(), token_id.clone(), false, false)
        .unwrap();

    // assert invalid nft throws error
    let mint_date = env.block.time;
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let error = contract
        .all_nft_info(deps.as_ref(), env, token_id.clone(), false, false)
        .unwrap_err();
    assert_eq!(
        error,
        ContractError::NftExpired {
            token_id,
            mint_date,
            expiration
        }
    );
}

#[test]
fn test_owner_of() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);
    let minter = mock_info(MINTER, &[]);

    let token_id = "grow1".to_string();
    let owner = String::from("ark");

    let mut env = mock_env();
    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner,
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    // assert valid nft is returned
    contract
        .owner_of(deps.as_ref(), env.clone(), token_id.clone(), false, false)
        .unwrap();

    // assert invalid nft throws error
    let mint_date = env.block.time;
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let error = contract
        .owner_of(deps.as_ref(), env, token_id.clone(), false, false)
        .unwrap_err();
    assert_eq!(
        error,
        ContractError::NftExpired {
            token_id,
            mint_date,
            expiration
        }
    );
}

#[test]
fn test_approval() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);
    let minter = mock_info(MINTER, &[]);

    let token_id = "grow1".to_string();
    let owner = String::from("ark");

    let mut env = mock_env();
    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: owner.clone(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    // assert valid nft is returned
    contract
        .approval(
            deps.as_ref(),
            env.clone(),
            token_id.clone(),
            owner.clone(),
            false,
            false,
        )
        .unwrap();

    // assert invalid nft throws error
    let mint_date = env.block.time;
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let error = contract
        .approval(deps.as_ref(), env, token_id.clone(), owner, false, false)
        .unwrap_err();
    assert_eq!(
        error,
        ContractError::NftExpired {
            token_id,
            mint_date,
            expiration
        }
    );
}

#[test]
fn test_approvals() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);
    let minter = mock_info(MINTER, &[]);

    let token_id = "grow1".to_string();
    let owner = String::from("ark");

    let mut env = mock_env();
    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner,
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    // assert valid nft is returned
    contract
        .approvals(deps.as_ref(), env.clone(), token_id.clone(), false, false)
        .unwrap();

    // assert invalid nft throws error
    let mint_date = env.block.time;
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let error = contract
        .approvals(deps.as_ref(), env, token_id.clone(), false, false)
        .unwrap_err();
    assert_eq!(
        error,
        ContractError::NftExpired {
            token_id,
            mint_date,
            expiration
        }
    );
}

#[test]
fn test_tokens() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);
    let minter = mock_info(MINTER, &[]);

    let token_id = "grow1".to_string();
    let owner = String::from("ark");

    let mut env = mock_env();
    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: owner.clone(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    // assert valid nft is returned
    contract
        .tokens(deps.as_ref(), env.clone(), owner.clone(), None, None, false)
        .unwrap();

    // assert invalid nft is not returned
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let tokens = contract
        .tokens(deps.as_ref(), env.clone(), owner.clone(), None, None, false)
        .unwrap();
    assert_eq!(tokens, TokensResponse { tokens: vec![] });

    // assert invalid nft is returned
    let tokens = contract
        .tokens(deps.as_ref(), env, owner, None, None, true)
        .unwrap();
    assert_eq!(
        tokens,
        TokensResponse {
            tokens: [token_id].to_vec()
        }
    );
}

#[test]
fn test_all_tokens() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), 1);
    let minter = mock_info(MINTER, &[]);

    let token_id = "grow1".to_string();
    let owner = String::from("ark");

    let mut env = mock_env();
    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: owner.clone(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    // assert valid nft is returned
    contract
        .all_tokens(deps.as_ref(), env.clone(), None, None, false)
        .unwrap();

    // assert invalid nft is not returned
    let expiration = env.block.time.plus_days(1);
    env.block.time = expiration;
    let tokens = contract
        .tokens(deps.as_ref(), env.clone(), owner, None, None, false)
        .unwrap();
    assert_eq!(tokens, TokensResponse { tokens: vec![] });

    // assert invalid nft is returned
    let tokens = contract
        .all_tokens(deps.as_ref(), env, None, None, true)
        .unwrap();
    assert_eq!(
        tokens,
        TokensResponse {
            tokens: [token_id].to_vec()
        }
    );
}
