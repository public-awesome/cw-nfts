#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

use cosmwasm_std::{
    from_json, to_json_binary, Addr, Coin, CosmosMsg, DepsMut, Empty, Response, StdError,
    Timestamp, WasmMsg,
};

use crate::error::Cw721ContractError;
use crate::msg::{
    ApprovalResponse, NftInfoResponse, OperatorResponse, OperatorsResponse, OwnerOfResponse,
};
use crate::msg::{CollectionInfoMsg, Cw721ExecuteMsg, Cw721InstantiateMsg, Cw721QueryMsg};
use crate::receiver::Cw721ReceiveMsg;
use crate::state::{
    CollectionInfo, DefaultOptionCollectionInfoExtension, DefaultOptionMetadataExtension, CREATOR,
    MINTER,
};
use crate::{execute::Cw721Execute, query::Cw721Query, Approval, Expiration};
use crate::{CollectionInfoExtension, RoyaltyInfo};
use cw_ownable::{Action, Ownership, OwnershipError};

use super::contract::Cw721Contract;

const MINTER_ADDR: &str = "minter";
const CREATOR_ADDR: &str = "creator";
const CONTRACT_NAME: &str = "Magic Power";
const SYMBOL: &str = "MGK";

fn setup_contract(
    deps: DepsMut<'_>,
) -> Cw721Contract<'static, DefaultOptionMetadataExtension, Empty, Empty, Empty> {
    let contract = Cw721Contract::default();
    let msg = Cw721InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        collection_info_extension: Empty {},
        minter: Some(String::from(MINTER_ADDR)),
        creator: Some(String::from(CREATOR_ADDR)),
        withdraw_address: None,
    };
    let info = mock_info("creator", &[]);
    let res = contract
        .instantiate(
            deps,
            mock_env(),
            info,
            msg,
            "contract_name",
            "contract_version",
        )
        .unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let contract = Cw721Contract::<DefaultOptionMetadataExtension, Empty, Empty, Empty>::default();

    let msg = Cw721InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        collection_info_extension: Empty {},
        minter: Some(String::from(MINTER_ADDR)),
        creator: Some(String::from(CREATOR_ADDR)),
        withdraw_address: Some(String::from(CREATOR_ADDR)),
    };
    let info = mock_info("creator", &[]);
    let env = mock_env();

    // we can just call .unwrap() to assert this was a success
    let res = contract
        .instantiate(
            deps.as_mut(),
            env.clone(),
            info,
            msg,
            "contract_name",
            "contract_version",
        )
        .unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let minter_ownership = MINTER.get_ownership(deps.as_ref().storage).unwrap();
    assert_eq!(Some(Addr::unchecked(MINTER_ADDR)), minter_ownership.owner);
    let creator_ownership = CREATOR.get_ownership(deps.as_ref().storage).unwrap();
    assert_eq!(Some(Addr::unchecked(CREATOR_ADDR)), creator_ownership.owner);
    let collection_info = contract
        .query_collection_info(deps.as_ref(), env.clone())
        .unwrap();
    assert_eq!(
        collection_info,
        CollectionInfo {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
            extension: Empty {},
            updated_at: env.block.time
        }
    );

    let withdraw_address = contract
        .config
        .withdraw_address
        .may_load(deps.as_ref().storage)
        .unwrap();
    assert_eq!(Some(CREATOR_ADDR.to_string()), withdraw_address);

    let count = contract
        .query_num_tokens(deps.as_ref(), env.clone())
        .unwrap();
    assert_eq!(0, count.count);

    // list the token_ids
    let tokens = contract
        .query_all_tokens(deps.as_ref(), env, None, None)
        .unwrap();
    assert_eq!(0, tokens.tokens.len());
}

