#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

use cosmwasm_std::{
    from_binary, Addr, DepsMut, Empty, Uint128,
};

use cw5144::{
    ContractInfoResponse,
    NftInfoResponse, OwnerOfResponse, Cw5144Query, Soul
};
use cw_ownable::OwnershipError;

use crate::{
    ContractError, ExecuteMsg, Extension, InstantiateMsg, MinterResponse, QueryMsg,
};
use crate::state::Cw5144Contract;

const MINTER: &str = "merlin";
const CONTRACT_NAME: &str = "Magic Power";
const SYMBOL: &str = "MGK";

fn setup_contract(deps: DepsMut<'_>) -> Cw5144Contract<'static, Extension, Empty, Empty, Empty> {
    let contract = Cw5144Contract::default();
    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: String::from(MINTER),
    };
    let info = mock_info("creator", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let contract = Cw5144Contract::<Extension, Empty, Empty, Empty>::default();

    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: String::from(MINTER),
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
        ContractInfoResponse {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
        }
    );

    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(0, count.count);

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(0, tokens.tokens.len());
}

#[test]
fn minting() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: Soul { nft_address: Addr::unchecked("test_1"), token_id: Uint128::from(1u32) },
        token_uri: Some(token_uri.clone()),
        extension: None,
    };

    // random cannot mint
    let random = mock_info("random", &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, mint_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

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
            nft_address: Addr::unchecked("test_1"),
            soul_token_id: Uint128::from(1u32) ,
        }
    );

    // Cannot mint same token_id again
    let mint_msg2 = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: Soul { nft_address: Addr::unchecked("test_2"), token_id: Uint128::from(1u32) },
        token_uri: None,
        extension: None,
    };

    let allowed = mock_info(MINTER, &[]);
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
    let contract = setup_contract(deps.as_mut());

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id,
        owner: Soul { nft_address: Addr::unchecked("test_1"), token_id: Uint128::from(1u32) },
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
    let minter: MinterResponse = from_binary(
        &contract
            .query(deps.as_ref(), mock_env(), QueryMsg::Minter {})
            .unwrap(),
    )
    .unwrap();
    assert_eq!(minter.minter, Some(MINTER.to_string()));

    // Pending ownership transfer should be discoverable via query.
    let ownership: cw_ownable::Ownership<Addr> = from_binary(
        &contract
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
    let minter: MinterResponse = from_binary(
        &contract
            .query(deps.as_ref(), mock_env(), QueryMsg::Minter {})
            .unwrap(),
    )
    .unwrap();
    assert_eq!(minter.minter, Some("random".to_string()));

    let mint_msg = ExecuteMsg::Mint {
        token_id: "randoms_token".to_string(),
        owner: Soul { nft_address: Addr::unchecked("test_1"), token_id: Uint128::from(1u32) },
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
fn query_tokens_by_owner() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());
    let minter = mock_info(MINTER, &[]);

    // Mint a couple tokens (from the same owner)
    let token_id1 = "grow1".to_string();
    let demeter = Soul { nft_address: Addr::unchecked("test_1"), token_id: Uint128::from(1u32) };
    let token_id2 = "grow2".to_string();
    let ceres = Soul { nft_address: Addr::unchecked("test_3"), token_id: Uint128::from(1u32) };
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
        .tokens(deps.as_ref(), demeter.clone(), None, None)
        .unwrap();
    assert_eq!(&by_demeter, &tokens.tokens);
    let tokens = contract.tokens(deps.as_ref(), ceres, None, None).unwrap();
    assert_eq!(&by_ceres, &tokens.tokens);

    // paginate for demeter
    let tokens = contract
        .tokens(deps.as_ref(), demeter.clone(), None, Some(1))
        .unwrap();
    assert_eq!(&by_demeter[..1], &tokens.tokens[..]);
    let tokens = contract
        .tokens(deps.as_ref(), demeter, Some(by_demeter[0].clone()), Some(3))
        .unwrap();
    assert_eq!(&by_demeter[1..], &tokens.tokens[..]);
}
