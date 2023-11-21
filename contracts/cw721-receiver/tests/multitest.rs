use cosmwasm_std::{to_binary, Addr};
use cw_multi_test::{App, ContractWrapper, Executor};

#[test]
fn test_cw721_base_receive() {
    use cw721_receiver::contract::*;
    use cw721_receiver::msg::*;

    use cw721_base::msg as base_msg;

    let mut app = App::default();
    let code_id = app.store_code(Box::new(ContractWrapper::new(execute, instantiate, query)));
    let nft_code_id = app.store_code(Box::new(ContractWrapper::new(
        cw721_base::entry::execute,
        cw721_base::entry::instantiate,
        cw721_base::entry::query,
    )));
    let admin = Addr::unchecked("admin");

    // setup contracts
    let nft_contract = app
        .instantiate_contract(
            nft_code_id,
            admin.clone(),
            &base_msg::InstantiateMsg {
                name: "nft".to_string(),
                symbol: "NFT".to_string(),
                minter: admin.to_string(),
            },
            &[],
            "nft".to_string(),
            None,
        )
        .unwrap();

    let receiver_contract = app
        .instantiate_contract(
            code_id,
            admin.clone(),
            &InstantiateMsg {},
            &[],
            "receiver".to_string(),
            None,
        )
        .unwrap();

    // mint token
    app.execute_contract(
        admin.clone(),
        nft_contract.clone(),
        &base_msg::ExecuteMsg::<(), ()>::Mint {
            token_id: "test".to_string(),
            owner: admin.to_string(),
            token_uri: Some("https://example.com".to_string()),
            extension: (),
        },
        &[],
    )
    .unwrap();

    // send token to receiver contract
    let response = app
        .execute_contract(
            admin,
            nft_contract,
            &base_msg::ExecuteMsg::<(), ()>::SendNft {
                contract: receiver_contract.to_string(),
                token_id: "test".to_string(),
                msg: to_binary(&InnerMsg::Succeed).unwrap(),
            },
            &[],
        )
        .unwrap();

    println!("{:?}", response);
}
