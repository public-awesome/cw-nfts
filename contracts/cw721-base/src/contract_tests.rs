#![cfg(test)]

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

use cosmwasm_std::{
    from_json, to_json_binary, Addr, Coin, CosmosMsg, DepsMut, Empty, Response, StdError, WasmMsg,
};

use cw721::{
    Approval, ApprovalResponse, ContractInfoResponse, Cw721Query, Cw721ReceiveMsg, Expiration,
    NftInfoResponse, OperatorResponse, OperatorsResponse, OwnerOfResponse,
};
use cw_ownable::OwnershipError;

use crate::{
    ContractError, Cw721Contract, ExecuteMsg, Extension, InstantiateMsg, MinterResponse, QueryMsg,
};

const MINTER: &str = "merlin";
const CONTRACT_NAME: &str = "Magic Power";
const SYMBOL: &str = "MGK";

fn setup_contract(
    deps: DepsMut<'_>,
    creator: Addr,
    minter: Addr,
) -> Cw721Contract<'static, Extension, Empty, Empty, Empty> {
    let contract = Cw721Contract::default();

    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: Some(minter.to_string()),
        withdraw_address: None,
    };
    let info = mock_info(creator.as_ref(), &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let contract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
    let minter = deps.api.addr_make(MINTER);
    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: Some(minter.to_string()),
        withdraw_address: Some(minter.to_string()),
    };
    let info = mock_info(deps.api.addr_make("creator").as_ref(), &[]);

    // we can just call .unwrap() to assert this was a success
    let res = contract
        .instantiate(deps.as_mut(), mock_env(), info, msg)
        .unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = contract.minter(deps.as_ref()).unwrap();
    assert_eq!(Some(minter.to_string()), res.minter);
    let info = contract.contract_info(deps.as_ref()).unwrap();
    assert_eq!(
        info,
        ContractInfoResponse {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
        }
    );

    let withdraw_address = contract
        .withdraw_address
        .may_load(deps.as_ref().storage)
        .unwrap();
    assert_eq!(Some(minter.to_string()), withdraw_address);

    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(0, count.count);

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(0, tokens.tokens.len());
}

#[test]
fn minting() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let contract = setup_contract(deps.as_mut(), creator, minter.clone());

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: deps.api.addr_make("medusa").to_string(),
        token_uri: Some(token_uri.clone()),
        extension: None,
    };

    // random cannot mint
    let random = mock_info(deps.api.addr_make("random").as_ref(), &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, mint_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

    // minter can mint
    let allowed = mock_info(minter.as_ref(), &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), allowed, mint_msg)
        .unwrap();

    // ensure num tokens increases
    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(1, count.count);

    // unknown nft returns error
    let _ = contract
        .nft_info(deps.as_ref(), "unknown".to_string())
        .unwrap_err();

    // this nft info is correct
    let info = contract.nft_info(deps.as_ref(), token_id.clone()).unwrap();
    assert_eq!(
        info,
        NftInfoResponse::<Extension> {
            token_uri: Some(token_uri),
            extension: None,
        }
    );

    // owner info is correct
    let owner = contract
        .owner_of(deps.as_ref(), mock_env(), token_id.clone(), true)
        .unwrap();
    assert_eq!(
        owner,
        OwnerOfResponse {
            owner: deps.api.addr_make("medusa").to_string(),
            approvals: vec![],
        }
    );

    // Cannot mint same token_id again
    let mint_msg2 = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: deps.api.addr_make("hercules").to_string(),
        token_uri: None,
        extension: None,
    };

    let allowed = mock_info(minter.as_ref(), &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), allowed, mint_msg2)
        .unwrap_err();
    assert_eq!(err, ContractError::Claimed {});

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id], tokens.tokens);
}

#[test]
fn test_update_minter() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let random = deps.api.addr_make("random");
    let contract = setup_contract(deps.as_mut(), creator, minter.clone());

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id,
        owner: deps.api.addr_make("medusa").to_string(),
        token_uri: Some(token_uri.clone()),
        extension: None,
    };

    // Minter can mint
    let minter_info = mock_info(minter.as_ref(), &[]);
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
                new_owner: random.to_string(),
                expiry: None,
            }),
        )
        .unwrap();

    // Minter does not change until ownership transfer completes.
    let minter_response: MinterResponse = from_json(
        contract
            .query(deps.as_ref(), mock_env(), QueryMsg::Minter {})
            .unwrap(),
    )
    .unwrap();
    assert_eq!(minter_response.minter, Some(minter.to_string()));

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
            owner: Some(minter),
            pending_owner: Some(random.clone()),
            pending_expiry: None,
        }
    );

    // Accept the ownership transfer.
    let random_info = mock_info(random.as_ref(), &[]);
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
    assert_eq!(minter.minter, Some(random.to_string()));

    let mint_msg = ExecuteMsg::Mint {
        token_id: "randoms_token".to_string(),
        owner: deps.api.addr_make("medusa").to_string(),
        token_uri: Some(token_uri),
        extension: None,
    };

    // Old owner can not mint.
    let err: ContractError = contract
        .execute(deps.as_mut(), mock_env(), minter_info, mint_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

    // New owner can mint.
    let _ = contract
        .execute(deps.as_mut(), mock_env(), random_info, mint_msg)
        .unwrap();
}