#[test]
fn proper_instantiation_with_collection_info() {
    let mut deps = mock_dependencies();
    let contract = Cw721Contract::<
        DefaultOptionMetadataExtension,
        Empty,
        Empty,
        DefaultOptionCollectionInfoExtension,
    >::default();

    let collection_info_extension = Some(CollectionInfoExtension {
        description: "description".to_string(),
        image: "image".to_string(),
        explicit_content: Some(true),
        external_link: Some("external_link".to_string()),
        start_trading_time: Some(Timestamp::from_seconds(0)),
        royalty_info: Some(RoyaltyInfo {
            payment_address: Addr::unchecked("payment_address"),
            share: "0.1".parse().unwrap(),
        }),
    });
    let msg = Cw721InstantiateMsg::<DefaultOptionCollectionInfoExtension> {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        collection_info_extension: collection_info_extension.clone(),
        minter: Some(String::from(MINTER_ADDR)),
        creator: Some(String::from(CREATOR_ADDR)),
        withdraw_address: Some(String::from(CREATOR_ADDR)),
    };
    let collection_info = mock_info("creator", &[]);
    let env = mock_env();

    // we can just call .unwrap() to assert this was a success
    let res = contract
        .instantiate(
            deps.as_mut(),
            env.clone(),
            collection_info,
            msg,
            "contract_name",
            "contract_version",
        )
        .unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let minter_ownership = MINTER.get_ownership(deps.as_ref().storage).unwrap();
    assert_eq!(Some(Addr::unchecked(MINTER_ADDR)), minter_ownership.owner);
    let creator_ownership = CREATOR.get_ownership(deps.as_ref().storage).unwrap();
    assert_eq!(Some(Addr::unchecked(CREATOR_ADDR)), creator_ownership.owner);
    let info = contract
        .query_collection_info(deps.as_ref(), env.clone())
        .unwrap();
    assert_eq!(
        info,
        CollectionInfo {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
            extension: collection_info_extension,
            updated_at: env.block.time
        }
    );

    let withdraw_address = contract
        .config
        .withdraw_address
        .may_load(deps.as_ref().storage)
        .unwrap();
    assert_eq!(Some(CREATOR_ADDR.to_string()), withdraw_address);

    let count = contract
        .query_num_tokens(deps.as_ref(), env.clone())
        .unwrap();
    assert_eq!(0, count.count);

    // list the token_ids
    let tokens = contract
        .query_all_tokens(deps.as_ref(), env, None, None)
        .unwrap();
    assert_eq!(0, tokens.tokens.len());
}

#[test]
fn minting() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("medusa"),
        token_uri: Some(token_uri.clone()),
        extension: None,
    };

    // random cannot mint
    let random = mock_info("random", &[]);
    let env = mock_env();
    let err = contract
        .execute(deps.as_mut(), env.clone(), random, mint_msg.clone())
        .unwrap_err();
    assert_eq!(err, Cw721ContractError::Ownership(OwnershipError::NotOwner));

    // minter can mint
    let allowed = mock_info(MINTER_ADDR, &[]);
    let _ = contract
        .execute(deps.as_mut(), env.clone(), allowed, mint_msg)
        .unwrap();

    // ensure num tokens increases
    let count = contract
        .query_num_tokens(deps.as_ref(), env.clone())
        .unwrap();
    assert_eq!(1, count.count);

    // unknown nft returns error
    let _ = contract
        .query_nft_info(deps.as_ref(), env.clone(), "unknown".to_string())
        .unwrap_err();

    // this nft info is correct
    let info = contract
        .query_nft_info(deps.as_ref(), env.clone(), token_id.clone())
        .unwrap();
    assert_eq!(
        info,
        NftInfoResponse::<DefaultOptionMetadataExtension> {
            token_uri: Some(token_uri),
            extension: None,
        }
    );

    // owner info is correct
    let owner = contract
        .query_owner_of(deps.as_ref(), mock_env(), token_id.clone(), true)
        .unwrap();
    assert_eq!(
        owner,
        OwnerOfResponse {
            owner: String::from("medusa"),
            approvals: vec![],
        }
    );

    // Cannot mint same token_id again
    let mint_msg2 = Cw721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("hercules"),
        token_uri: None,
        extension: None,
    };

    let allowed = mock_info(MINTER_ADDR, &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), allowed, mint_msg2)
        .unwrap_err();
    assert_eq!(err, Cw721ContractError::Claimed {});

    // list the token_ids
    let tokens = contract
        .query_all_tokens(deps.as_ref(), env, None, None)
        .unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id], tokens.tokens);
}

