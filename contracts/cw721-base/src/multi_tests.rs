use cosmwasm_std::{Addr, Empty, QuerierWrapper};
use cw721::OwnerOfResponse;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

fn cw721_base_latest_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::entry::execute,
        crate::entry::instantiate,
        crate::entry::query,
    )
    .with_migrate(crate::entry::migrate);
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
    assert_eq!(owner.to_string(), sender.to_string());

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
    assert_eq!(owner.to_string(), "burner".to_string());

    app.execute_contract(
        Addr::unchecked("burner"),
        cw721,
        &crate::ExecuteMsg::<Empty, Empty>::Burn { token_id },
        &[],
    )
    .unwrap();
}