#[test]
fn burning() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let contract = setup_contract(deps.as_mut(), creator, minter.clone());

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: minter.to_string(),
        token_uri: Some(token_uri),
        extension: None,
    };

    let burn_msg = ExecuteMsg::Burn { token_id };

    // mint some NFT
    let allowed = mock_info(minter.as_ref(), &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), allowed.clone(), mint_msg)
        .unwrap();

    // random not allowed to burn
    let random = mock_info(deps.api.addr_make("random").as_ref(), &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, burn_msg.clone())
        .unwrap_err();

    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

    let _ = contract
        .execute(deps.as_mut(), mock_env(), allowed, burn_msg)
        .unwrap();

    // ensure num tokens decreases
    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(0, count.count);

    // trying to get nft returns error
    let _ = contract
        .nft_info(deps.as_ref(), "petrify".to_string())
        .unwrap_err();

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert!(tokens.tokens.is_empty());
}

#[test]
fn transferring_nft() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let contract = setup_contract(deps.as_mut(), creator, minter.clone());

    // Mint a token
    let token_id = "melt".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: deps.api.addr_make("venus").to_string(),
        token_uri: Some(token_uri),
        extension: None,
    };

    let minter = mock_info(minter.as_ref(), &[]);
    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    // random cannot transfer
    let random = mock_info("random", &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: deps.api.addr_make("random").to_string(),
        token_id: token_id.clone(),
    };

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, transfer_msg)
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

    // owner can
    let random = mock_info(deps.api.addr_make("venus").as_ref(), &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: deps.api.addr_make("random").to_string(),
        token_id: token_id.clone(),
    };

    let res = contract
        .execute(deps.as_mut(), mock_env(), random, transfer_msg)
        .unwrap();

    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "transfer_nft")
            .add_attribute("sender", deps.api.addr_make("venus").as_ref())
            .add_attribute("recipient", deps.api.addr_make("random").as_ref())
            .add_attribute("token_id", token_id)
    );
}

#[test]
fn sending_nft() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let contract = setup_contract(deps.as_mut(), creator, minter.clone());

    // Mint a token
    let token_id = "melt".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: deps.api.addr_make("venus").to_string(),
        token_uri: Some(token_uri),
        extension: None,
    };

    let minter = mock_info(minter.as_ref(), &[]);
    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    let msg = to_json_binary("You now have the melting power").unwrap();
    let target = deps.api.addr_make("another_contract");
    let send_msg = ExecuteMsg::SendNft {
        contract: target.to_string(),
        token_id: token_id.clone(),
        msg: msg.clone(),
    };

    let random = mock_info(deps.api.addr_make("random").as_ref(), &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, send_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

    // but owner can
    let random = mock_info(deps.api.addr_make("venus").as_ref(), &[]);
    let res = contract
        .execute(deps.as_mut(), mock_env(), random, send_msg)
        .unwrap();

    let payload = Cw721ReceiveMsg {
        sender: deps.api.addr_make("venus").to_string(),
        token_id: token_id.clone(),
        msg,
    };
    let expected = payload.into_cosmos_msg(target.clone()).unwrap();
    // ensure expected serializes as we think it should
    match &expected {
        CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, .. }) => {
            assert_eq!(contract_addr, target.as_ref())
        }
        _m => panic!("Unexpected message type: {_m:?}"),
    }
    // and make sure this is the request sent by the contract
    assert_eq!(
        res,
        Response::new()
            .add_message(expected)
            .add_attribute("action", "send_nft")
            .add_attribute("sender", deps.api.addr_make("venus").as_ref())
            .add_attribute("recipient", deps.api.addr_make("another_contract").as_ref())
            .add_attribute("token_id", token_id)
    );
}