#[test]
fn test_update_collection_info() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let update_collection_info_msg = Cw721ExecuteMsg::UpdateCollectionInfo {
        collection_info: CollectionInfoMsg {
            name: "new name".to_string(),
            symbol: "NEW".to_string(),
            extension: Empty {},
        },
    };

    // Creator can update collection info
    let creator_info = mock_info(CREATOR_ADDR, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            creator_info.clone(),
            update_collection_info_msg,
        )
        .unwrap();

    // Update the owner to "random". The new owner should be able to
    // mint new tokens, the old one should not.
    contract
        .execute(
            deps.as_mut(),
            mock_env(),
            creator_info.clone(),
            Cw721ExecuteMsg::UpdateCreatorOwnership(Action::TransferOwnership {
                new_owner: "random".to_string(),
                expiry: None,
            }),
        )
        .unwrap();

    // Creator does not change until ownership transfer completes.
    // Pending ownership transfer should be discoverable via query.
    let ownership: Ownership<Addr> = from_json(
        contract
            .query(
                deps.as_ref(),
                mock_env(),
                Cw721QueryMsg::GetCreatorOwnership {},
            )
            .unwrap(),
    )
    .unwrap();

    assert_eq!(
        ownership,
        Ownership::<Addr> {
            owner: Some(Addr::unchecked(CREATOR_ADDR)),
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
            Cw721ExecuteMsg::UpdateCreatorOwnership(Action::AcceptOwnership),
        )
        .unwrap();

    // Creator changes after ownership transfer is accepted.
    let creator_ownership: Ownership<Addr> = from_json(
        contract
            .query(
                deps.as_ref(),
                mock_env(),
                Cw721QueryMsg::GetCreatorOwnership {},
            )
            .unwrap(),
    )
    .unwrap();
    assert_eq!(creator_ownership.owner, Some(random_info.sender.clone()));

    let update_collection_info_msg = Cw721ExecuteMsg::UpdateCollectionInfo {
        collection_info: CollectionInfoMsg {
            name: "new name".to_string(),
            symbol: "NEW".to_string(),
            extension: Empty {},
        },
    };

    // Old owner can not update.
    let err: Cw721ContractError = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            creator_info,
            update_collection_info_msg.clone(),
        )
        .unwrap_err();
    assert_eq!(err, Cw721ContractError::Ownership(OwnershipError::NotOwner));

    // New owner can update.
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            random_info,
            update_collection_info_msg,
        )
        .unwrap();
}

