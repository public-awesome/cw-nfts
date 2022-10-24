use crate::helpers::chain::Chain;
use cosmwasm_std::Empty;
use test_context::test_context;

#[test_context(Chain)]
#[test]
#[ignore]
fn execute_transfer_nft(chain: &mut Chain) {
    let user_addr = chain.users["user1"].account.address.clone();
    let user_key = chain.users["user1"].key.clone();
    let receiver = chain.users["user2"].account.address.clone();

    let msg = cw721_base::msg::InstantiateMsg {
        name: "token".to_string(),
        symbol: "nonfungible".to_string(),
        minter: user_addr.clone(),
    };

    chain
        .orc
        .instantiate(
            "cw721_base",
            "cw721_base_init",
            &msg,
            &user_key,
            Some(user_addr.clone()),
            vec![],
        )
        .unwrap();

    chain
        .orc
        .execute(
            "cw721_base",
            "cw721_base_mint",
            &cw721_base::ExecuteMsg::<Empty, Empty>::Mint(cw721_base::MintMsg {
                token_id: "1".to_string(),
                owner: user_addr,
                token_uri: None,
                extension: Default::default(),
            }),
            &user_key,
            vec![],
        )
        .unwrap();

    chain
        .orc
        .execute(
            "cw721_base",
            "cw721_base_transfer",
            &cw721_base::ExecuteMsg::<Empty, Empty>::TransferNft {
                recipient: receiver.clone(),
                token_id: "1".to_string(),
            },
            &user_key,
            vec![],
        )
        .unwrap();

    let owner: cw721::OwnerOfResponse = chain
        .orc
        .query(
            "cw721_base",
            &cw721_base::QueryMsg::<Empty>::OwnerOf {
                token_id: "1".to_string(),
                include_expired: None,
            },
        )
        .unwrap()
        .data()
        .unwrap();
    assert_eq!(owner.owner, receiver);
}
