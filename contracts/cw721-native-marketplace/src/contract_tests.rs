#[cfg(test)]
mod tests {
    use crate::helpers::Cw721MarketplaceContract;

    use crate::msg::ExecuteMsg;
    use crate::ContractError;
    use anyhow::{anyhow, Result};
    use derivative::Derivative;

    use cosmwasm_std::{to_binary, Addr, Coin, Empty, QueryRequest, StdError, Uint128, WasmQuery};

    use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};
    use serde::de::DeserializeOwned;

    use cw721_base::helpers::Cw721Contract;
    use cw721_base::MintMsg;
    use cw721_metadata_onchain::{Extension, Metadata};
    use serde::Serialize;

    pub fn contract_marketplace() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::execute::execute,
            crate::execute::instantiate,
            crate::query::query,
        );
        Box::new(contract)
    }
    pub fn contract_cw721() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw721_metadata_onchain::entry::execute,
            cw721_metadata_onchain::entry::instantiate,
            cw721_metadata_onchain::entry::query,
        );
        Box::new(contract)
    }

    const BOB: &str = "bob";

    const MINTER: &str = "minter";
    const ADMIN: &str = "admin";
    const RANDOM: &str = "random";
    const ALLOWED_NATIVE: &str = "ujuno";

    const TOKEN_ID1: &str = "token1";

    fn mock_app() -> App {
        App::new(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(RANDOM),
                    vec![
                        Coin {
                            denom: "ujuno".into(),
                            amount: Uint128::new(50000000),
                        },
                        Coin {
                            denom: "zuhaha".into(),
                            amount: Uint128::new(400),
                        },
                    ],
                )
                .unwrap();

            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(BOB),
                    vec![Coin {
                        denom: "ujuno".into(),
                        amount: Uint128::new(500),
                    }],
                )
                .unwrap();
        })
    }

    #[derive(Derivative)]
    #[derivative(Debug)]
    pub struct Suite {
        /// Application mock
        #[derivative(Debug = "ignore")]
        pub app: App,
        /// Special account
        pub owner: String,

        nft_code_id: u64,
        marketplace_code_id: u64,
    }

    #[allow(dead_code)]
    impl Suite {
        pub fn init() -> Result<Suite> {
            let mut app = mock_app();
            let owner = "owner".to_owned();
            let nft_code_id = app.store_code(contract_cw721());
            let marketplace_code_id = app.store_code(contract_marketplace());

            Ok(Suite {
                app,
                owner,
                nft_code_id,
                marketplace_code_id,
            })
        }

        fn instantiate_nft(&mut self, minter: String) -> Cw721Contract {
            let nft_id = self.app.store_code(contract_cw721());
            let msg = cw721_base::InstantiateMsg {
                name: "Strange Clan".to_string(),
                symbol: "STR".to_string(),
                minter: minter.clone(),
            };
            Cw721Contract(
                self.app
                    .instantiate_contract(nft_id, Addr::unchecked(minter), &msg, &[], "flex", None)
                    .unwrap(),
            )
        }

        fn instantiate_marketplace(
            &mut self,
            nft_addr: String,
            allowed_native: String,
        ) -> Cw721MarketplaceContract {
            let marketplace_id = self.app.store_code(contract_marketplace());
            let msg = crate::msg::InstantiateMsg {
                admin: String::from(ADMIN),
                nft_addr,
                allowed_native,
            };
            Cw721MarketplaceContract(
                self.app
                    .instantiate_contract(
                        marketplace_id,
                        Addr::unchecked(ADMIN),
                        &msg,
                        &[],
                        "flex",
                        None,
                    )
                    .unwrap(),
            )
        }

        fn proper_instantiate(&mut self) -> (Cw721Contract, Cw721MarketplaceContract) {
            // setup nft contract
            let nft = self.instantiate_nft(String::from(MINTER));
            let mint_msg1 = cw721_metadata_onchain::ExecuteMsg::Mint(MintMsg {
                token_id: TOKEN_ID1.to_string(),
                owner: BOB.to_string(),
                token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
                extension: Some(Metadata {
                    description: Some("Spaceship with Warp Drive".into()),
                    name: Some("Starship USS Enterprise".to_string()),
                    ..Metadata::default()
                }),
            });
            let cosmos_msg = nft.call(mint_msg1).unwrap();
            self.app
                .execute(Addr::unchecked(MINTER), cosmos_msg)
                .unwrap();
            let marketplace =
                self.instantiate_marketplace(nft.addr().into(), String::from(ALLOWED_NATIVE));
            (nft, marketplace)
        }

        pub fn execute<M>(
            &mut self,
            sender: Addr,
            contract_addr: Addr,
            msg: ExecuteMsg,
            _funds: Vec<Coin>,
        ) -> Result<AppResponse>
        where
            M: Serialize + DeserializeOwned,
        {
            self.app
                .execute_contract(sender, contract_addr, &msg, &[])
                .map_err(|err| anyhow!(err))
        }

        pub fn query<M>(&self, target_contract: Addr, msg: M) -> Result<M, StdError>
        where
            M: Serialize + DeserializeOwned,
        {
            self.app.wrap().query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: target_contract.to_string(),
                msg: to_binary(&msg).unwrap(),
            }))
        }
    }

    #[test]
    fn test_register_tokens() {
        let mut suite = Suite::init().unwrap();
        let (_nft_contract, marketplace_contract) = suite.proper_instantiate();

        // empty tokens throw error
        let msg = marketplace_contract
            .call(ExecuteMsg::ListTokens { tokens: vec![] }, vec![])
            .unwrap();
        let res = suite.app.execute(Addr::unchecked(ADMIN), msg).unwrap_err();
        assert_eq!(ContractError::WrongInput {}, res.downcast().unwrap());

        // only admin can register tokens
        let token = crate::state::Token {
            owner: Addr::unchecked(BOB),
            id: TOKEN_ID1.into(),
            price: Default::default(),
            on_sale: true,
        };
        let msg = marketplace_contract
            .call(
                ExecuteMsg::ListTokens {
                    tokens: vec![token],
                },
                vec![],
            )
            .unwrap();
        let res = suite
            .app
            .execute(Addr::unchecked(RANDOM), msg.clone())
            .unwrap_err();
        assert_eq!(ContractError::Unauthorized {}, res.downcast().unwrap());

        // admin can register token
        suite.app.execute(Addr::unchecked(ADMIN), msg).unwrap();
    }

    #[test]
    fn test_list_tokens() {
        let mut suite = Suite::init().unwrap();
        let (nft_contract, marketplace_contract) = suite.proper_instantiate();

        // empty tokens throw error
        let msg = marketplace_contract
            .call(ExecuteMsg::ListTokens { tokens: vec![] }, vec![])
            .unwrap();
        let res = suite.app.execute(Addr::unchecked(ADMIN), msg).unwrap_err();
        assert_eq!(ContractError::WrongInput {}, res.downcast().unwrap());

        let token = crate::state::Token {
            owner: Addr::unchecked(BOB),
            id: String::from(TOKEN_ID1),
            price: Default::default(),
            on_sale: true,
        };
        let msg = marketplace_contract
            .call(
                ExecuteMsg::ListTokens {
                    tokens: vec![token.clone()],
                },
                vec![],
            )
            .unwrap();

        // register token
        suite
            .app
            .execute(Addr::unchecked(ADMIN), msg.clone())
            .unwrap();

        // only token owner can list
        let res = suite
            .app
            .execute(Addr::unchecked(RANDOM), msg.clone())
            .unwrap_err();
        assert_eq!(ContractError::Unauthorized {}, res.downcast().unwrap());

        // non approved tokens are not accepted
        let res = suite.app.execute(Addr::unchecked(BOB), msg).unwrap_err();
        assert_eq!(ContractError::NotApproved {}, res.downcast().unwrap());

        // marketplace contract is not spender
        let exec_msg: cw721_base::ExecuteMsg<Extension> = cw721_base::ExecuteMsg::Approve {
            spender: RANDOM.into(),
            token_id: TOKEN_ID1.into(),
            expires: None,
        };
        let msg = nft_contract.call(exec_msg).unwrap();
        suite.app.execute(Addr::unchecked(BOB), msg).unwrap();

        let msg = marketplace_contract
            .call(
                ExecuteMsg::ListTokens {
                    tokens: vec![token.clone()],
                },
                vec![],
            )
            .unwrap();
        let res = suite.app.execute(Addr::unchecked(BOB), msg).unwrap_err();
        assert_eq!(ContractError::NotApproved {}, res.downcast().unwrap());

        // marketplace contract is spender, happy path
        let exec_msg = cw721_metadata_onchain::ExecuteMsg::Approve {
            spender: marketplace_contract.addr().into(),
            token_id: token.id.clone(),
            expires: None,
        };
        let msg = nft_contract.call(exec_msg).unwrap();
        suite.app.execute(Addr::unchecked(BOB), msg).unwrap();

        let msg = marketplace_contract
            .call(
                ExecuteMsg::ListTokens {
                    tokens: vec![token.clone()],
                },
                vec![],
            )
            .unwrap();
        suite.app.execute(Addr::unchecked(BOB), msg).unwrap();

        let t = marketplace_contract.token(&suite.app, TOKEN_ID1).unwrap();
        assert_eq!(t.token, token)
    }

    #[test]
    fn test_delist_token() {
        let mut suite = Suite::init().unwrap();
        let (_nft_contract, marketplace_contract) = suite.proper_instantiate();

        // list token
        let token = crate::state::Token {
            owner: Addr::unchecked(BOB),
            id: TOKEN_ID1.into(),
            price: Default::default(),
            on_sale: true,
        };
        let msg = marketplace_contract
            .call(
                ExecuteMsg::ListTokens {
                    tokens: vec![token.clone()],
                },
                vec![],
            )
            .unwrap();
        suite.app.execute(Addr::unchecked(ADMIN), msg).unwrap();

        let msg = marketplace_contract
            .call(
                ExecuteMsg::DelistTokens {
                    tokens: vec![token.id.clone()],
                },
                vec![],
            )
            .unwrap();

        // only owner can delist
        let res = suite
            .app
            .execute(Addr::unchecked(RANDOM), msg.clone())
            .unwrap_err();
        assert_eq!(ContractError::Unauthorized {}, res.downcast().unwrap());

        // happy path
        suite.app.execute(Addr::unchecked(BOB), msg).unwrap();

        let t = marketplace_contract.token(&suite.app, TOKEN_ID1).unwrap();
        assert_eq!(
            t.token,
            crate::state::Token {
                owner: token.owner,
                id: token.id,
                price: token.price,
                on_sale: false
            }
        )
    }

    #[test]
    fn test_change_price() {
        let mut suite = Suite::init().unwrap();
        let (_nft_contract, marketplace_contract) = suite.proper_instantiate();

        let token = crate::state::Token {
            owner: Addr::unchecked(BOB),
            id: TOKEN_ID1.into(),
            price: Uint128::new(1),
            on_sale: true,
        };
        let msg = marketplace_contract
            .call(
                ExecuteMsg::ListTokens {
                    tokens: vec![token.clone()],
                },
                vec![],
            )
            .unwrap();
        suite.app.execute(Addr::unchecked(ADMIN), msg).unwrap();

        let msg = marketplace_contract
            .call(
                ExecuteMsg::UpdatePrice {
                    token: TOKEN_ID1.into(),
                    price: Uint128::new(100),
                },
                vec![],
            )
            .unwrap();

        // only owner can update price
        let res = suite
            .app
            .execute(Addr::unchecked(RANDOM), msg.clone())
            .unwrap_err();
        assert_eq!(ContractError::Unauthorized {}, res.downcast().unwrap());

        // happy path
        suite.app.execute(Addr::unchecked(BOB), msg).unwrap();

        let t = marketplace_contract.token(&suite.app, TOKEN_ID1).unwrap();
        assert_eq!(
            t.token,
            crate::state::Token {
                owner: token.owner,
                id: token.id,
                price: Uint128::new(100),
                on_sale: true
            }
        )
    }
    #[test]

    fn test_delist_and_register() {
        let mut suite = Suite::init().unwrap();
        let (nft_contract, marketplace_contract) = suite.proper_instantiate();

        // list token
        let token = crate::state::Token {
            owner: Addr::unchecked(BOB),
            id: TOKEN_ID1.into(),
            price: Default::default(),
            on_sale: true,
        };

        // owner approves
        let exec_msg = cw721_metadata_onchain::ExecuteMsg::Approve {
            spender: marketplace_contract.addr().into(),
            token_id: token.id.clone(),
            expires: None,
        };
        let msg = nft_contract.call(exec_msg).unwrap();
        suite.app.execute(Addr::unchecked(BOB), msg).unwrap();

        // admin lists
        let msg = marketplace_contract
            .call(
                ExecuteMsg::ListTokens {
                    tokens: vec![token.clone()],
                },
                vec![],
            )
            .unwrap();
        suite.app.execute(Addr::unchecked(ADMIN), msg).unwrap();
        // owner delists
        let msg = marketplace_contract
            .call(
                ExecuteMsg::DelistTokens {
                    tokens: vec![token.id.clone()],
                },
                vec![],
            )
            .unwrap();
        suite.app.execute(Addr::unchecked(BOB), msg).unwrap();

        // owner lists
        let msg = marketplace_contract
            .call(
                ExecuteMsg::ListTokens {
                    tokens: vec![token],
                },
                vec![],
            )
            .unwrap();
        suite.app.execute(Addr::unchecked(BOB), msg).unwrap();
    }

    #[test]
    fn test_buy() {
        let mut suite = Suite::init().unwrap();
        let (nft_contract, marketplace_contract) = suite.proper_instantiate();

        let token = crate::state::Token {
            owner: Addr::unchecked(BOB),
            id: TOKEN_ID1.into(),
            price: Uint128::new(1),
            on_sale: true,
        };
        let msg = marketplace_contract
            .call(
                ExecuteMsg::ListTokens {
                    tokens: vec![token.clone()],
                },
                vec![],
            )
            .unwrap();
        suite.app.execute(Addr::unchecked(ADMIN), msg).unwrap();

        // approve marketplace
        let exec_msg = cw721_metadata_onchain::ExecuteMsg::Approve {
            spender: marketplace_contract.addr().into(),
            token_id: token.id,
            expires: None,
        };
        let msg = nft_contract.call(exec_msg).unwrap();
        suite.app.execute(Addr::unchecked(BOB), msg).unwrap();

        // no tokens
        let msg = marketplace_contract
            .call(
                ExecuteMsg::Buy {
                    recipient: None,
                    token_id: TOKEN_ID1.into(),
                },
                vec![],
            )
            .unwrap();
        let res = suite.app.execute(Addr::unchecked(RANDOM), msg).unwrap_err();
        assert_eq!(
            ContractError::SendSingleNativeToken {},
            res.downcast().unwrap()
        );

        // multiple tokens
        let msg = marketplace_contract
            .call(
                ExecuteMsg::Buy {
                    recipient: None,
                    token_id: TOKEN_ID1.into(),
                },
                vec![
                    Coin {
                        denom: "ujuno".into(),
                        amount: Uint128::new(2),
                    },
                    Coin {
                        denom: "zuhaha".into(),
                        amount: Uint128::new(2),
                    },
                ],
            )
            .unwrap();
        let res = suite.app.execute(Addr::unchecked(RANDOM), msg).unwrap_err();
        assert_eq!(
            ContractError::SendSingleNativeToken {},
            res.downcast().unwrap()
        );

        // disallowed native token
        let msg = marketplace_contract
            .call(
                ExecuteMsg::Buy {
                    recipient: None,
                    token_id: TOKEN_ID1.into(),
                },
                vec![Coin {
                    denom: "zuhaha".into(),
                    amount: Default::default(),
                }],
            )
            .unwrap();
        let res = suite.app.execute(Addr::unchecked(RANDOM), msg).unwrap_err();
        assert_eq!(
            ContractError::NativeDenomNotAllowed {
                denom: "zuhaha".into()
            },
            res.downcast().unwrap()
        );

        // wrong coin amount
        let msg = marketplace_contract
            .call(
                ExecuteMsg::Buy {
                    recipient: None,
                    token_id: TOKEN_ID1.into(),
                },
                vec![Coin {
                    denom: ALLOWED_NATIVE.into(),
                    amount: Default::default(),
                }],
            )
            .unwrap();
        let res = suite.app.execute(Addr::unchecked(RANDOM), msg).unwrap_err();
        assert_eq!(
            ContractError::InsufficientBalance {
                need: Uint128::new(1),
                sent: Default::default()
            },
            res.downcast().unwrap()
        );

        // wrong coin amount
        let msg = marketplace_contract
            .call(
                ExecuteMsg::Buy {
                    recipient: None,
                    token_id: TOKEN_ID1.into(),
                },
                vec![Coin {
                    denom: ALLOWED_NATIVE.into(),
                    amount: Uint128::new(10000),
                }],
            )
            .unwrap();
        let res = suite.app.execute(Addr::unchecked(RANDOM), msg).unwrap_err();
        assert_eq!(
            ContractError::InsufficientBalance {
                need: Uint128::new(1),
                sent: Uint128::new(10000)
            },
            res.downcast().unwrap()
        );

        // happy path
        let msg = marketplace_contract
            .call(
                ExecuteMsg::Buy {
                    recipient: None,
                    token_id: TOKEN_ID1.into(),
                },
                vec![Coin {
                    denom: ALLOWED_NATIVE.into(),
                    amount: Uint128::new(1),
                }],
            )
            .unwrap();
        suite.app.execute(Addr::unchecked(RANDOM), msg).unwrap();

        // marketplace owner updated
        let t = marketplace_contract.token(&suite.app, TOKEN_ID1).unwrap();
        assert_eq!(t.token.owner.into_string(), String::from(RANDOM));
    }
}