#[test]
fn test_update_minter() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id,
        owner: String::from("medusa"),
        token_uri: Some(token_uri.clone()),
        extension: None,
    };

    // Minter can mint
    let minter_info = mock_info(MINTER_ADDR, &[]);
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
            Cw721ExecuteMsg::UpdateMinterOwnership(Action::TransferOwnership {
                new_owner: "random".to_string(),
                expiry: None,
            }),
        )
        .unwrap();

    // Minter does not change until ownership transfer completes.
    // Pending ownership transfer should be discoverable via query.
    let ownership: Ownership<Addr> = from_json(
        contract
            .query(
                deps.as_ref(),
                mock_env(),
                Cw721QueryMsg::GetMinterOwnership {},
            )
            .unwrap(),
    )
    .unwrap();

    assert_eq!(
        ownership,
        Ownership::<Addr> {
            owner: Some(Addr::unchecked(MINTER_ADDR)),
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
            Cw721ExecuteMsg::UpdateMinterOwnership(Action::AcceptOwnership),
        )
        .unwrap();

    // Minter changes after ownership transfer is accepted.
    let minter_ownership: Ownership<Addr> = from_json(
        contract
            .query(
                deps.as_ref(),
                mock_env(),
                Cw721QueryMsg::GetMinterOwnership {},
            )
            .unwrap(),
    )
    .unwrap();
    assert_eq!(minter_ownership.owner, Some(random_info.sender.clone()));

    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id: "randoms_token".to_string(),
        owner: String::from("medusa"),
        token_uri: Some(token_uri),
        extension: None,
    };

    // Old owner can not mint.
    let err: Cw721ContractError = contract
        .execute(deps.as_mut(), mock_env(), minter_info, mint_msg.clone())
        .unwrap_err();
    assert_eq!(err, Cw721ContractError::Ownership(OwnershipError::NotOwner));

    // New owner can mint.
    let _ = contract
        .execute(deps.as_mut(), mock_env(), random_info, mint_msg)
        .unwrap();
}

#[test]
fn burning() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: MINTER_ADDR.to_string(),
        token_uri: Some(token_uri),
        extension: None,
    };

    let burn_msg = Cw721ExecuteMsg::Burn { token_id };

    // mint some NFT
    let allowed = mock_info(MINTER_ADDR, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), allowed.clone(), mint_msg)
        .unwrap();

    // random not allowed to burn
    let random = mock_info("random", &[]);
    let env = mock_env();
    let err = contract
        .execute(deps.as_mut(), env.clone(), random, burn_msg.clone())
        .unwrap_err();

    assert_eq!(err, Cw721ContractError::Ownership(OwnershipError::NotOwner));

    let _ = contract
        .execute(deps.as_mut(), env.clone(), allowed, burn_msg)
        .unwrap();

    // ensure num tokens decreases
    let count = contract
        .query_num_tokens(deps.as_ref(), env.clone())
        .unwrap();
    assert_eq!(0, count.count);

    // trying to get nft returns error
    let _ = contract
        .query_nft_info(deps.as_ref(), env.clone(), "petrify".to_string())
        .unwrap_err();

    // list the token_ids
    let tokens = contract
        .query_all_tokens(deps.as_ref(), env, None, None)
        .unwrap();
    assert!(tokens.tokens.is_empty());
}

#[test]
fn transferring_nft() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    // Mint a token
    let token_id = "melt".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("venus"),
        token_uri: Some(token_uri),
        extension: None,
    };

    let minter = mock_info(MINTER_ADDR, &[]);
    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    // random cannot transfer
    let random = mock_info("random", &[]);
    let transfer_msg = Cw721ExecuteMsg::TransferNft {
        recipient: String::from("random"),
        token_id: token_id.clone(),
    };

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, transfer_msg)
        .unwrap_err();
    assert_eq!(err, Cw721ContractError::Ownership(OwnershipError::NotOwner));

    // owner can
    let random = mock_info("venus", &[]);
    let transfer_msg = Cw721ExecuteMsg::TransferNft {
        recipient: String::from("random"),
        token_id: token_id.clone(),
    };

    let res = contract
        .execute(deps.as_mut(), mock_env(), random, transfer_msg)
        .unwrap();

    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "transfer_nft")
            .add_attribute("sender", "venus")
            .add_attribute("recipient", "random")
            .add_attribute("token_id", token_id)
    );
}

#[test]
fn sending_nft() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    // Mint a token
    let token_id = "melt".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("venus"),
        token_uri: Some(token_uri),
        extension: None,
    };

    let minter = mock_info(MINTER_ADDR, &[]);
    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    let msg = to_json_binary("You now have the melting power").unwrap();
    let target = String::from("another_contract");
    let send_msg = Cw721ExecuteMsg::SendNft {
        contract: target.clone(),
        token_id: token_id.clone(),
        msg: msg.clone(),
    };

    let random = mock_info("random", &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, send_msg.clone())
        .unwrap_err();
    assert_eq!(err, Cw721ContractError::Ownership(OwnershipError::NotOwner));

    // but owner can
    let random = mock_info("venus", &[]);
    let res = contract
        .execute(deps.as_mut(), mock_env(), random, send_msg)
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
            .add_attribute("token_id", token_id)
    );
}