#[test]
fn approving_revoking() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let contract = setup_contract(deps.as_mut(), creator, minter.clone());

    // Mint a token
    let token_id = "grow".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/grow".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: deps.api.addr_make("demeter").to_string(),
        token_uri: Some(token_uri),
        extension: None,
    };

    let minter = mock_info(minter.as_ref(), &[]);
    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    // token owner shows in approval query
    let res = contract
        .approval(
            deps.as_ref(),
            mock_env(),
            token_id.clone(),
            deps.api.addr_make("demeter").to_string(),
            false,
        )
        .unwrap();
    assert_eq!(
        res,
        ApprovalResponse {
            approval: Approval {
                spender: deps.api.addr_make("demeter").to_string(),
                expires: Expiration::Never {}
            }
        }
    );

    // Give random transferring power
    let approve_msg = ExecuteMsg::Approve {
        spender: deps.api.addr_make("random").to_string(),
        token_id: token_id.clone(),
        expires: None,
    };
    let owner = mock_info(deps.api.addr_make("demeter").as_ref(), &[]);
    let res = contract
        .execute(deps.as_mut(), mock_env(), owner, approve_msg)
        .unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "approve")
            .add_attribute("sender", deps.api.addr_make("demeter").as_ref())
            .add_attribute("spender", deps.api.addr_make("random").as_ref())
            .add_attribute("token_id", token_id.clone())
    );

    // test approval query
    let res = contract
        .approval(
            deps.as_ref(),
            mock_env(),
            token_id.clone(),
            deps.api.addr_make("random").to_string(),
            true,
        )
        .unwrap();
    assert_eq!(
        res,
        ApprovalResponse {
            approval: Approval {
                spender: deps.api.addr_make("random").to_string(),
                expires: Expiration::Never {}
            }
        }
    );

    // random can now transfer
    let random = mock_info(deps.api.addr_make("random").as_ref(), &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: deps.api.addr_make("person").to_string(),
        token_id: token_id.clone(),
    };
    contract
        .execute(deps.as_mut(), mock_env(), random, transfer_msg)
        .unwrap();

    // Approvals are removed / cleared
    let query_msg = QueryMsg::OwnerOf {
        token_id: token_id.clone(),
        include_expired: None,
    };
    let res: OwnerOfResponse = from_json(
        contract
            .query(deps.as_ref(), mock_env(), query_msg.clone())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        res,
        OwnerOfResponse {
            owner: deps.api.addr_make("person").to_string(),
            approvals: vec![],
        }
    );

    // Approve, revoke, and check for empty, to test revoke
    let approve_msg = ExecuteMsg::Approve {
        spender: deps.api.addr_make("random").to_string(),
        token_id: token_id.clone(),
        expires: None,
    };
    let owner = mock_info(deps.api.addr_make("person").as_ref(), &[]);
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), approve_msg)
        .unwrap();

    let revoke_msg = ExecuteMsg::Revoke {
        spender: deps.api.addr_make("random").to_string(),
        token_id,
    };
    contract
        .execute(deps.as_mut(), mock_env(), owner, revoke_msg)
        .unwrap();

    // Approvals are now removed / cleared
    let res: OwnerOfResponse = from_json(
        contract
            .query(deps.as_ref(), mock_env(), query_msg)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        res,
        OwnerOfResponse {
            owner: deps.api.addr_make("person").to_string(),
            approvals: vec![],
        }
    );
}

