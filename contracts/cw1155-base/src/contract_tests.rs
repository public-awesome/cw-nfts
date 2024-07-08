#[cfg(test)]
mod tests {
    use crate::{Cw1155BaseContract, Cw1155BaseExecuteMsg, Cw1155BaseQueryMsg};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{
        from_json, to_json_binary, Addr, Binary, Empty, Event, OverflowError, Response, StdError,
        Uint128,
    };
    use cw1155::error::Cw1155ContractError;
    use cw1155::execute::Cw1155Execute;
    use cw1155::msg::{
        ApprovedForAllResponse, Balance, BalanceResponse, BalancesResponse, Cw1155InstantiateMsg,
        Cw1155MintMsg, Cw1155QueryMsg, NumTokensResponse, OwnerToken, TokenAmount,
        TokenInfoResponse,
    };
    use cw1155::query::Cw1155Query;
    use cw1155::receiver::Cw1155BatchReceiveMsg;
    use cw721::msg::TokensResponse;
    use cw721::Approval;
    use cw_ownable::OwnershipError;
    use cw_utils::Expiration;

    #[test]
    fn check_transfers() {
        let contract = Cw1155BaseContract::default();
        // A long test case that try to cover as many cases as possible.
        // Summary of what it does:
        // - try mint without permission, fail
        // - mint with permission, success
        // - query balance of recipient, success
        // - try transfer without approval, fail
        // - approve
        // - transfer again, success
        // - query balance of transfer participants
        // - try batch transfer without approval, fail
        // - approve and try batch transfer again, success
        // - batch query balances
        // - user1 revoke approval to minter
        // - query approval status
        // - minter try to transfer, fail
        // - user1 burn token1
        // - user1 batch burn token2 and token3
        let token1 = "token1".to_owned();
        let token2 = "token2".to_owned();
        let token3 = "token3".to_owned();
        let minter = String::from("minter");
        let user1 = String::from("user1");
        let user2 = String::from("user2");

        let mut deps = mock_dependencies();
        let msg = Cw1155InstantiateMsg {
            name: "name".to_string(),
            symbol: "symbol".to_string(),
            minter: Some(minter.to_string()),
        };
        let res = contract
            .instantiate(
                deps.as_mut(),
                mock_env(),
                mock_info("operator", &[]),
                msg,
                "contract_name",
                "contract_version",
            )
            .unwrap();
        assert_eq!(0, res.messages.len());

        // invalid mint, user1 don't mint permission
        let mint_msg = Cw1155BaseExecuteMsg::Mint {
            recipient: user1.to_string(),
            msg: Cw1155MintMsg {
                token_id: token1.to_string(),
                amount: 1u64.into(),
                token_uri: None,
                extension: None,
            },
        };
        assert!(matches!(
            contract.execute(
                deps.as_mut(),
                mock_env(),
                mock_info(user1.as_ref(), &[]),
                mint_msg.clone(),
            ),
            Err(Cw1155ContractError::Ownership(OwnershipError::NotOwner))
        ));

        // valid mint
        assert_eq!(
            contract
                .execute(
                    deps.as_mut(),
                    mock_env(),
                    mock_info(minter.as_ref(), &[]),
                    mint_msg,
                )
                .unwrap(),
            Response::new().add_event(Event::new("mint_single").add_attributes(vec![
                ("recipient", user1.as_str()),
                ("token_id", token1.as_str()),
                ("amount", "1"),
            ]))
        );

        // query balance
        assert_eq!(
            to_json_binary(&BalanceResponse {
                balance: 1u64.into()
            }),
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155BaseQueryMsg::BalanceOf(OwnerToken {
                    owner: user1.clone(),
                    token_id: token1.clone(),
                })
            ),
        );

        let transfer_msg = Cw1155BaseExecuteMsg::Send {
            from: Some(user1.to_string()),
            to: user2.clone(),
            token_id: token1.clone(),
            amount: 1u64.into(),
            msg: None,
        };

