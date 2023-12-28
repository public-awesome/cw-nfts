use cosmwasm_std::{to_json_binary, Addr, Empty, QuerierWrapper, WasmMsg};
use cw721::OwnerOfResponse;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

use crate::MinterResponse;

fn cw721_base_latest_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::entry::execute,
        crate::entry::instantiate,
        crate::entry::query,
    )
    .with_migrate(crate::entry::migrate);
    Box::new(contract)
}

fn cw721_base_016_contract() -> Box<dyn Contract<Empty>> {
    use cw721_base_016 as v16;
    let contract = ContractWrapper::new(
        v16::entry::execute,
        v16::entry::instantiate,
        v16::entry::query,
    );
    Box::new(contract)
}

fn query_owner(querier: QuerierWrapper, cw721: &Addr, token_id: String) -> Addr {
    let resp: OwnerOfResponse = querier
        .query_wasm_smart(
            cw721,
            &crate::QueryMsg::<Empty>::OwnerOf {
                token_id,
                include_expired: None,
            },
        )
        .unwrap();
    Addr::unchecked(resp.owner)
}

fn mint_transfer_and_burn(app: &mut App, cw721: Addr, sender: Addr, token_id: String) {
    app.execute_contract(
        sender.clone(),
        cw721.clone(),
        &crate::ExecuteMsg::<Empty, Empty>::Mint {
            token_id: token_id.clone(),
            owner: sender.to_string(),
            token_uri: None,
            extension: Empty::default(),
        },
        &[],
    )
    .unwrap();

    let owner = query_owner(app.wrap(), &cw721, token_id.clone());
    assert_eq!(owner, sender.to_string());

    app.execute_contract(
        sender,
        cw721.clone(),
        &crate::ExecuteMsg::<Empty, Empty>::TransferNft {
            recipient: "burner".to_string(),
            token_id: token_id.clone(),
        },
        &[],
    )
    .unwrap();

    let owner = query_owner(app.wrap(), &cw721, token_id.clone());
    assert_eq!(owner, "burner".to_string());

    app.execute_contract(
        Addr::unchecked("burner"),
        cw721,
        &crate::ExecuteMsg::<Empty, Empty>::Burn { token_id },
        &[],
    )
    .unwrap();
}

/// Instantiates a 0.16 version of this contract and tests that tokens
/// can be minted, transferred, and burnred after migration.
#[test]
fn test_migration_016_to_latest() {
    use cw721_base_016 as v16;
    let mut app = App::default();
    let admin = || Addr::unchecked("admin");

    let code_id_016 = app.store_code(cw721_base_016_contract());
    let code_id_latest = app.store_code(cw721_base_latest_contract());

    let cw721 = app
        .instantiate_contract(
            code_id_016,
            admin(),
            &v16::InstantiateMsg {
                name: "collection".to_string(),
                symbol: "symbol".to_string(),
                minter: admin().into_string(),
            },
            &[],
            "cw721-base",
            Some(admin().into_string()),
        )
        .unwrap();

    mint_transfer_and_burn(&mut app, cw721.clone(), admin(), "1".to_string());

    app.execute(
        admin(),
        WasmMsg::Migrate {
            contract_addr: cw721.to_string(),
            new_code_id: code_id_latest,
            msg: to_json_binary(&Empty::default()).unwrap(),
        }
        .into(),
    )
    .unwrap();

    mint_transfer_and_burn(&mut app, cw721.clone(), admin(), "1".to_string());

    // check new mint query response works.
    let m: MinterResponse = app
        .wrap()
        .query_wasm_smart(&cw721, &crate::QueryMsg::<Empty>::Minter {})
        .unwrap();
    assert_eq!(m.minter, Some(admin().to_string()));

    // check that the new response is backwards compatable when minter
    // is not None.
    let m: v16::MinterResponse = app
        .wrap()
        .query_wasm_smart(&cw721, &crate::QueryMsg::<Empty>::Minter {})
        .unwrap();
    assert_eq!(m.minter, admin().to_string());
}
