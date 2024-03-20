use crate::{
    error::Cw721ContractError,
    execute::Cw721Execute,
    msg::{
        CollectionMetadataExtensionMsg, CollectionMetadataMsg, Cw721ExecuteMsg,
        Cw721InstantiateMsg, RoyaltyInfoResponse,
    },
    query::{Cw721Query, MAX_LIMIT},
    state::{
        CollectionMetadata, NftMetadata, CREATOR, MAX_COLLECTION_DESCRIPTION_LENGTH,
        MAX_ROYALTY_SHARE_DELTA_PCT, MAX_ROYALTY_SHARE_PCT, MINTER,
    },
    CollectionMetadataExtension, DefaultOptionCollectionMetadataExtension,
    DefaultOptionCollectionMetadataExtensionMsg, DefaultOptionNftMetadataExtension,
    DefaultOptionNftMetadataExtensionMsg, RoyaltyInfo,
};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Addr, Api, Decimal, Empty, Timestamp,
};
use cw2::ContractVersion;
use cw_ownable::Action;
use cw_storage_plus::Item;
use unit_tests::{
    contract::Cw721Contract,
    multi_tests::{CREATOR_ADDR, MINTER_ADDR, OTHER_ADDR},
};

use super::*;

#[test]
fn test_instantiation() {
    let mut deps = mock_dependencies();

    // error on empty name
    let err = Cw721Contract::<
        DefaultOptionNftMetadataExtension,
        DefaultOptionNftMetadataExtensionMsg,
        DefaultOptionCollectionMetadataExtension,
        DefaultOptionCollectionMetadataExtensionMsg,
        Empty,
    >::default()
    .instantiate(
        deps.as_mut(),
        &mock_env(),
        &mock_info("mr-t", &[]),
        Cw721InstantiateMsg {
            name: "".into(),
            symbol: "collection_symbol".into(),
            collection_metadata_extension: None,
            creator: None,
            minter: None,
            withdraw_address: None,
        },
        "contract_name",
        "contract_version",
    )
    .unwrap_err();
    assert_eq!(err, Cw721ContractError::CollectionNameEmpty {});

    // error on empty symbol
    let err = Cw721Contract::<
        DefaultOptionNftMetadataExtension,
        DefaultOptionNftMetadataExtensionMsg,
        DefaultOptionCollectionMetadataExtension,
        DefaultOptionCollectionMetadataExtensionMsg,
        Empty,
    >::default()
    .instantiate(
        deps.as_mut(),
        &mock_env(),
        &mock_info("mr-t", &[]),
        Cw721InstantiateMsg {
            name: "collection_name".into(),
            symbol: "".into(),
            collection_metadata_extension: None,
            creator: None,
            minter: None,
            withdraw_address: None,
        },
        "contract_name",
        "contract_version",
    )
    .unwrap_err();
    assert_eq!(err, Cw721ContractError::CollectionSymbolEmpty {});

    Cw721Contract::<
        DefaultOptionNftMetadataExtension,
        DefaultOptionNftMetadataExtensionMsg,
        DefaultOptionCollectionMetadataExtension,
        DefaultOptionCollectionMetadataExtensionMsg,
        Empty,
    >::default()
    .instantiate(
        deps.as_mut(),
        &mock_env(),
        &mock_info("larry", &[]),
        Cw721InstantiateMsg {
            name: "collection_name".into(),
            symbol: "collection_symbol".into(),
            collection_metadata_extension: None,
            minter: Some("minter".into()),
            creator: Some("creator".into()),
            withdraw_address: None,
        },
        "contract_name",
        "contract_version",
    )
    .unwrap();

    // assert minter and creator
    let minter = MINTER
        .get_ownership(deps.as_ref().storage)
        .unwrap()
        .owner
        .map(|a| a.into_string());
    assert_eq!(minter, Some("minter".to_string()));

    let creator = CREATOR
        .get_ownership(deps.as_ref().storage)
        .unwrap()
        .owner
        .map(|a| a.into_string());
    assert_eq!(creator, Some("creator".to_string()));

    //
    let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
    assert_eq!(
        version,
        ContractVersion {
            contract: "contract_name".into(),
            version: "contract_version".into(),
        },
    );
}