#[test]
fn approving_all_revoking_all() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let contract = setup_contract(deps.as_mut(), creator, minter);

    // Mint a couple tokens (from the same owner)
    let token_id1 = "grow1".to_string();
    let token_uri1 = "https://www.merriam-webster.com/dictionary/grow1".to_string();

    let token_id2 = "grow2".to_string();
    let token_uri2 = "https://www.merriam-webster.com/dictionary/grow2".to_string();

    let mint_msg1 = ExecuteMsg::Mint {
        token_id: token_id1.clone(),
        owner: deps.api.addr_make("demeter").to_string(),
        token_uri: Some(token_uri1),
        extension: None,
    };

    let minter = mock_info(deps.api.addr_make(MINTER).as_ref(), &[]);
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg1)
        .unwrap();

    let mint_msg2 = ExecuteMsg::Mint {
        token_id: token_id2.clone(),
        owner: deps.api.addr_make("demeter").to_string(),
        token_uri: Some(token_uri2),
        extension: None,
    };

    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg2)
        .unwrap();

    // paginate the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, Some(1)).unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id1.clone()], tokens.tokens);
    let tokens = contract
        .all_tokens(deps.as_ref(), Some(token_id1.clone()), Some(3))
        .unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id2.clone()], tokens.tokens);

    // demeter gives random full (operator) power over her tokens
    let approve_all_msg = ExecuteMsg::ApproveAll {
        operator: deps.api.addr_make("random").to_string(),
        expires: None,
    };
    let owner = mock_info(deps.api.addr_make("demeter").as_ref(), &[]);
    let res = contract
        .execute(deps.as_mut(), mock_env(), owner, approve_all_msg)
        .unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "approve_all")
            .add_attribute("sender", deps.api.addr_make("demeter").as_ref())
            .add_attribute("operator", deps.api.addr_make("random").as_ref())
    );

    // random can now transfer
    let random = mock_info(deps.api.addr_make("random").as_ref(), &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: deps.api.addr_make("person").to_string(),
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
        contract: deps.api.addr_make("another_contract").to_string(),
        token_id: token_id2,
        msg: to_json_binary(&msg).unwrap(),
    };
    contract
        .execute(deps.as_mut(), mock_env(), random, send_msg)
        .unwrap();

    // Approve_all, revoke_all, and check for empty, to test revoke_all
    let approve_all_msg = ExecuteMsg::ApproveAll {
        operator: deps.api.addr_make("operator").to_string(),
        expires: None,
    };
    // person is now the owner of the tokens
    let owner = mock_info(deps.api.addr_make("person").as_ref(), &[]);
    contract
        .execute(deps.as_mut(), mock_env(), owner, approve_all_msg)
        .unwrap();

    // query for operator should return approval
    let res = contract
        .operator(
            deps.as_ref(),
            mock_env(),
            deps.api.addr_make("person").to_string(),
            deps.api.addr_make("operator").to_string(),
            true,
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorResponse {
            approval: Approval {
                spender: deps.api.addr_make("operator").to_string(),
                expires: Expiration::Never {}
            }
        }
    );

    // query for other should throw error
    let res = contract.operator(
        deps.as_ref(),
        mock_env(),
        deps.api.addr_make("person").to_string(),
        deps.api.addr_make("other").to_string(),
        true,
    );
    match res {
        Err(StdError::NotFound { kind, .. }) => assert_eq!(kind, "Approval not found"),
        _ => panic!("Unexpected error"),
    }

    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            deps.api.addr_make("person").to_string(),
            true,
            None,
            None,
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: deps.api.addr_make("operator").to_string(),
                expires: Expiration::Never {}
            }]
        }
    );

    // second approval
    let buddy_expires = Expiration::AtHeight(1234567);
    let approve_all_msg = ExecuteMsg::ApproveAll {
        operator: deps.api.addr_make("buddy").to_string(),
        expires: Some(buddy_expires),
    };
    let owner = mock_info(deps.api.addr_make("person").as_ref(), &[]);
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), approve_all_msg)
        .unwrap();

    // and paginate queries
    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            deps.api.addr_make("person").to_string(),
            true,
            None,
            Some(1),
        )
        .unwrap();
    // addr_make actually makes the buddy address *after* the operator one

    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: deps.api.addr_make("operator").to_string(),
                expires: Expiration::Never {}
            }]
        }
    );
    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            deps.api.addr_make("person").to_string(),
            true,
            Some(deps.api.addr_make("operator").to_string()),
            Some(2),
        )
        .unwrap();

    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: deps.api.addr_make("buddy").to_string(),
                expires: buddy_expires,
            }]
        }
    );
    let revoke_all_msg = ExecuteMsg::RevokeAll {
        operator: deps.api.addr_make("operator").to_string(),
    };
    contract
        .execute(deps.as_mut(), mock_env(), owner, revoke_all_msg)
        .unwrap();

    // query for operator should return error
    let res = contract.operator(
        deps.as_ref(),
        mock_env(),
        deps.api.addr_make("person").to_string(),
        deps.api.addr_make("operator").to_string(),
        true,
    );
    match res {
        Err(StdError::NotFound { kind, .. }) => assert_eq!(kind, "Approval not found"),
        _ => panic!("Unexpected error"),
    }

    // Approvals are removed / cleared without affecting others
    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            deps.api.addr_make("person").to_string(),
            false,
            None,
            None,
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: deps.api.addr_make("buddy").to_string(),
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
            deps.api.addr_make("person").to_string(),
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
        deps.api.addr_make("person").to_string(),
        deps.api.addr_make("buddy").to_string(),
        false,
    );

    match res {
        Err(StdError::NotFound { kind, .. }) => assert_eq!(kind, "Approval not found"),
        _ => panic!("Unexpected error"),
    }
}

