#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, DepsMut, Empty};

use crate::{
    ContractError, Cw4973Contract, ExecuteMsg, InstantiateMsg, QueryMsg,
};

const MINTER: &str = "minter";

fn setup_contract(deps: DepsMut<'_>) -> Cw4973Contract {
    let contract = Cw4973Contract::default();
    let msg = InstantiateMsg {
        name: "Aura 4973".to_string(),
        symbol: "A4973".to_string(),
        minter: String::from(MINTER),
    };
    let info = mock_info("creator", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    // prepare the mock dependencies
    let mut deps = mock_dependencies();
    
    // setup the contract
    setup_contract(deps.as_mut());

    // get contract by using default Aura4973 contract
    let contract = Cw4973Contract::default();

    // prepare the env
    let env = mock_env();

    // it worked, let's query the state
    let res = contract.query(deps.as_ref(), mock_env(), QueryMsg::ContractInfo {}).unwrap();
    let value: cw721_base::state::ContractInfoResponse = from_binary(&res).unwrap();
    assert_eq!(CONTRACT_NAME, value.name);
    assert_eq!(SYMBOL, value.symbol);
}