#[test]
fn approving_revoking() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    // Mint a token
    let token_id = "grow".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/grow".to_string();

    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("demeter"),
        token_uri: Some(token_uri),
        extension: None,
    };

    let minter = mock_info(MINTER_ADDR, &[]);
    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    // token owner shows in approval query
    let res = contract
        .query_approval(
            deps.as_ref(),
            mock_env(),
            token_id.clone(),
            String::from("demeter"),
            false,
        )
        .unwrap();
    assert_eq!(
        res,
        ApprovalResponse {
            approval: Approval {
                spender: Addr::unchecked("demeter"),
                expires: Expiration::Never {}
            }
        }
    );

    // Give random transferring power
    let approve_msg = Cw721ExecuteMsg::Approve {
        spender: String::from("random"),
        token_id: token_id.clone(),
        expires: None,
    };
    let owner = mock_info("demeter", &[]);
    let res = contract
        .execute(deps.as_mut(), mock_env(), owner, approve_msg)
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
        .query_approval(
            deps.as_ref(),
            mock_env(),
            token_id.clone(),
            String::from("random"),
            true,
        )
        .unwrap();
    assert_eq!(
        res,
        ApprovalResponse {
            approval: Approval {
                spender: Addr::unchecked("random"),
                expires: Expiration::Never {}
            }
        }
    );

    // random can now transfer
    let random = mock_info("random", &[]);
    let transfer_msg = Cw721ExecuteMsg::TransferNft {
        recipient: String::from("person"),
        token_id: token_id.clone(),
    };
    contract
        .execute(deps.as_mut(), mock_env(), random, transfer_msg)
        .unwrap();

    // Approvals are removed / cleared
    let query_msg = Cw721QueryMsg::OwnerOf {
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
            owner: String::from("person"),
            approvals: vec![],
        }
    );

    // Approve, revoke, and check for empty, to test revoke
    let approve_msg = Cw721ExecuteMsg::Approve {
        spender: String::from("random"),
        token_id: token_id.clone(),
        expires: None,
    };
    let owner = mock_info("person", &[]);
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), approve_msg)
        .unwrap();

    let revoke_msg = Cw721ExecuteMsg::Revoke {
        spender: String::from("random"),
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
            owner: String::from("person"),
            approvals: vec![],
        }
    );
}

