use crate::contract::{test_utils::Cw721Interface, InstantiateMsgData};
use cw721::ContractInfoResponse;
use sylvia::multitest::App;

use crate::contract::multitest_utils::CodeId;

const CREATOR: &str = "creator";

#[test]
fn test_instantiate() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let nft = code_id
        .instantiate(InstantiateMsgData {
            name: "Sylvia".to_string(),
            symbol: "SYLVIA".to_string(),
            minter: CREATOR.to_string(),
        })
        .with_label("cw721-sylvia-base contract")
        .call("addr0001")
        .unwrap();

    // Check contract metadata was set
    let res = nft.cw721_interface_proxy().contract_info().unwrap();
    assert_eq!(
        ContractInfoResponse {
            name: "Sylvia".to_string(),
            symbol: "SYLVIA".to_string()
        },
        res
    );
}