        // not approved yet
        assert!(matches!(
            contract.execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                transfer_msg.clone(),
            ),
            Err(Cw1155ContractError::Unauthorized {})
        ));

        // approve
        contract
            .execute(
                deps.as_mut(),
                mock_env(),
                mock_info(user1.as_ref(), &[]),
                Cw1155BaseExecuteMsg::ApproveAll {
                    operator: minter.clone(),
                    expires: None,
                },
            )
            .unwrap();

        // transfer
        assert_eq!(
            contract
                .execute(
                    deps.as_mut(),
                    mock_env(),
                    mock_info(minter.as_ref(), &[]),
                    transfer_msg,
                )
                .unwrap(),
            Response::new().add_event(Event::new("transfer_single").add_attributes(vec![
                ("sender", user1.as_str()),
                ("recipient", user2.as_str()),
                ("token_id", token1.as_str()),
                ("amount", "1"),
            ]))
        );

        // query balance
        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155BaseQueryMsg::BalanceOf(OwnerToken {
                    owner: user2.clone(),
                    token_id: token1.clone(),
                })
            ),
            to_json_binary(&BalanceResponse {
                balance: 1u64.into()
            }),
        );
        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155BaseQueryMsg::BalanceOf(OwnerToken {
                    owner: user1.clone(),
                    token_id: token1.clone(),
                })
            ),
            to_json_binary(&BalanceResponse {
                balance: 0u64.into()
            }),
        );

        // mint token2 and token3
        contract
            .execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                Cw1155BaseExecuteMsg::Mint {
                    recipient: user2.clone(),
                    msg: Cw1155MintMsg {
                        token_id: token2.clone(),
                        amount: 1u64.into(),
                        token_uri: None,
                        extension: None,
                    },
                },
            )
            .unwrap();

        contract
            .execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                Cw1155BaseExecuteMsg::Mint {
                    recipient: user2.clone(),
                    msg: Cw1155MintMsg {
                        token_id: token3.clone(),
                        amount: 1u64.into(),
                        token_uri: None,
                        extension: None,
                    },
                },
            )
            .unwrap();

        // invalid batch transfer, (user2 not approved yet)
        let batch_transfer_msg = Cw1155BaseExecuteMsg::SendBatch {
            from: Some(user2.clone()),
            to: user1.clone(),
            batch: vec![
                TokenAmount {
                    token_id: token1.to_string(),
                    amount: 1u64.into(),
                },
                TokenAmount {
                    token_id: token2.to_string(),
                    amount: 1u64.into(),
                },
                TokenAmount {
                    token_id: token3.to_string(),
                    amount: 1u64.into(),
                },
            ],
            msg: None,
        };
        assert!(matches!(
            contract.execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                batch_transfer_msg.clone(),
            ),
            Err(Cw1155ContractError::Unauthorized {}),
        ));

        // user2 approve
        contract
            .execute(
                deps.as_mut(),
                mock_env(),
                mock_info(user2.as_ref(), &[]),
                Cw1155BaseExecuteMsg::ApproveAll {
                    operator: minter.clone(),
                    expires: None,
                },
            )
            .unwrap();

        // valid batch transfer
        assert_eq!(
            contract
                .execute(
                    deps.as_mut(),
                    mock_env(),
                    mock_info(minter.as_ref(), &[]),
                    batch_transfer_msg,
                )
                .unwrap(),
            Response::new().add_event(Event::new("transfer_batch").add_attributes(vec![
                ("sender", user2.as_str()),
                ("recipient", user1.as_str()),
                ("token_ids", &format!("{},{},{}", token1, token2, token3)),
                ("amounts", "1,1,1"),
            ]))
        );

        // batch query
        assert_eq!(
            from_json(
                contract
                    .query(
                        deps.as_ref(),
                        mock_env(),
                        Cw1155BaseQueryMsg::BalanceOfBatch(vec![
                            OwnerToken {
                                owner: user1.clone(),
                                token_id: token1.clone(),
                            },
                            OwnerToken {
                                owner: user1.clone(),
                                token_id: token2.clone(),
                            },
                            OwnerToken {
                                owner: user1.clone(),
                                token_id: token3.clone(),
                            }
                        ]),
                    )
                    .unwrap()
            ),
            Ok(BalancesResponse {
                balances: vec![
                    Balance {
                        token_id: token1.to_string(),
                        owner: Addr::unchecked(user1.to_string()),
                        amount: Uint128::one(),
                    },
                    Balance {
                        token_id: token2.to_string(),
                        owner: Addr::unchecked(user1.to_string()),
                        amount: Uint128::one(),
                    },
                    Balance {
                        token_id: token3.to_string(),
                        owner: Addr::unchecked(user1.to_string()),
                        amount: Uint128::one(),
                    }
                ]
            }),
        );

        // user1 revoke approval
        contract
            .execute(
                deps.as_mut(),
                mock_env(),
                mock_info(user1.as_ref(), &[]),
                Cw1155BaseExecuteMsg::RevokeAll {
                    operator: minter.clone(),
                },
            )
            .unwrap();

        // query approval status
        let approvals: ApprovedForAllResponse = from_json(
            contract
                .query(
                    deps.as_ref(),
                    mock_env(),
                    Cw1155QueryMsg::ApprovalsForAll {
                        owner: user1.clone(),
                        include_expired: None,
                        start_after: None,
                        limit: None,
                    },
                )
                .unwrap(),
        )
        .unwrap();
        assert!(
            approvals.operators.is_empty()
                || !approvals
                    .operators
                    .iter()
                    .all(|approval| approval.spender == minter)
        );

        // transfer without approval
        assert!(matches!(
            contract.execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                Cw1155BaseExecuteMsg::Send {
                    from: Some(user1.clone()),
                    to: user2,
                    token_id: token1.clone(),
                    amount: 1u64.into(),
                    msg: None,
                },
            ),
            Err(Cw1155ContractError::Unauthorized {})
        ));

        // burn token1
        assert_eq!(
            contract
                .execute(
                    deps.as_mut(),
                    mock_env(),
                    mock_info(user1.as_ref(), &[]),
                    Cw1155BaseExecuteMsg::Burn {
                        from: None,
                        token_id: token1.clone(),
                        amount: 1u64.into(),
                    },
                )
                .unwrap(),
            Response::new().add_event(Event::new("burn_single").add_attributes(vec![
                ("sender", user1.as_str()),
                ("token_id", &token1),
                ("amount", "1"),
            ]))
        );

        // burn them all
        assert_eq!(
            contract
                .execute(
                    deps.as_mut(),
                    mock_env(),
                    mock_info(user1.as_ref(), &[]),
                    Cw1155BaseExecuteMsg::BurnBatch {
                        from: None,
                        batch: vec![
                            TokenAmount {
                                token_id: token2.to_string(),
                                amount: 1u64.into(),
                            },
                            TokenAmount {
                                token_id: token3.to_string(),
                                amount: 1u64.into(),
                            },
                        ],
                    },
                )
                .unwrap(),
            Response::new().add_event(Event::new("burn_batch").add_attributes(vec![
                ("sender", user1.as_str()),
                ("token_ids", &format!("{},{}", token2, token3)),
                ("amounts", "1,1"),
            ]))
        );
    }

    #[test]
    fn check_send_contract() {
        let contract = Cw1155BaseContract::default();
        let receiver = String::from("receive_contract");
        let minter = String::from("minter");
        let user1 = String::from("user1");
        let token2 = "token2".to_owned();
        let operator_info = mock_info("operator", &[]);
        let dummy_msg = Binary::default();

        let mut deps = mock_dependencies();
        let msg = Cw1155InstantiateMsg {
            name: "name".to_string(),
            symbol: "symbol".to_string(),
            minter: Some(minter.to_string()),
        };
        let res = contract
            .instantiate(
                deps.as_mut(),
                mock_env(),
                operator_info.clone(),
                msg,
                "contract_name",
                "contract_version",
            )
            .unwrap();
        assert_eq!(0, res.messages.len());

        contract
            .execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                Cw1155BaseExecuteMsg::Mint {
                    recipient: user1.clone(),
                    msg: Cw1155MintMsg {
                        token_id: token2.clone(),
                        amount: 1u64.into(),
                        token_uri: None,
                        extension: None,
                    },
                },
            )
            .unwrap();

        // BatchSendFrom
        assert_eq!(
            contract
                .execute(
                    deps.as_mut(),
                    mock_env(),
                    mock_info(user1.as_ref(), &[]),
                    Cw1155BaseExecuteMsg::SendBatch {
                        from: Some(user1.clone()),
                        to: receiver.clone(),
                        batch: vec![TokenAmount {
                            token_id: token2.to_string(),
                            amount: 1u64.into(),
                        },],
                        msg: Some(dummy_msg.clone()),
                    },
                )
                .unwrap(),
            Response::new()
                .add_message(
                    Cw1155BatchReceiveMsg {
                        operator: user1.clone(),
                        from: Some(user1.clone()),
                        batch: vec![TokenAmount {
                            token_id: token2.to_string(),
                            amount: 1u64.into(),
                        }],
                        msg: dummy_msg,
                    }
                    .into_cosmos_msg(&operator_info, receiver.clone())
                    .unwrap()
                )
                .add_event(Event::new("transfer_single").add_attributes(vec![
                    ("sender", user1.as_str()),
                    ("recipient", receiver.as_str()),
                    ("token_id", token2.as_str()),
                    ("amount", "1"),
                ]))
        );
    }

    #[test]
    fn check_queries() {
        let contract = Cw1155BaseContract::default();
        // mint multiple types of tokens, and query them
        // grant approval to multiple operators, and query them
        let tokens = (0..10).map(|i| format!("token{}", i)).collect::<Vec<_>>();
        let users = (0..10).map(|i| format!("user{}", i)).collect::<Vec<_>>();
        let minter = String::from("minter");

        let mut deps = mock_dependencies();
        let msg = Cw1155InstantiateMsg {
            name: "name".to_string(),
            symbol: "symbol".to_string(),
            minter: Some(minter.to_string()),
        };
        let res = contract
            .instantiate(
                deps.as_mut(),
                mock_env(),
                mock_info("operator", &[]),
                msg,
                "contract_name",
                "contract_version",
            )
            .unwrap();
        assert_eq!(0, res.messages.len());

        for token_id in tokens.clone() {
            contract
                .execute(
                    deps.as_mut(),
                    mock_env(),
                    mock_info(minter.as_ref(), &[]),
                    Cw1155BaseExecuteMsg::Mint {
                        recipient: users[0].clone(),
                        msg: Cw1155MintMsg {
                            token_id: token_id.clone(),
                            amount: 1u64.into(),
                            token_uri: None,
                            extension: None,
                        },
                    },
                )
                .unwrap();
        }

        for user in users[1..].iter() {
            contract
                .execute(
                    deps.as_mut(),
                    mock_env(),
                    mock_info(minter.as_ref(), &[]),
                    Cw1155BaseExecuteMsg::Mint {
                        recipient: user.clone(),
                        msg: Cw1155MintMsg {
                            token_id: tokens[9].clone(),
                            amount: 1u64.into(),
                            token_uri: None,
                            extension: None,
                        },
                    },
                )
                .unwrap();
        }

        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::NumTokens {
                    token_id: Some(tokens[0].clone()),
                },
            ),
            to_json_binary(&NumTokensResponse {
                count: Uint128::new(1),
            })
        );

        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::NumTokens {
                    token_id: Some(tokens[0].clone()),
                },
            ),
            to_json_binary(&NumTokensResponse {
                count: Uint128::new(1),
            })
        );

        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::AllBalances {
                    token_id: tokens[9].clone(),
                    start_after: None,
                    limit: Some(5),
                },
            ),
            to_json_binary(&BalancesResponse {
                balances: users[..5]
                    .iter()
                    .map(|user| {
                        Balance {
                            owner: Addr::unchecked(user),
                            amount: Uint128::new(1),
                            token_id: tokens[9].clone(),
                        }
                    })
                    .collect(),
            })
        );

        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::AllBalances {
                    token_id: tokens[9].clone(),
                    start_after: Some("user5".to_owned()),
                    limit: Some(5),
                },
            ),
            to_json_binary(&BalancesResponse {
                balances: users[6..]
                    .iter()
                    .map(|user| {
                        Balance {
                            owner: Addr::unchecked(user),
                            amount: Uint128::new(1),
                            token_id: tokens[9].clone(),
                        }
                    })
                    .collect(),
            })
        );

        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::Tokens {
                    owner: users[0].clone(),
                    start_after: None,
                    limit: Some(5),
                },
            ),
            to_json_binary(&TokensResponse {
                tokens: tokens[..5].to_owned()
            })
        );

        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::Tokens {
                    owner: users[0].clone(),
                    start_after: Some("token5".to_owned()),
                    limit: Some(5),
                },
            ),
            to_json_binary(&TokensResponse {
                tokens: tokens[6..].to_owned()
            })
        );

        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::AllTokens {
                    start_after: Some("token5".to_owned()),
                    limit: Some(5),
                },
            ),
            to_json_binary(&TokensResponse {
                tokens: tokens[6..].to_owned()
            })
        );

        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::TokenInfo {
                    token_id: "token5".to_owned()
                },
            ),
            to_json_binary(&TokenInfoResponse::<Option<Empty>> {
                token_uri: None,
                extension: None,
            }),
        );

        for user in users[1..].iter() {
            contract
                .execute(
                    deps.as_mut(),
                    mock_env(),
                    mock_info(users[0].as_ref(), &[]),
                    Cw1155BaseExecuteMsg::ApproveAll {
                        operator: user.clone(),
                        expires: None,
                    },
                )
                .unwrap();
        }

        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::ApprovalsForAll {
                    owner: users[0].clone(),
                    include_expired: None,
                    start_after: Some(String::from("user2")),
                    limit: Some(1),
                },
            ),
            to_json_binary(&ApprovedForAllResponse {
                operators: vec![Approval {
                    spender: Addr::unchecked(&users[3]),
                    expires: Expiration::Never {},
                }],
            })
        );
    }

    #[test]
    fn approval_expires() {
        let contract = Cw1155BaseContract::default();
        let mut deps = mock_dependencies();
        let token1 = "token1".to_owned();
        let minter = String::from("minter");
        let user1 = String::from("user1");
        let user2 = String::from("user2");

        let env = {
            let mut env = mock_env();
            env.block.height = 10;
            env
        };

        let msg = Cw1155InstantiateMsg {
            name: "name".to_string(),
            symbol: "symbol".to_string(),
            minter: Some(minter.to_string()),
        };
        let res = contract
            .instantiate(
                deps.as_mut(),
                env.clone(),
                mock_info("operator", &[]),
                msg,
                "contract_name",
                "contract_version",
            )
            .unwrap();
        assert_eq!(0, res.messages.len());

        contract
            .execute(
                deps.as_mut(),
                env.clone(),
                mock_info(minter.as_ref(), &[]),
                Cw1155BaseExecuteMsg::Mint {
                    recipient: user1.clone(),
                    msg: Cw1155MintMsg {
                        token_id: token1,
                        amount: 1u64.into(),
                        token_uri: None,
                        extension: None,
                    },
                },
            )
            .unwrap();

        // invalid expires should be rejected
        assert!(contract
            .execute(
                deps.as_mut(),
                env.clone(),
                mock_info(user1.as_ref(), &[]),
                Cw1155BaseExecuteMsg::ApproveAll {
                    operator: user2.clone(),
                    expires: Some(Expiration::AtHeight(5)),
                },
            )
            .is_err());

        contract
            .execute(
                deps.as_mut(),
                env,
                mock_info(user1.as_ref(), &[]),
                Cw1155BaseExecuteMsg::ApproveAll {
                    operator: user2.clone(),
                    expires: Some(Expiration::AtHeight(100)),
                },
            )
            .unwrap();

        let approvals: ApprovedForAllResponse = from_json(
            contract
                .query(
                    deps.as_ref(),
                    mock_env(),
                    Cw1155QueryMsg::ApprovalsForAll {
                        owner: user1.to_string(),
                        include_expired: None,
                        start_after: None,
                        limit: None,
                    },
                )
                .unwrap(),
        )
        .unwrap();
        assert!(approvals
            .operators
            .iter()
            .all(|approval| approval.spender == user2));

        let env = {
            let mut env = mock_env();
            env.block.height = 100;
            env
        };

        let approvals: ApprovedForAllResponse = from_json(
            contract
                .query(
                    deps.as_ref(),
                    env,
                    Cw1155QueryMsg::ApprovalsForAll {
                        owner: user1,
                        include_expired: None,
                        start_after: None,
                        limit: None,
                    },
                )
                .unwrap(),
        )
        .unwrap();
        assert!(
            approvals.operators.is_empty()
                || !approvals
                    .operators
                    .iter()
                    .all(|approval| approval.spender == user2)
        );
    }

    #[test]
    fn mint_overflow() {
        let contract = Cw1155BaseContract::default();
        let mut deps = mock_dependencies();
        let token1 = "token1".to_owned();
        let token2 = "token2".to_owned();
        let minter = String::from("minter");
        let user1 = String::from("user1");

        let env = mock_env();
        let msg = Cw1155InstantiateMsg {
            name: "name".to_string(),
            symbol: "symbol".to_string(),
            minter: Some(minter.to_string()),
        };
        let res = contract
            .instantiate(
                deps.as_mut(),
                env.clone(),
                mock_info("operator", &[]),
                msg,
                "contract_name",
                "contract_version",
            )
            .unwrap();
        assert_eq!(0, res.messages.len());

        // minting up to max u128 should pass
        let res = contract.execute(
            deps.as_mut(),
            env.clone(),
            mock_info(minter.as_ref(), &[]),
            Cw1155BaseExecuteMsg::Mint {
                recipient: user1.clone(),
                msg: Cw1155MintMsg {
                    token_id: token1.clone(),
                    amount: u128::MAX.into(),
                    token_uri: None,
                    extension: None,
                },
            },
        );

        assert!(res.is_ok());

        // minting one more should fail
        let res = contract.execute(
            deps.as_mut(),
            env.clone(),
            mock_info(minter.as_ref(), &[]),
            Cw1155BaseExecuteMsg::Mint {
                recipient: user1.clone(),
                msg: Cw1155MintMsg {
                    token_id: token1,
                    amount: 1u128.into(),
                    token_uri: None,
                    extension: None,
                },
            },
        );

        assert!(matches!(
            res,
            Err(Cw1155ContractError::Std(StdError::Overflow {
                source: OverflowError { .. },
                ..
            }))
        ));

        // minting one more of a different token id should fail
        let res = contract.execute(
            deps.as_mut(),
            env,
            mock_info(minter.as_ref(), &[]),
            Cw1155BaseExecuteMsg::Mint {
                recipient: user1,
                msg: Cw1155MintMsg {
                    token_id: token2,
                    amount: 1u128.into(),
                    token_uri: None,
                    extension: None,
                },
            },
        );

        assert!(matches!(
            res,
            Err(Cw1155ContractError::Std(StdError::Overflow {
                source: OverflowError { .. },
                ..
            }))
        ));
    }

    #[test]
    fn token_uri() {
        let contract = Cw1155BaseContract::default();
        let minter = String::from("minter");
        let user1 = String::from("user1");
        let token1 = "token1".to_owned();
        let url1 = "url1".to_owned();
        let url2 = "url2".to_owned();

        let mut deps = mock_dependencies();
        let msg = Cw1155InstantiateMsg {
            name: "name".to_string(),
            symbol: "symbol".to_string(),
            minter: Some(minter.to_string()),
        };
        let res = contract
            .instantiate(
                deps.as_mut(),
                mock_env(),
                mock_info("operator", &[]),
                msg,
                "contract_name",
                "contract_version",
            )
            .unwrap();
        assert_eq!(0, res.messages.len());

        // first mint
        contract
            .execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                Cw1155BaseExecuteMsg::Mint {
                    recipient: user1.clone(),
                    msg: Cw1155MintMsg {
                        token_id: token1.clone(),
                        amount: 1u64.into(),
                        token_uri: Some(url1.clone()),
                        extension: None,
                    },
                },
            )
            .unwrap();

        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::TokenInfo {
                    token_id: token1.clone()
                },
            ),
            to_json_binary(&TokenInfoResponse::<Option<Empty>> {
                token_uri: Some(url1.clone()),
                extension: None,
            })
        );

        // mint after the first mint
        contract
            .execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                Cw1155BaseExecuteMsg::Mint {
                    recipient: user1,
                    msg: Cw1155MintMsg {
                        token_id: token1.clone(),
                        amount: 1u64.into(),
                        token_uri: Some(url2),
                        extension: None,
                    },
                },
            )
            .unwrap();

        // url doesn't changed
        assert_eq!(
            contract.query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::TokenInfo { token_id: token1 },
            ),
            to_json_binary(&TokenInfoResponse::<Option<Empty>> {
                token_uri: Some(url1),
                extension: None,
            })
        );
    }
}