#[test]
fn approving_all_revoking_all() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    // Mint a couple tokens (from the same owner)
    let token_id1 = "grow1".to_string();
    let token_uri1 = "https://www.merriam-webster.com/dictionary/grow1".to_string();

    let token_id2 = "grow2".to_string();
    let token_uri2 = "https://www.merriam-webster.com/dictionary/grow2".to_string();

    let mint_msg1 = Cw721ExecuteMsg::Mint {
        token_id: token_id1.clone(),
        owner: String::from("demeter"),
        token_uri: Some(token_uri1),
        extension: None,
    };

    let minter = mock_info(MINTER_ADDR, &[]);
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg1)
        .unwrap();

    let mint_msg2 = Cw721ExecuteMsg::Mint {
        token_id: token_id2.clone(),
        owner: String::from("demeter"),
        token_uri: Some(token_uri2),
        extension: None,
    };

    let env = mock_env();
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg2)
        .unwrap();

    // paginate the token_ids
    let tokens = contract
        .query_all_tokens(deps.as_ref(), env.clone(), None, Some(1))
        .unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id1.clone()], tokens.tokens);
    let tokens = contract
        .query_all_tokens(deps.as_ref(), env, Some(token_id1.clone()), Some(3))
        .unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id2.clone()], tokens.tokens);

    // demeter gives random full (operator) power over her tokens
    let approve_all_msg = Cw721ExecuteMsg::ApproveAll {
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
    let transfer_msg = Cw721ExecuteMsg::TransferNft {
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

    let send_msg = Cw721ExecuteMsg::SendNft {
        contract: String::from("another_contract"),
        token_id: token_id2,
        msg: to_json_binary(&msg).unwrap(),
    };
    contract
        .execute(deps.as_mut(), mock_env(), random, send_msg)
        .unwrap();

    // Approve_all, revoke_all, and check for empty, to test revoke_all
    let approve_all_msg = Cw721ExecuteMsg::ApproveAll {
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
        .query_operator(
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
                spender: Addr::unchecked("operator"),
                expires: Expiration::Never {}
            }
        }
    );

    // query for other should throw error
    let res = contract.query_operator(
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
        .query_operators(
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
            operators: vec![Approval {
                spender: Addr::unchecked("operator"),
                expires: Expiration::Never {}
            }]
        }
    );

    // second approval
    let buddy_expires = Expiration::AtHeight(1234567);
    let approve_all_msg = Cw721ExecuteMsg::ApproveAll {
        operator: String::from("buddy"),
        expires: Some(buddy_expires),
    };
    let owner = mock_info("person", &[]);
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), approve_all_msg)
        .unwrap();

    // and paginate queries
    let res = contract
        .query_operators(
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
            operators: vec![Approval {
                spender: Addr::unchecked("buddy"),
                expires: buddy_expires,
            }]
        }
    );
    let res = contract
        .query_operators(
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
            operators: vec![Approval {
                spender: Addr::unchecked("operator"),
                expires: Expiration::Never {}
            }]
        }
    );

    let revoke_all_msg = Cw721ExecuteMsg::RevokeAll {
        operator: String::from("operator"),
    };
    contract
        .execute(deps.as_mut(), mock_env(), owner, revoke_all_msg)
        .unwrap();

    // query for operator should return error
    let res = contract.query_operator(
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
        .query_operators(
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
            operators: vec![Approval {
                spender: Addr::unchecked("buddy"),
                expires: buddy_expires,
            }]
        }
    );

    // ensure the filter works (nothing should be here
    let mut late_env = mock_env();
    late_env.block.height = 1234568; //expired
    let res = contract
        .query_operators(
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
    let res = contract.query_operator(
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
fn test_set_withdraw_address() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    // other than creator cant set
    let err = contract
        .set_withdraw_address(
            deps.as_mut(),
            &Addr::unchecked(MINTER_ADDR),
            "foo".to_string(),
        )
        .unwrap_err();
    assert_eq!(err, Cw721ContractError::Ownership(OwnershipError::NotOwner));

    // creator can set
    contract
        .set_withdraw_address(
            deps.as_mut(),
            &Addr::unchecked(CREATOR_ADDR),
            "foo".to_string(),
        )
        .unwrap();

    let withdraw_address = contract
        .config
        .withdraw_address
        .load(deps.as_ref().storage)
        .unwrap();
    assert_eq!(withdraw_address, "foo".to_string())
}

#[test]
fn test_remove_withdraw_address() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    // other than creator cant remove
    let err = contract
        .remove_withdraw_address(deps.as_mut().storage, &Addr::unchecked(MINTER_ADDR))
        .unwrap_err();
    assert_eq!(err, Cw721ContractError::Ownership(OwnershipError::NotOwner));

    // no withdraw address set yet
    let err = contract
        .remove_withdraw_address(deps.as_mut().storage, &Addr::unchecked(CREATOR_ADDR))
        .unwrap_err();
    assert_eq!(err, Cw721ContractError::NoWithdrawAddress {});

    // set and remove
    contract
        .set_withdraw_address(
            deps.as_mut(),
            &Addr::unchecked(CREATOR_ADDR),
            "foo".to_string(),
        )
        .unwrap();
    contract
        .remove_withdraw_address(deps.as_mut().storage, &Addr::unchecked(CREATOR_ADDR))
        .unwrap();
    assert!(!contract
        .config
        .withdraw_address
        .exists(deps.as_ref().storage));

    // test that we can set again
    contract
        .set_withdraw_address(
            deps.as_mut(),
            &Addr::unchecked(CREATOR_ADDR),
            "foo".to_string(),
        )
        .unwrap();
    let withdraw_address = contract
        .config
        .withdraw_address
        .load(deps.as_ref().storage)
        .unwrap();
    assert_eq!(withdraw_address, "foo".to_string())
}