#[test]
fn test_instantiation_with_proper_minter_and_creator() {
    // case 1: sender is used in case minter and creator is not set
    {
        let mut deps = mock_dependencies();

        let info_minter_and_creator = mock_info("minter_and_creator", &[]);
        Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .instantiate(
            deps.as_mut(),
            &mock_env(),
            &info_minter_and_creator,
            Cw721InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_metadata_extension: None,
                creator: None,
                minter: None,
                withdraw_address: None,
            },
            "contract_name",
            "contract_version",
        )
        .unwrap();

        let minter = MINTER.item.load(deps.as_ref().storage).unwrap().owner;
        assert_eq!(minter, Some(info_minter_and_creator.sender.clone()));
        let creator = CREATOR.item.load(deps.as_ref().storage).unwrap().owner;
        assert_eq!(creator, Some(info_minter_and_creator.sender));
    }
    // case 2: minter and creator are set
    {
        let mut deps = mock_dependencies();

        let info = mock_info(OTHER_ADDR, &[]);
        Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .instantiate(
            deps.as_mut(),
            &mock_env(),
            &info,
            Cw721InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_metadata_extension: None,
                creator: Some(CREATOR_ADDR.into()),
                minter: Some(MINTER_ADDR.into()),
                withdraw_address: None,
            },
            "contract_name",
            "contract_version",
        )
        .unwrap();

        let minter = MINTER.item.load(deps.as_ref().storage).unwrap().owner;
        assert_eq!(minter, Some(Addr::unchecked(MINTER_ADDR.to_string())));
        let creator = CREATOR.item.load(deps.as_ref().storage).unwrap().owner;
        assert_eq!(creator, Some(Addr::unchecked(CREATOR_ADDR.to_string())));
    }
    // case 3: sender is minter and creator is set
    {
        let mut deps = mock_dependencies();

        let info = mock_info(MINTER_ADDR, &[]);
        Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .instantiate(
            deps.as_mut(),
            &mock_env(),
            &info,
            Cw721InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_metadata_extension: None,
                creator: Some(CREATOR_ADDR.into()),
                minter: None,
                withdraw_address: None,
            },
            "contract_name",
            "contract_version",
        )
        .unwrap();

        let minter = MINTER.item.load(deps.as_ref().storage).unwrap().owner;
        assert_eq!(minter, Some(info.sender.clone()));
        let creator = CREATOR.item.load(deps.as_ref().storage).unwrap().owner;
        assert_eq!(creator, Some(Addr::unchecked(CREATOR_ADDR.to_string())));
    }
    // case 4: sender is creator and minter is set
    {
        let mut deps = mock_dependencies();

        let info = mock_info(CREATOR_ADDR, &[]);
        Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .instantiate(
            deps.as_mut(),
            &mock_env(),
            &info,
            Cw721InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_metadata_extension: None,
                creator: None,
                minter: Some(MINTER_ADDR.into()),
                withdraw_address: None,
            },
            "contract_name",
            "contract_version",
        )
        .unwrap();

        let minter = MINTER.item.load(deps.as_ref().storage).unwrap().owner;
        assert_eq!(minter, Some(Addr::unchecked(MINTER_ADDR.to_string())));
        let creator = CREATOR.item.load(deps.as_ref().storage).unwrap().owner;
        assert_eq!(creator, Some(info.sender));
    }
}

