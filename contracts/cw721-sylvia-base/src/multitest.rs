use crate::{
    contract::{multitest_utils::Cw721ContractProxy, InstantiateMsgData, MinterResponse},
    ContractError,
};
use cosmwasm_std::Empty;
use cw721::{
    ContractInfoResponse, NftInfoResponse, NumTokensResponse, OwnerOfResponse, TokensResponse,
};
use cw_ownable::OwnershipError;
use sylvia::multitest::App;

use crate::base::test_utils::Cw721Interface;
use crate::contract::multitest_utils::CodeId;

const CREATOR: &str = "creator";
const RANDOM: &str = "random";

pub struct TestCase<'a> {
    nft_contract: Cw721ContractProxy<'a>,
}

impl TestCase<'_> {
    // Lifetimes with Sylvia are fun. Open to a better way of doing this
    pub fn new<'b>(app: &'b App) -> TestCase<'b> {
        let code_id = CodeId::store_code(app);

        TestCase::<'b> {
            nft_contract: code_id
                .instantiate(InstantiateMsgData {
                    name: "Sylvia".to_string(),
                    symbol: "SYLVIA".to_string(),
                    minter: CREATOR.to_string(),
                })
                .with_label("cw721-sylvia-base contract")
                .call(CREATOR)
                .unwrap(),
        }
    }
}

#[test]
fn test_instantiate() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let nft_contract = code_id
        .instantiate(InstantiateMsgData {
            name: "Sylvia".to_string(),
            symbol: "SYLVIA".to_string(),
            minter: CREATOR.to_string(),
        })
        .with_label("cw721-sylvia-base contract")
        .call(CREATOR)
        .unwrap();

    // Check contract metadata was set
    let contract_info = nft_contract
        .cw721_interface_proxy()
        .contract_info()
        .unwrap();
    assert_eq!(
        ContractInfoResponse {
            name: "Sylvia".to_string(),
            symbol: "SYLVIA".to_string()
        },
        contract_info
    );

    // Query minter
    let minter = nft_contract.minter().unwrap();
    assert_eq!(
        MinterResponse {
            minter: Some(CREATOR.to_string())
        },
        minter
    );

    // Check number of tokens is zero
    let count = nft_contract.cw721_interface_proxy().num_tokens().unwrap();
    assert_eq!(NumTokensResponse { count: 0 }, count);
}

#[test]
fn test_mint() {
    let app = App::default();
    let TestCase { nft_contract } = TestCase::new(&app);

    // Only minter / owner can mint
    let res = nft_contract
        .mint(
            "1".to_string(),
            RANDOM.to_string(),
            Some("https://example.com".to_string()),
        )
        .call(RANDOM)
        .unwrap_err();
    assert_eq!(res, ContractError::Ownership(OwnershipError::NotOwner));

    // Minter / owner can mint an NFT
    nft_contract
        .mint(
            "1".to_string(),
            CREATOR.to_string(),
            Some("https://example.com".to_string()),
        )
        .call(CREATOR)
        .unwrap();

    // Check number of tokens is now one
    let count = nft_contract.cw721_interface_proxy().num_tokens().unwrap();
    assert_eq!(NumTokensResponse { count: 1 }, count);

    // Query token info
    let info = nft_contract
        .cw721_interface_proxy()
        .nft_info("1".to_string())
        .unwrap();
    assert_eq!(
        NftInfoResponse::<Empty> {
            token_uri: Some("https://example.com".to_string()),
            extension: Empty {}
        },
        info
    );

    // Creator mints an NFT for a random address
    nft_contract
        .mint(
            "2".to_string(),
            RANDOM.to_string(),
            Some("https://example.com".to_string()),
        )
        .call(CREATOR)
        .unwrap();

    // Query new tokens by owner
    let tokens = nft_contract
        .cw721_interface_proxy()
        .tokens(CREATOR.to_string(), None, None)
        .unwrap();
    assert_eq!(
        TokensResponse {
            tokens: vec!["1".to_string()]
        },
        tokens
    );

    // Query all tokens
    let all_tokens = nft_contract
        .cw721_interface_proxy()
        .all_tokens(None, None)
        .unwrap();
    assert_eq!(
        TokensResponse {
            tokens: vec!["1".to_string(), "2".to_string()]
        },
        all_tokens
    );
}

#[test]
fn test_burn() {
    let app = App::default();
    let TestCase { nft_contract } = TestCase::new(&app);

    // Mint NFT
    nft_contract
        .mint(
            "1".to_string(),
            CREATOR.to_string(),
            Some("https://example.com".to_string()),
        )
        .call(CREATOR)
        .unwrap();

    // Check number of tokens is now one
    let count = nft_contract.cw721_interface_proxy().num_tokens().unwrap();
    assert_eq!(NumTokensResponse { count: 1 }, count);

    // Non minter / owner cannot burn
    nft_contract
        .cw721_interface_proxy()
        .burn("1".to_string())
        .call(RANDOM)
        .unwrap_err();

    // Only owner can burn
    nft_contract
        .cw721_interface_proxy()
        .burn("1".to_string())
        .call(CREATOR)
        .unwrap();

    // Token count has been reduced
    let count = nft_contract.cw721_interface_proxy().num_tokens().unwrap();
    assert_eq!(NumTokensResponse { count: 0 }, count);
}

#[test]
fn test_transfer() {
    let app = App::default();
    let TestCase { nft_contract } = TestCase::new(&app);

    // Mint NFT
    nft_contract
        .mint(
            "1".to_string(),
            CREATOR.to_string(),
            Some("https://example.com".to_string()),
        )
        .call(CREATOR)
        .unwrap();

    // Owned by creator
    let nft_ownership = nft_contract
        .cw721_interface_proxy()
        .owner_of("1".to_string(), false)
        .unwrap();
    assert_eq!(
        OwnerOfResponse {
            owner: CREATOR.to_string(),
            approvals: vec![]
        },
        nft_ownership
    );

    // Random address can't transfer
    let err = nft_contract
        .cw721_interface_proxy()
        .transfer_nft(RANDOM.to_string(), "1".to_string())
        .call(RANDOM)
        .unwrap_err();
    assert_eq!(ContractError::Ownership(OwnershipError::NotOwner), err);

    // Only owner can transfer
    nft_contract
        .cw721_interface_proxy()
        .transfer_nft(RANDOM.to_string(), "1".to_string())
        .call(CREATOR)
        .unwrap();

    // Ownership has changed
    let nft_ownership = nft_contract
        .cw721_interface_proxy()
        .owner_of("1".to_string(), false)
        .unwrap();
    assert_eq!(
        OwnerOfResponse {
            owner: RANDOM.to_string(),
            approvals: vec![]
        },
        nft_ownership
    );

    // Now that random is the owner, creator can't transfer it back
    // even though they are the minter.
    let err = nft_contract
        .cw721_interface_proxy()
        .transfer_nft(CREATOR.to_string(), "1".to_string())
        .call(CREATOR)
        .unwrap_err();
    assert_eq!(ContractError::Ownership(OwnershipError::NotOwner), err);
}
