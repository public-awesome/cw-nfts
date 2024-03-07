use cosmwasm_std::{to_json_binary, Addr, Attribute, Binary, Empty};
use cw721::state::DefaultOptionCollectionInfoExtension;
use cw_multi_test::{App, ContractWrapper, Executor};

#[test]
fn test_cw721_base_receive_succeed() {
    use cw721_base::msg::*;
    use cw721_receiver_tester::msg::InnerMsg;

    let mut app = App::default();
    let admin = app.api().addr_make("admin");

    let Contracts {
        nft_contract,
        receiver_contract,
    } = setup_contracts(&mut app, admin.clone());

    // send token to receiver contract
    let response = app
        .execute_contract(
            admin.clone(),
            nft_contract,
            &ExecuteMsg::<(), (), ()>::SendNft {
                contract: receiver_contract.to_string(),
                token_id: "test".to_string(),
                msg: to_json_binary(&InnerMsg::Succeed).unwrap(),
            },
            &[],
        )
        .unwrap();
    let mut wasm_events = response.events.iter().filter(|e| e.ty == "wasm");

    let send_event = wasm_events.next().unwrap();
    assert_eq!(
        get_attribute(&send_event.attributes, "action"),
        Some("send_nft")
    );
    assert_eq!(
        get_attribute(&send_event.attributes, "token_id"),
        Some("test")
    );
    assert_eq!(
        get_attribute(&send_event.attributes, "recipient"),
        Some(receiver_contract.as_str())
    );

    let receive_event = wasm_events.next().unwrap();
    assert_eq!(
        get_attribute(&receive_event.attributes, "action"),
        Some("receive_nft")
    );
    assert_eq!(
        get_attribute(&receive_event.attributes, "token_id"),
        Some("test")
    );
    assert_eq!(
        get_attribute(&receive_event.attributes, "sender"),
        Some(admin.as_str()) // this is set to the sender of the original message
    );
}

#[test]
fn test_cw721_base_receive_fail() {
    use cw721_base::msg::*;
    use cw721_receiver_tester::msg::InnerMsg;

    let mut app = App::default();
    let admin = app.api().addr_make("admin");

    let Contracts {
        nft_contract,
        receiver_contract,
    } = setup_contracts(&mut app, admin.clone());

    // send fail message
    let result = app.execute_contract(
        admin.clone(),
        nft_contract.clone(),
        &ExecuteMsg::<(), (), ()>::SendNft {
            contract: receiver_contract.to_string(),
            token_id: "test".to_string(),
            msg: to_json_binary(&InnerMsg::Fail).unwrap(),
        },
        &[],
    );
    assert!(result.is_err());

    // send incorrect message
    let result = app.execute_contract(
        admin,
        nft_contract,
        &ExecuteMsg::<(), (), ()>::SendNft {
            contract: receiver_contract.to_string(),
            token_id: "test".to_string(),
            msg: Binary::from(br#"{"invalid": "fields"}"#),
        },
        &[],
    );
    assert!(result.is_err());
}

struct Contracts {
    nft_contract: Addr,
    receiver_contract: Addr,
}

/// Setup the cw721-receiver and cw721-base contracts and mint a test token
fn setup_contracts(app: &mut App, admin: Addr) -> Contracts {
    use cw721_receiver_tester::contract::*;
    use cw721_receiver_tester::msg::*;

    use cw721_base::msg as base_msg;

    let code_id = app.store_code(Box::new(ContractWrapper::new(execute, instantiate, query)));
    let nft_code_id = app.store_code(Box::new(ContractWrapper::new(
        cw721_base::entry::execute,
        cw721_base::entry::instantiate,
        cw721_base::entry::query,
    )));

    // setup contracts
    let nft_contract = app
        .instantiate_contract(
            nft_code_id,
            admin.clone(),
            &base_msg::InstantiateMsg::<DefaultOptionCollectionInfoExtension> {
                name: "nft".to_string(),
                symbol: "NFT".to_string(),
                collection_info_extension: None,
                minter: Some(admin.to_string()),
                creator: Some(admin.to_string()),
                withdraw_address: None,
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
        &base_msg::ExecuteMsg::<(), (), ()>::Mint {
            token_id: "test".to_string(),
            owner: admin.to_string(),
            token_uri: Some("https://example.com".to_string()),
            extension: (),
        },
        &[],
    )
    .unwrap();

    Contracts {
        nft_contract,
        receiver_contract,
    }
}

fn get_attribute<'a>(attributes: &'a [Attribute], key: &str) -> Option<&'a str> {
    attributes
        .iter()
        .find(|a| a.key == key)
        .map(|a| a.value.as_str())
}