#[test]
fn test_instantiation_with_collection_metadata() {
    // case 1: extension set with proper data
    {
        let mut deps = mock_dependencies();

        let info_creator = mock_info(CREATOR_ADDR, &[]);
        let extension = Some(CollectionMetadataExtension {
            description: "description".into(),
            image: "https://moonphases.org".to_string(),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            // no minter owner assertion on start trading time, so even creator can change this here
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfo {
                payment_address: Addr::unchecked("payment_address"),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        let extension_msg = Some(CollectionMetadataExtensionMsg {
            description: Some("description".into()),
            image: Some("https://moonphases.org".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .instantiate(
            deps.as_mut(),
            &mock_env(),
            &info_creator,
            Cw721InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_metadata_extension: extension_msg.clone(),
                creator: Some(CREATOR_ADDR.into()),
                minter: Some(MINTER_ADDR.into()),
                withdraw_address: None,
            },
            "contract_name",
            "contract_version",
        )
        .unwrap();

        // validate data
        let collection_metadata = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .query_collection_metadata(deps.as_ref(), &mock_env())
        .unwrap();
        assert_eq!(collection_metadata.name, "collection_name");
        assert_eq!(collection_metadata.symbol, "collection_symbol");
        assert_eq!(collection_metadata.extension, extension);
    }
    // case 2: invalid data
    {
        // invalid image
        let mut deps = mock_dependencies();

        let info_creator = mock_info(CREATOR_ADDR, &[]);
        let extension_msg = Some(CollectionMetadataExtensionMsg {
            description: Some("description".into()),
            image: Some("invalid_url".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        let err = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .instantiate(
            deps.as_mut(),
            &mock_env(),
            &info_creator,
            Cw721InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_metadata_extension: extension_msg.clone(),
                creator: Some(CREATOR_ADDR.into()),
                minter: Some(MINTER_ADDR.into()),
                withdraw_address: None,
            },
            "contract_name",
            "contract_version",
        )
        .unwrap_err();
        assert_eq!(
            err,
            Cw721ContractError::ParseError(url::ParseError::RelativeUrlWithoutBase)
        );

        // invalid external link
        let extension_msg = Some(CollectionMetadataExtensionMsg {
            description: Some("description".into()),
            image: Some("https://moonphases.org".to_string()),
            explicit_content: Some(true),
            external_link: Some("invalid_url".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        let err = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .instantiate(
            deps.as_mut(),
            &mock_env(),
            &info_creator,
            Cw721InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_metadata_extension: extension_msg.clone(),
                creator: Some(CREATOR_ADDR.into()),
                minter: Some(MINTER_ADDR.into()),
                withdraw_address: None,
            },
            "contract_name",
            "contract_version",
        )
        .unwrap_err();
        assert_eq!(
            err,
            Cw721ContractError::ParseError(url::ParseError::RelativeUrlWithoutBase)
        );

        // empty description
        let extension_msg = Some(CollectionMetadataExtensionMsg {
            description: Some("".into()),
            image: Some("https://moonphases.org".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        let err = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .instantiate(
            deps.as_mut(),
            &mock_env(),
            &info_creator,
            Cw721InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_metadata_extension: extension_msg.clone(),
                creator: Some(CREATOR_ADDR.into()),
                minter: Some(MINTER_ADDR.into()),
                withdraw_address: None,
            },
            "contract_name",
            "contract_version",
        )
        .unwrap_err();
        assert_eq!(err, Cw721ContractError::CollectionDescriptionEmpty {});

        // description too long
        let extension_msg = Some(CollectionMetadataExtensionMsg {
            description: Some("a".repeat(1001)),
            image: Some("https://moonphases.org".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        let err = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .instantiate(
            deps.as_mut(),
            &mock_env(),
            &info_creator,
            Cw721InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_metadata_extension: extension_msg.clone(),
                creator: Some(CREATOR_ADDR.into()),
                minter: Some(MINTER_ADDR.into()),
                withdraw_address: None,
            },
            "contract_name",
            "contract_version",
        )
        .unwrap_err();
        assert_eq!(
            err,
            Cw721ContractError::CollectionDescriptionTooLong {
                max_length: MAX_COLLECTION_DESCRIPTION_LENGTH
            }
        );

        // royalty share too high
        let extension_msg = Some(CollectionMetadataExtensionMsg {
            description: Some("description".into()),
            image: Some("https://moonphases.org".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: (MAX_ROYALTY_SHARE_PCT * 2).to_string().parse().unwrap(),
            }),
        });
        let err = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .instantiate(
            deps.as_mut(),
            &mock_env(),
            &info_creator,
            Cw721InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_metadata_extension: extension_msg.clone(),
                creator: Some(CREATOR_ADDR.into()),
                minter: Some(MINTER_ADDR.into()),
                withdraw_address: None,
            },
            "contract_name",
            "contract_version",
        )
        .unwrap_err();
        assert_eq!(
            err,
            Cw721ContractError::InvalidRoyalties(format!(
                "Share cannot be greater than {MAX_ROYALTY_SHARE_PCT}%"
            ))
        );
    }
}

#[test]
fn test_collection_metadata_update() {
    // case 1: update with proper data
    {
        // initialize contract
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info_creator = mock_info(CREATOR_ADDR, &[]);
        let expected_instantiated_extension = Some(CollectionMetadataExtension {
            description: "description".into(),
            image: "https://moonphases.org".to_string(),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfo {
                payment_address: deps.api.addr_validate("payment_address").unwrap(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        let instantiated_extension_msg = Some(CollectionMetadataExtensionMsg {
            description: Some("description".into()),
            image: Some("https://moonphases.org".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        let contract = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default();
        contract
            .instantiate(
                deps.as_mut(),
                &env,
                &info_creator,
                Cw721InstantiateMsg {
                    name: "collection_name".into(),
                    symbol: "collection_symbol".into(),
                    collection_metadata_extension: instantiated_extension_msg.clone(),
                    creator: Some(CREATOR_ADDR.into()),
                    minter: Some(MINTER_ADDR.into()),
                    withdraw_address: None,
                },
                "contract_name",
                "contract_version",
            )
            .unwrap();

        // update collection with no data
        let empty_extension_msg = CollectionMetadataExtensionMsg {
            description: None,
            image: None,
            explicit_content: None,
            external_link: None,
            start_trading_time: None,
            royalty_info: None,
        };
        let empty_collection_metadata_msg = CollectionMetadataMsg {
            name: None,
            symbol: None,
            extension: Some(empty_extension_msg),
        };
        contract
            .execute(
                deps.as_mut(),
                &env,
                &info_creator,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: empty_collection_metadata_msg,
                },
            )
            .unwrap();
        // validate data
        let collection_metadata = contract
            .query_collection_metadata(deps.as_ref(), &mock_env())
            .unwrap();
        assert_eq!(collection_metadata.name, "collection_name");
        assert_eq!(collection_metadata.symbol, "collection_symbol");
        assert_eq!(
            collection_metadata.extension,
            expected_instantiated_extension
        );

        // update collection with proper data by creator
        let updated_extension_msg = CollectionMetadataExtensionMsg {
            description: Some("new_description".into()),
            image: Some("https://en.wikipedia.org/wiki/Non-fungible_token".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://github.com/CosmWasm/cw-nfts".to_string()),
            start_trading_time: None, // start trading time belongs to minter - not creator!
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        };
        let updated_collection_metadata_msg = CollectionMetadataMsg {
            name: Some("new_collection_name".into()),
            symbol: Some("new_collection_symbol".into()),
            extension: Some(updated_extension_msg),
        };
        contract
            .execute(
                deps.as_mut(),
                &env,
                &info_creator,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg,
                },
            )
            .unwrap();

        // validate data
        let collection_metadata = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .query_collection_metadata(deps.as_ref(), &mock_env())
        .unwrap();
        assert_eq!(collection_metadata.name, "new_collection_name");
        assert_eq!(collection_metadata.symbol, "new_collection_symbol");
        assert_eq!(
            collection_metadata.extension,
            Some(CollectionMetadataExtension {
                description: "new_description".into(),
                image: "https://en.wikipedia.org/wiki/Non-fungible_token".to_string(),
                explicit_content: Some(true),
                external_link: Some("https://github.com/CosmWasm/cw-nfts".to_string()),
                start_trading_time: Some(Timestamp::from_seconds(0)),
                royalty_info: Some(RoyaltyInfo {
                    payment_address: Addr::unchecked("payment_address"),
                    share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                        .to_string()
                        .parse()
                        .unwrap(),
                }),
            })
        );

        // update start trading time by minter
        let updated_extension_msg = CollectionMetadataExtensionMsg {
            description: None,
            image: None,
            explicit_content: None,
            external_link: None,
            start_trading_time: Some(Timestamp::from_seconds(1)),
            royalty_info: None,
        };
        let updated_collection_metadata_msg = CollectionMetadataMsg {
            name: None,
            symbol: None,
            extension: Some(updated_extension_msg),
        };
        let info_minter = mock_info(MINTER_ADDR, &[]);
        contract
            .execute(
                deps.as_mut(),
                &env,
                &info_minter,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg,
                },
            )
            .unwrap();
        // validate data
        let collection_metadata = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default()
        .query_collection_metadata(deps.as_ref(), &mock_env())
        .unwrap();
        assert_eq!(collection_metadata.name, "new_collection_name");
        assert_eq!(collection_metadata.symbol, "new_collection_symbol");
        assert_eq!(
            collection_metadata.extension,
            Some(CollectionMetadataExtension {
                description: "new_description".into(),
                image: "https://en.wikipedia.org/wiki/Non-fungible_token".to_string(),
                explicit_content: Some(true),
                external_link: Some("https://github.com/CosmWasm/cw-nfts".to_string()),
                start_trading_time: Some(Timestamp::from_seconds(1)),
                royalty_info: Some(RoyaltyInfo {
                    payment_address: Addr::unchecked("payment_address"),
                    share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                        .to_string()
                        .parse()
                        .unwrap(),
                }),
            })
        );
    }
    // case 2: udpate with invalid data
    {
        // initialize contract
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(CREATOR_ADDR, &[]);
        let instantiated_extension_msg = Some(CollectionMetadataExtensionMsg {
            description: Some("description".into()),
            image: Some("https://moonphases.org".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        let contract = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default();
        contract
            .instantiate(
                deps.as_mut(),
                &env,
                &info,
                Cw721InstantiateMsg {
                    name: "collection_name".into(),
                    symbol: "collection_symbol".into(),
                    collection_metadata_extension: instantiated_extension_msg.clone(),
                    creator: None,
                    minter: None,
                    withdraw_address: None,
                },
                "contract_name",
                "contract_version",
            )
            .unwrap();

        // empty description
        let updated_extension_msg = CollectionMetadataExtensionMsg {
            description: Some("".into()),
            image: Some("https://en.wikipedia.org/wiki/Non-fungible_token".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://github.com/CosmWasm/cw-nfts".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        };
        let updated_collection_metadata_msg = CollectionMetadataMsg {
            name: Some("new_collection_name".into()),
            symbol: Some("new_collection_symbol".into()),
            extension: Some(updated_extension_msg),
        };
        let err = contract
            .execute(
                deps.as_mut(),
                &env,
                &info,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg,
                },
            )
            .unwrap_err();
        assert_eq!(err, Cw721ContractError::CollectionDescriptionEmpty {});

        // description too long
        let updated_extension_msg = CollectionMetadataExtensionMsg {
            description: Some("a".repeat(1001)),
            image: Some("https://en.wikipedia.org/wiki/Non-fungible_token".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://github.com/CosmWasm/cw-nfts".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        };
        let updated_collection_metadata_msg = CollectionMetadataMsg {
            name: Some("new_collection_name".into()),
            symbol: Some("new_collection_symbol".into()),
            extension: Some(updated_extension_msg),
        };
        let err = contract
            .execute(
                deps.as_mut(),
                &env,
                &info,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg,
                },
            )
            .unwrap_err();
        assert_eq!(
            err,
            Cw721ContractError::CollectionDescriptionTooLong {
                max_length: MAX_COLLECTION_DESCRIPTION_LENGTH
            }
        );

        // invalid image url
        let updated_extension_msg = CollectionMetadataExtensionMsg {
            description: Some("new_description".into()),
            image: Some("invalid_url".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://github.com/CosmWasm/cw-nfts".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        };
        let updated_collection_metadata_msg = CollectionMetadataMsg {
            name: Some("new_collection_name".into()),
            symbol: Some("new_collection_symbol".into()),
            extension: Some(updated_extension_msg),
        };
        let err = contract
            .execute(
                deps.as_mut(),
                &env,
                &info,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg,
                },
            )
            .unwrap_err();
        assert_eq!(
            err,
            Cw721ContractError::ParseError(url::ParseError::RelativeUrlWithoutBase)
        );

        // invalid external link url
        let updated_extension_msg = CollectionMetadataExtensionMsg {
            description: Some("new_description".into()),
            image: Some("https://en.wikipedia.org/wiki/Non-fungible_token".to_string()),
            explicit_content: Some(true),
            external_link: Some("invalid_url".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        };
        let updated_collection_metadata_msg = CollectionMetadataMsg {
            name: Some("new_collection_name".into()),
            symbol: Some("new_collection_symbol".into()),
            extension: Some(updated_extension_msg),
        };
        let err = contract
            .execute(
                deps.as_mut(),
                &env,
                &info,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg,
                },
            )
            .unwrap_err();
        assert_eq!(
            err,
            Cw721ContractError::ParseError(url::ParseError::RelativeUrlWithoutBase)
        );

        // royalty share too high
        let updated_extension_msg = CollectionMetadataExtensionMsg {
            description: Some("new_description".into()),
            image: Some("https://en.wikipedia.org/wiki/Non-fungible_token".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://github.com/CosmWasm/cw-nfts".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT + MAX_ROYALTY_SHARE_DELTA_PCT - 1)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        };
        let updated_collection_metadata_msg = CollectionMetadataMsg {
            name: Some("new_collection_name".into()),
            symbol: Some("new_collection_symbol".into()),
            extension: Some(updated_extension_msg),
        };
        let err = contract
            .execute(
                deps.as_mut(),
                &env,
                &info,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg,
                },
            )
            .unwrap_err();
        assert_eq!(
            err,
            Cw721ContractError::InvalidRoyalties(format!(
                "Share cannot be greater than {MAX_ROYALTY_SHARE_PCT}%"
            ))
        );

        // max share delta too high
        let updated_extension_msg = CollectionMetadataExtensionMsg {
            description: Some("new_description".into()),
            image: Some("https://en.wikipedia.org/wiki/Non-fungible_token".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://github.com/CosmWasm/cw-nfts".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT + MAX_ROYALTY_SHARE_DELTA_PCT + 1)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        };
        let updated_collection_metadata_msg = CollectionMetadataMsg {
            name: Some("new_collection_name".into()),
            symbol: Some("new_collection_symbol".into()),
            extension: Some(updated_extension_msg),
        };
        let err = contract
            .execute(
                deps.as_mut(),
                &env,
                &info,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg,
                },
            )
            .unwrap_err();
        assert_eq!(
            err,
            Cw721ContractError::InvalidRoyalties(format!(
                "Share increase cannot be greater than {MAX_ROYALTY_SHARE_DELTA_PCT}%"
            ))
        );
    }
    // case 3: non-creator updating data
    {
        // initialize contract
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(CREATOR_ADDR, &[]);
        let instantiated_extension = Some(CollectionMetadataExtensionMsg {
            description: Some("description".into()),
            image: Some("https://moonphases.org".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        let contract = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default();
        contract
            .instantiate(
                deps.as_mut(),
                &env,
                &info,
                Cw721InstantiateMsg {
                    name: "collection_name".into(),
                    symbol: "collection_symbol".into(),
                    collection_metadata_extension: instantiated_extension.clone(),
                    creator: None,
                    minter: None,
                    withdraw_address: None,
                },
                "contract_name",
                "contract_version",
            )
            .unwrap();

        // update collection by other user
        let updated_extension_msg = CollectionMetadataExtensionMsg {
            description: Some("new_description".into()),
            image: Some("https://en.wikipedia.org/wiki/Non-fungible_token".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://github.com/CosmWasm/cw-nfts".to_string()),
            start_trading_time: None,
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        };
        let updated_collection_metadata_msg = CollectionMetadataMsg {
            name: Some("new_collection_name".into()),
            symbol: Some("new_collection_symbol".into()),
            extension: Some(updated_extension_msg),
        };
        let info_other = mock_info(OTHER_ADDR, &[]);
        let err = contract
            .execute(
                deps.as_mut(),
                &env,
                &info_other,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, Cw721ContractError::NotCreator {});
        // transfer creator to other user
        contract
            .execute(
                deps.as_mut(),
                &env,
                &info,
                Cw721ExecuteMsg::UpdateCreatorOwnership(Action::TransferOwnership {
                    new_owner: info_other.sender.to_string(),
                    expiry: None,
                }),
            )
            .unwrap();
        // other still cannot update collection, until ownership is accepted
        let err = contract
            .execute(
                deps.as_mut(),
                &env,
                &info_other,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, Cw721ContractError::NotCreator {});
        // accept ownership
        contract
            .execute(
                deps.as_mut(),
                &env,
                &info_other,
                Cw721ExecuteMsg::UpdateCreatorOwnership(Action::AcceptOwnership {}),
            )
            .unwrap();
        // other can update collection now
        contract
            .execute(
                deps.as_mut(),
                &env,
                &info_other,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg,
                },
            )
            .unwrap();
    }
    // case 4: minter updating data
    {
        // initialize contract
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info_creator = mock_info(CREATOR_ADDR, &[]);
        let info_minter = mock_info(MINTER_ADDR, &[]);
        let instantiated_extension = Some(CollectionMetadataExtensionMsg {
            description: Some("description".into()),
            image: Some("https://moonphases.org".to_string()),
            explicit_content: Some(true),
            external_link: Some("https://moonphases.org".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(0)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "payment_address".into(),
                share: Decimal::percent(MAX_ROYALTY_SHARE_PCT)
                    .to_string()
                    .parse()
                    .unwrap(),
            }),
        });
        let contract = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default();
        contract
            .instantiate(
                deps.as_mut(),
                &env,
                &info_creator,
                Cw721InstantiateMsg {
                    name: "collection_name".into(),
                    symbol: "collection_symbol".into(),
                    collection_metadata_extension: instantiated_extension.clone(),
                    creator: None, // in case of none, sender is creator
                    minter: info_minter.sender.to_string().into(),
                    withdraw_address: None,
                },
                "contract_name",
                "contract_version",
            )
            .unwrap();

        // update start trading time by creator user not allowed
        let updated_extension_msg = CollectionMetadataExtensionMsg {
            description: None,
            image: None,
            explicit_content: None,
            external_link: None,
            start_trading_time: Some(Timestamp::from_seconds(1)),
            royalty_info: None,
        };
        let updated_collection_metadata_msg = CollectionMetadataMsg {
            name: None,
            symbol: None,
            extension: Some(updated_extension_msg),
        };
        let err = contract
            .execute(
                deps.as_mut(),
                &env,
                &info_creator,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, Cw721ContractError::NotMinter {});
        // update start trading time by minter
        contract
            .execute(
                deps.as_mut(),
                &env,
                &info_minter,
                Cw721ExecuteMsg::UpdateCollectionMetadata {
                    collection_metadata: updated_collection_metadata_msg,
                },
            )
            .unwrap();
        // assert start trading has changed
        let collection_metadata = contract
            .query_collection_metadata(deps.as_ref(), &env)
            .unwrap();
        assert_eq!(
            collection_metadata.extension.unwrap().start_trading_time,
            Some(Timestamp::from_seconds(1))
        );
    }
}

#[test]
fn test_nft_mint() {
    // case 1: mint without onchain metadata
    {
        let mut deps = mock_dependencies();
        let contract = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default();

        let info = mock_info(CREATOR_ADDR, &[]);
        let init_msg = Cw721InstantiateMsg {
            name: "collection_name".into(),
            symbol: "collection_symbol".into(),
            collection_metadata_extension: None,
            minter: None,
            creator: None,
            withdraw_address: None,
        };
        let env = mock_env();
        contract
            .instantiate(
                deps.as_mut(),
                &env,
                &info,
                init_msg,
                "contract_name",
                "contract_version",
            )
            .unwrap();

        let token_id = "Enterprise";
        let token_uri = Some("https://starships.example.com/Starship/Enterprise.json".into());
        let extension = None;
        let exec_msg = Cw721ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: token_uri.clone(),
            extension: extension.clone(),
        };
        contract
            .execute(deps.as_mut(), &env, &info, exec_msg)
            .unwrap();

        let res = contract
            .query_nft_info(deps.as_ref(), &env, token_id.into())
            .unwrap();
        assert_eq!(res.token_uri, token_uri);
        assert_eq!(res.extension, extension);
        // mint with empty token_uri
        let exec_msg = Cw721ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: "".to_string().into(), // empty token_uri
            extension: extension.clone(),
        };
        let err = contract
            .execute(deps.as_mut(), &env, &info, exec_msg)
            .unwrap_err();
        assert_eq!(
            err,
            Cw721ContractError::ParseError(url::ParseError::RelativeUrlWithoutBase)
        );
        // non-minter cant mint
        let info = mock_info("john", &[]);
        let exec_msg = Cw721ExecuteMsg::Mint {
            token_id: "Enterprise".to_string(),
            owner: "john".to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: None,
        };
        let err = contract
            .execute(deps.as_mut(), &env, &info, exec_msg)
            .unwrap_err();
        assert_eq!(err, Cw721ContractError::NotMinter {});
    }
    // case 2: mint with onchain metadata
    {
        let mut deps = mock_dependencies();
        let contract = Cw721Contract::<
            DefaultOptionNftMetadataExtension,
            DefaultOptionNftMetadataExtensionMsg,
            DefaultOptionCollectionMetadataExtension,
            DefaultOptionCollectionMetadataExtensionMsg,
            Empty,
        >::default();

        let info = mock_info(CREATOR_ADDR, &[]);
        let init_msg = Cw721InstantiateMsg {
            name: "collection_name".into(),
            symbol: "collection_symbol".into(),
            collection_metadata_extension: None,
            minter: None,
            creator: None,
            withdraw_address: None,
        };
        let env = mock_env();
        contract
            .instantiate(
                deps.as_mut(),
                &env,
                &info,
                init_msg,
                "contract_name",
                "contract_version",
            )
            .unwrap();

        let token_id = "Enterprise";
        let token_uri = Some("https://starships.example.com/Starship/Enterprise.json".into());
        let extension = Some(NftMetadata {
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..NftMetadata::default()
        });
        let exec_msg = Cw721ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: token_uri.clone(),
            extension: extension.clone(),
        };
        contract
            .execute(deps.as_mut(), &env, &info, exec_msg)
            .unwrap();

        let res = contract
            .query_nft_info(deps.as_ref(), &env, token_id.into())
            .unwrap();
        assert_eq!(res.token_uri, token_uri);
        assert_eq!(res.extension, extension);
    }
}

#[test]
fn test_migrate() {
    let mut deps = mock_dependencies();

    // instantiate v16 contract
    let env = mock_env();
    use cw721_base_016 as v16;
    v16::entry::instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &[]),
        v16::InstantiateMsg {
            name: "legacy_name".into(),
            symbol: "legacy_symbol".into(),
            minter: "legacy_minter".into(),
        },
    )
    .unwrap();

    // mint 200 NFTs before migration - using v16 contract
    for i in 0..200 {
        let info = mock_info("legacy_minter", &[]);
        let msg = v16::ExecuteMsg::Mint(v16::msg::MintMsg {
            token_id: i.to_string(),
            owner: "owner".into(),
            token_uri: None,
            extension: None,
        });
        v16::entry::execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    }

    // assert new data before migration:
    // - minter, creator, and collection metadata throws NotFound Error
    MINTER.item.load(deps.as_ref().storage).unwrap_err(); // cw_ownable in v16 is used for minter
    let contract = Cw721Contract::<
        DefaultOptionNftMetadataExtension,
        DefaultOptionNftMetadataExtensionMsg,
        DefaultOptionCollectionMetadataExtension,
        DefaultOptionCollectionMetadataExtensionMsg,
        Empty,
    >::default();
    contract
        .query_collection_metadata(deps.as_ref(), &env)
        .unwrap_err();
    // - query in new minter and creator ownership store throws NotFound Error (in v16 it was stored outside cw_ownable, in dedicated "minter" store)
    MINTER.get_ownership(deps.as_ref().storage).unwrap_err();
    CREATOR.get_ownership(deps.as_ref().storage).unwrap_err();
    // assert legacy data before migration:
    // - version
    let version = cw2::get_contract_version(deps.as_ref().storage)
        .unwrap()
        .version;
    assert_eq!(version, "0.16.0");
    // - legacy minter is set
    let legacy_minter_store: Item<Addr> = Item::new("minter");
    let legacy_minter = legacy_minter_store.load(deps.as_ref().storage).unwrap();
    assert_eq!(legacy_minter, "legacy_minter");
    // - legacy collection metadata is set
    let legacy_collection_metadata_store: Item<cw721_016::ContractInfoResponse> =
        Item::new("nft_info");
    let legacy_collection_metadata = legacy_collection_metadata_store
        .load(deps.as_ref().storage)
        .unwrap();
    assert_eq!(legacy_collection_metadata.name, "legacy_name");
    assert_eq!(legacy_collection_metadata.symbol, "legacy_symbol");
    // 200 NFTs still exist
    let all_tokens = contract
        .query_all_tokens(deps.as_ref(), &env, None, Some(MAX_LIMIT))
        .unwrap();
    assert_eq!(all_tokens.tokens.len(), 200);
    // NFTs have proper owner
    for token_id in 0..200 {
        let token = contract
            .query_owner_of(deps.as_ref(), &env, token_id.to_string(), false)
            .unwrap();
        assert_eq!(token.owner.as_str(), "owner");
    }

    // migrate
    Cw721Contract::<
        DefaultOptionNftMetadataExtension,
        DefaultOptionNftMetadataExtensionMsg,
        DefaultOptionCollectionMetadataExtension,
        DefaultOptionCollectionMetadataExtensionMsg,
        Empty,
    >::default()
    .migrate(
        deps.as_mut(),
        env.clone(),
        crate::msg::Cw721MigrateMsg::WithUpdate {
            minter: None,
            creator: None,
        },
        "contract_name",
        "new_contract_version",
    )
    .unwrap();

    // assert version has changed
    let version = cw2::get_contract_version(deps.as_ref().storage)
        .unwrap()
        .version;
    assert_eq!(version, "new_contract_version");

    // assert minter ownership
    let minter_ownership = MINTER
        .get_ownership(deps.as_ref().storage)
        .unwrap()
        .owner
        .map(|a| a.into_string());
    assert_eq!(minter_ownership, Some("legacy_minter".to_string()));

    // assert creator ownership
    let creator_ownership = CREATOR
        .get_ownership(deps.as_ref().storage)
        .unwrap()
        .owner
        .map(|a| a.into_string());
    assert_eq!(creator_ownership, Some("legacy_minter".to_string()));

    // assert collection metadata
    let collection_metadata = contract
        .query_collection_metadata(deps.as_ref(), &env)
        .unwrap();
    let legacy_contract_info = CollectionMetadata {
        name: "legacy_name".to_string(),
        symbol: "legacy_symbol".to_string(),
        extension: None,
        updated_at: env.block.time,
    };
    assert_eq!(collection_metadata, legacy_contract_info);

    // assert tokens
    let all_tokens = contract
        .query_all_tokens(deps.as_ref(), &env, None, Some(MAX_LIMIT))
        .unwrap();
    assert_eq!(all_tokens.tokens.len(), 200);

    // assert legacy data is still there (allowing backward migration in case of issues)
    // - minter
    let legacy_minter = legacy_minter_store.load(deps.as_ref().storage).unwrap();
    assert_eq!(legacy_minter, "legacy_minter");
    // - legacy collection metadata
    let legacy_collection_metadata = legacy_collection_metadata_store
        .load(deps.as_ref().storage)
        .unwrap();
    assert_eq!(legacy_collection_metadata.name, "legacy_name");
    assert_eq!(legacy_collection_metadata.symbol, "legacy_symbol");
    // - tokens are unchanged/still exist
    let all_tokens = contract
        .query_all_tokens(deps.as_ref(), &env, None, Some(MAX_LIMIT))
        .unwrap();
    assert_eq!(all_tokens.tokens.len(), 200);
    for token_id in 0..200 {
        let token = contract
            .query_owner_of(deps.as_ref(), &env, token_id.to_string(), false)
            .unwrap();
        assert_eq!(token.owner.as_str(), "owner");
    }
}