#[test]
fn test_withdraw_funds() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    // no withdraw address set
    let err = contract
        .withdraw_funds(deps.as_mut().storage, &Coin::new(100, "uark"))
        .unwrap_err();
    assert_eq!(err, Cw721ContractError::NoWithdrawAddress {});

    // set and withdraw by non-creator
    contract
        .set_withdraw_address(
            deps.as_mut(),
            &Addr::unchecked(CREATOR_ADDR),
            "foo".to_string(),
        )
        .unwrap();
    contract
        .withdraw_funds(deps.as_mut().storage, &Coin::new(100, "uark"))
        .unwrap();
}

#[test]
fn query_tokens_by_owner() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());
    let minter = mock_info(MINTER_ADDR, &[]);

    // Mint a couple tokens (from the same owner)
    let token_id1 = "grow1".to_string();
    let demeter = String::from("demeter");
    let token_id2 = "grow2".to_string();
    let ceres = String::from("ceres");
    let token_id3 = "sing".to_string();

    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id: token_id1.clone(),
        owner: demeter.clone(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg)
        .unwrap();

    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id: token_id2.clone(),
        owner: ceres.clone(),
        token_uri: None,
        extension: None,
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg)
        .unwrap();

    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id: token_id3.clone(),
        owner: demeter.clone(),
        token_uri: None,
        extension: None,
    };
    let env = mock_env();
    contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg)
        .unwrap();

    // get all tokens in order:
    let expected = vec![token_id1.clone(), token_id2.clone(), token_id3.clone()];
    let tokens = contract
        .query_all_tokens(deps.as_ref(), env.clone(), None, None)
        .unwrap();
    assert_eq!(&expected, &tokens.tokens);
    // paginate
    let tokens = contract
        .query_all_tokens(deps.as_ref(), env.clone(), None, Some(2))
        .unwrap();
    assert_eq!(&expected[..2], &tokens.tokens[..]);
    let tokens = contract
        .query_all_tokens(deps.as_ref(), env.clone(), Some(expected[1].clone()), None)
        .unwrap();
    assert_eq!(&expected[2..], &tokens.tokens[..]);

    // get by owner
    let by_ceres = vec![token_id2];
    let by_demeter = vec![token_id1, token_id3];
    // all tokens by owner
    let tokens = contract
        .query_tokens(deps.as_ref(), env.clone(), demeter.clone(), None, None)
        .unwrap();
    assert_eq!(&by_demeter, &tokens.tokens);
    let tokens = contract
        .query_tokens(deps.as_ref(), env.clone(), ceres, None, None)
        .unwrap();
    assert_eq!(&by_ceres, &tokens.tokens);

    // paginate for demeter
    let tokens = contract
        .query_tokens(deps.as_ref(), env.clone(), demeter.clone(), None, Some(1))
        .unwrap();
    assert_eq!(&by_demeter[..1], &tokens.tokens[..]);
    let tokens = contract
        .query_tokens(
            deps.as_ref(),
            env,
            demeter,
            Some(by_demeter[0].clone()),
            Some(3),
        )
        .unwrap();
    assert_eq!(&by_demeter[1..], &tokens.tokens[..]);
}