#[test]
fn test_set_withdraw_address() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let contract = setup_contract(deps.as_mut(), creator, minter.clone());
    let other = deps.api.addr_make("other");
    // other cant set
    let foo = deps.api.addr_make("foo");
    let err = contract
        .set_withdraw_address(deps.as_mut(), &other, foo.to_string())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

    // minter can set
    contract
        .set_withdraw_address(deps.as_mut(), &minter, foo.to_string())
        .unwrap();

    let withdraw_address = contract
        .withdraw_address
        .load(deps.as_ref().storage)
        .unwrap();
    assert_eq!(withdraw_address, foo.to_string())
}

#[test]
fn test_remove_withdraw_address() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let contract = setup_contract(deps.as_mut(), creator, minter);
    let other = deps.api.addr_make("other");
    let foo = deps.api.addr_make("foo");
    // other cant remove
    let err = contract
        .remove_withdraw_address(deps.as_mut().storage, &other)
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));
    let minter = deps.api.addr_make(MINTER);

    // no owner set yet
    let err = contract
        .remove_withdraw_address(deps.as_mut().storage, &minter)
        .unwrap_err();
    assert_eq!(err, ContractError::NoWithdrawAddress {});

    // set and remove
    contract
        .set_withdraw_address(deps.as_mut(), &minter, foo.to_string())
        .unwrap();
    contract
        .remove_withdraw_address(deps.as_mut().storage, &minter)
        .unwrap();
    assert!(!contract.withdraw_address.exists(deps.as_ref().storage));

    // test that we can set again
    contract
        .set_withdraw_address(deps.as_mut(), &minter, foo.to_string())
        .unwrap();
    let withdraw_address = contract
        .withdraw_address
        .load(deps.as_ref().storage)
        .unwrap();
    assert_eq!(withdraw_address, foo.to_string())
}

#[test]
fn test_withdraw_funds() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let foo = deps.api.addr_make("foo");
    let contract = setup_contract(deps.as_mut(), creator, minter.clone());

    // no withdraw address set
    let err = contract
        .withdraw_funds(deps.as_mut().storage, &Coin::new(100u32, "uark"))
        .unwrap_err();
    assert_eq!(err, ContractError::NoWithdrawAddress {});

    // set and withdraw by non-owner
    contract
        .set_withdraw_address(deps.as_mut(), &minter, foo.to_string())
        .unwrap();
    contract
        .withdraw_funds(deps.as_mut().storage, &Coin::new(100u32, "uark"))
        .unwrap();
}

#[test]
fn query_tokens_by_owner() {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let minter = deps.api.addr_make(MINTER);
    let contract = setup_contract(deps.as_mut(), creator, minter.clone());

    let minter = mock_info(minter.as_ref(), &[]);

    // Mint a couple tokens (from the same owner)
    let token_id1 = "grow1".to_string();
    let demeter = deps.api.addr_make("demeter");
    let token_id2 = "grow2".to_string();
    let ceres = deps.api.addr_make("ceres");
    let token_id3 = "sing".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id1.clone(),
        owner: demeter.to_string(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg)
        .unwrap();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id2.clone(),
        owner: ceres.to_string(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg)
        .unwrap();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id3.clone(),
        owner: demeter.to_string(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    // get all tokens in order:
    let expected = vec![token_id1.clone(), token_id2.clone(), token_id3.clone()];
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(&expected, &tokens.tokens);
    // paginate
    let tokens = contract.all_tokens(deps.as_ref(), None, Some(2)).unwrap();
    assert_eq!(&expected[..2], &tokens.tokens[..]);
    let tokens = contract
        .all_tokens(deps.as_ref(), Some(expected[1].clone()), None)
        .unwrap();
    assert_eq!(&expected[2..], &tokens.tokens[..]);

    // get by owner
    let by_ceres = vec![token_id2];
    let by_demeter = vec![token_id1, token_id3];
    // all tokens by owner
    let tokens = contract
        .tokens(deps.as_ref(), demeter.to_string(), None, None)
        .unwrap();
    assert_eq!(&by_demeter, &tokens.tokens);
    let tokens = contract
        .tokens(deps.as_ref(), ceres.to_string(), None, None)
        .unwrap();
    assert_eq!(&by_ceres, &tokens.tokens);

    // paginate for demeter
    let tokens = contract
        .tokens(deps.as_ref(), demeter.to_string(), None, Some(1))
        .unwrap();
    assert_eq!(&by_demeter[..1], &tokens.tokens[..]);
    let tokens = contract
        .tokens(
            deps.as_ref(),
            demeter.to_string(),
            Some(by_demeter[0].clone()),
            Some(3),
        )
        .unwrap();
    assert_eq!(&by_demeter[1..], &tokens.tokens[..]);
}
