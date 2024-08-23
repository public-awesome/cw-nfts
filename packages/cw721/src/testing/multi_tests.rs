use crate::{
    error::Cw721ContractError,
    extension::Cw721OnchainExtensions,
    msg::{
        CollectionExtensionMsg, ConfigResponse, Cw721ExecuteMsg, Cw721InstantiateMsg,
        Cw721MigrateMsg, Cw721QueryMsg, MinterResponse, NumTokensResponse, OwnerOfResponse,
        RoyaltyInfoResponse,
    },
    state::{CollectionInfo, NftExtension, Trait},
    traits::{Cw721Execute, Cw721Query},
    DefaultOptionalCollectionExtension, DefaultOptionalCollectionExtensionMsg,
    DefaultOptionalNftExtension, DefaultOptionalNftExtensionMsg, NftExtensionMsg,
};
use anyhow::Result;
use bech32::{decode, encode, Hrp};
use cosmwasm_std::{
    instantiate2_address, to_json_binary, Addr, Api, Binary, CanonicalAddr, Decimal, Deps, DepsMut,
    Empty, Env, GovMsg, MemoryStorage, MessageInfo, QuerierWrapper, RecoverPubkeyError, Response,
    StdError, StdResult, Storage, Timestamp, VerificationError, WasmMsg,
};
use cw721_016::NftInfoResponse;
use cw_multi_test::{
    AddressGenerator, App, AppBuilder, BankKeeper, Contract, ContractWrapper, DistributionKeeper,
    Executor, FailingModule, IbcAcceptingModule, Router, StakeKeeper, StargateFailing, WasmKeeper,
};
use cw_ownable::{Ownership, OwnershipError};
use cw_utils::Expiration;
use sha2::{digest::Update, Digest, Sha256};
use url::ParseError;

const BECH32_PREFIX_HRP: &str = "stars";
pub const ADMIN_ADDR: &str = "admin";
pub const CREATOR_ADDR: &str = "creator";
pub const MINTER_ADDR: &str = "minter";
pub const OTHER1_ADDR: &str = "other";
pub const OTHER2_ADDR: &str = "other";
pub const NFT_OWNER_ADDR: &str = "nft_owner";

type MockRouter = Router<
    BankKeeper,
    FailingModule<Empty, Empty, Empty>,
    WasmKeeper<Empty, Empty>,
    StakeKeeper,
    DistributionKeeper,
    IbcAcceptingModule,
    FailingModule<GovMsg, Empty, Empty>,
    StargateFailing,
>;

type MockApp = App<
    BankKeeper,
    MockApiBech32,
    MemoryStorage,
    FailingModule<Empty, Empty, Empty>,
    WasmKeeper<Empty, Empty>,
    StakeKeeper,
    DistributionKeeper,
    IbcAcceptingModule,
>;

#[derive(Default)]
pub struct MockAddressGenerator;

impl AddressGenerator for MockAddressGenerator {
    fn contract_address(
        &self,
        api: &dyn Api,
        _storage: &mut dyn Storage,
        code_id: u64,
        instance_id: u64,
    ) -> Result<Addr> {
        let canonical_addr = Self::instantiate_address(code_id, instance_id);
        Ok(Addr::unchecked(api.addr_humanize(&canonical_addr)?))
    }

    fn predictable_contract_address(
        &self,
        api: &dyn Api,
        _storage: &mut dyn Storage,
        _code_id: u64,
        _instance_id: u64,
        checksum: &[u8],
        creator: &CanonicalAddr,
        salt: &[u8],
    ) -> Result<Addr> {
        let canonical_addr = instantiate2_address(checksum, creator, salt)?;
        Ok(Addr::unchecked(api.addr_humanize(&canonical_addr)?))
    }
}

impl MockAddressGenerator {
    // non-predictable contract address generator, see `BuildContractAddressClassic`
    // implementation in wasmd: https://github.com/CosmWasm/wasmd/blob/main/x/wasm/keeper/addresses.go#L35-L42
    fn instantiate_address(code_id: u64, instance_id: u64) -> CanonicalAddr {
        let mut key = Vec::<u8>::new();
        key.extend_from_slice(b"wasm\0");
        key.extend_from_slice(&code_id.to_be_bytes());
        key.extend_from_slice(&instance_id.to_be_bytes());
        let module = Sha256::digest("module".as_bytes());
        Sha256::new()
            .chain(module)
            .chain(key)
            .finalize()
            .to_vec()
            .into()
    }
}
pub struct MockApiBech32 {
    prefix: Hrp,
}

impl MockApiBech32 {
    pub fn new(prefix: &'static str) -> Self {
        Self {
            prefix: Hrp::parse(prefix).unwrap(),
        }
    }
}

impl Api for MockApiBech32 {
    fn addr_validate(&self, input: &str) -> StdResult<Addr> {
        let canonical = self.addr_canonicalize(input)?;
        let normalized = self.addr_humanize(&canonical)?;
        if input != normalized {
            Err(StdError::generic_err(
                "Invalid input: address not normalized",
            ))
        } else {
            Ok(Addr::unchecked(input))
        }
    }

    fn addr_canonicalize(&self, input: &str) -> StdResult<CanonicalAddr> {
        if let Ok((prefix, decoded)) = decode(input) {
            if prefix == self.prefix {
                return Ok(decoded.into());
            }
        }
        Err(StdError::generic_err(format!("Invalid input: {input}")))
    }

    fn addr_humanize(&self, canonical: &CanonicalAddr) -> StdResult<Addr> {
        let hrp = self.prefix;
        let data = canonical.as_slice();
        if let Ok(encoded) = encode::<bech32::Bech32>(hrp, data) {
            Ok(Addr::unchecked(encoded))
        } else {
            Err(StdError::generic_err("Invalid canonical address"))
        }
    }

    fn secp256k1_verify(
        &self,
        _message_hash: &[u8],
        _signature: &[u8],
        _public_key: &[u8],
    ) -> Result<bool, VerificationError> {
        unimplemented!()
    }

    fn secp256k1_recover_pubkey(
        &self,
        _message_hash: &[u8],
        _signature: &[u8],
        _recovery_param: u8,
    ) -> Result<Vec<u8>, RecoverPubkeyError> {
        unimplemented!()
    }

    fn ed25519_verify(
        &self,
        _message: &[u8],
        _signature: &[u8],
        _public_key: &[u8],
    ) -> Result<bool, VerificationError> {
        unimplemented!()
    }

    fn ed25519_batch_verify(
        &self,
        _messages: &[&[u8]],
        _signatures: &[&[u8]],
        _public_keys: &[&[u8]],
    ) -> Result<bool, VerificationError> {
        unimplemented!()
    }

    fn debug(&self, _message: &str) {
        unimplemented!()
    }
}

impl MockApiBech32 {
    pub fn addr_make(&self, input: &str) -> Addr {
        let digest = Sha256::digest(input).to_vec();
        match encode::<bech32::Bech32>(self.prefix, &digest) {
            Ok(address) => Addr::unchecked(address),
            Err(reason) => panic!("Generating address failed with reason: {reason}"),
        }
    }
}

fn new() -> MockApp {
    AppBuilder::new()
        .with_wasm::<WasmKeeper<Empty, Empty>>(
            WasmKeeper::new().with_address_generator(MockAddressGenerator),
        )
        .with_ibc(IbcAcceptingModule::default())
        .with_api(MockApiBech32::new(BECH32_PREFIX_HRP))
        .build(no_init)
}

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw721InstantiateMsg<DefaultOptionalCollectionExtensionMsg>,
) -> Result<Response, Cw721ContractError> {
    let contract = Cw721OnchainExtensions::default();
    contract.instantiate_with_version(deps, &env, &info, msg, "contract_name", "contract_version")
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw721ExecuteMsg<
        DefaultOptionalNftExtensionMsg,
        DefaultOptionalCollectionExtensionMsg,
        Empty,
    >,
) -> Result<Response, Cw721ContractError> {
    let contract = Cw721OnchainExtensions::default();
    contract.execute(deps, &env, &info, msg)
}

pub fn query(
    deps: Deps,
    env: Env,
    msg: Cw721QueryMsg<DefaultOptionalNftExtension, DefaultOptionalCollectionExtension, Empty>,
) -> Result<Binary, Cw721ContractError> {
    let contract = Cw721OnchainExtensions::default();
    contract.query(deps, &env, msg)
}

pub fn migrate(
    deps: DepsMut,
    env: Env,
    msg: Cw721MigrateMsg,
) -> Result<Response, Cw721ContractError> {
    let contract = Cw721OnchainExtensions::default();
    contract.migrate(deps, env, msg, "contract_name", "contract_version")
}

fn no_init(_router: &mut MockRouter, _api: &dyn Api, _storage: &mut dyn Storage) {}

fn cw721_base_latest_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query).with_migrate(migrate);
    Box::new(contract)
}

fn cw721_base_015_contract() -> Box<dyn Contract<Empty>> {
    use cw721_base_015 as v15;
    let contract = ContractWrapper::new(
        v15::entry::execute,
        v15::entry::instantiate,
        v15::entry::query,
    );
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

fn cw721_base_017_contract() -> Box<dyn Contract<Empty>> {
    use cw721_base_017 as v17;
    let contract = ContractWrapper::new(
        v17::entry::execute,
        v17::entry::instantiate,
        v17::entry::query,
    );
    Box::new(contract)
}

fn cw721_base_018_contract() -> Box<dyn Contract<Empty>> {
    use cw721_base_018 as v18;
    let contract = ContractWrapper::new(
        v18::entry::execute,
        v18::entry::instantiate,
        v18::entry::query,
    );
    Box::new(contract)
}

fn query_owner(querier: QuerierWrapper, cw721: &Addr, token_id: String) -> Addr {
    let resp: OwnerOfResponse = querier
        .query_wasm_smart(
            cw721,
            &Cw721QueryMsg::<
                DefaultOptionalNftExtension,
                DefaultOptionalCollectionExtension,
                Empty,
            >::OwnerOf {
                token_id,
                include_expired: None,
            },
        )
        .unwrap();
    Addr::unchecked(resp.owner)
}

fn query_nft_info(
    querier: QuerierWrapper,
    cw721: &Addr,
    token_id: String,
) -> NftInfoResponse<Option<NftExtension>> {
    querier
        .query_wasm_smart(
            cw721,
            &Cw721QueryMsg::<
                DefaultOptionalNftExtension,
                DefaultOptionalCollectionExtension,
                Empty,
            >::NftInfo {
                token_id,
            },
        )
        .unwrap()
}

fn query_all_collection_info(
    querier: QuerierWrapper,
    cw721: &Addr,
) -> ConfigResponse<DefaultOptionalCollectionExtension> {
    querier
        .query_wasm_smart(
            cw721,
            &Cw721QueryMsg::<
                DefaultOptionalNftExtension,
                DefaultOptionalCollectionExtension,
                Empty,
            >::GetConfig {},
        )
        .unwrap()
}

fn mint_transfer_and_burn(app: &mut MockApp, cw721: Addr, sender: Addr, token_id: String) {
    app.execute_contract(
        sender.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<Empty, Empty, Empty>::Mint {
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

    let burner = app.api().addr_make("burner");
    app.execute_contract(
        sender,
        cw721.clone(),
        &Cw721ExecuteMsg::<Empty, Empty, Empty>::TransferNft {
            recipient: burner.to_string(),
            token_id: token_id.clone(),
        },
        &[],
    )
    .unwrap();

    let owner = query_owner(app.wrap(), &cw721, token_id.clone());
    assert_eq!(owner, burner.to_string());

    app.execute_contract(
        burner,
        cw721,
        &Cw721ExecuteMsg::<Empty, Empty, Empty>::Burn { token_id },
        &[],
    )
    .unwrap();
}

#[test]
fn test_operator() {
    // --- setup ---
    let mut app = new();
    let admin = app.api().addr_make(ADMIN_ADDR);
    let creator = app.api().addr_make(CREATOR_ADDR);
    let minter = app.api().addr_make(MINTER_ADDR);
    let code_id = app.store_code(cw721_base_latest_contract());
    let other = app.api().addr_make(OTHER1_ADDR);
    let cw721 = app
        .instantiate_contract(
            code_id,
            other.clone(),
            &Cw721InstantiateMsg::<DefaultOptionalCollectionExtension> {
                name: "collection".to_string(),
                symbol: "symbol".to_string(),
                minter: Some(minter.to_string()),
                creator: Some(creator.to_string()),
                collection_info_extension: None,
                withdraw_address: None,
            },
            &[],
            "cw721-base",
            Some(admin.to_string()),
        )
        .unwrap();
    // mint
    let nft_owner = app.api().addr_make(NFT_OWNER_ADDR);
    app.execute_contract(
        minter,
        cw721.clone(),
        &Cw721ExecuteMsg::<Empty, Empty, Empty>::Mint {
            token_id: "1".to_string(),
            owner: nft_owner.to_string(),
            token_uri: Some("".to_string()), // empty uri, response contains attribute with value "empty"
            extension: Empty::default(),
        },
        &[],
    )
    .unwrap();

    // --- test operator/approve all ---
    // owner adds other user as operator using approve all
    app.execute_contract(
        nft_owner.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<Empty, Empty, Empty>::ApproveAll {
            operator: other.to_string(),
            expires: Some(Expiration::Never {}),
        },
        &[],
    )
    .unwrap();

    // transfer by operator
    app.execute_contract(
        other.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<Empty, Empty, Empty>::TransferNft {
            recipient: other.to_string(),
            token_id: "1".to_string(),
        },
        &[],
    )
    .unwrap();
    // check other is new owner
    let owner_response: OwnerOfResponse = app
        .wrap()
        .query_wasm_smart(
            &cw721,
            &Cw721QueryMsg::<
                DefaultOptionalNftExtension,
                DefaultOptionalCollectionExtension,
                Empty,
            >::OwnerOf {
                token_id: "1".to_string(),
                include_expired: None,
            },
        )
        .unwrap();
    assert_eq!(owner_response.owner, other.to_string());
    // check previous owner cant transfer
    let err: Cw721ContractError = app
        .execute_contract(
            nft_owner.clone(),
            cw721.clone(),
            &Cw721ExecuteMsg::<Empty, Empty, Empty>::TransferNft {
                recipient: other.to_string(),
                token_id: "1".to_string(),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, Cw721ContractError::Ownership(OwnershipError::NotOwner));

    // transfer back to previous owner
    app.execute_contract(
        other.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<Empty, Empty, Empty>::TransferNft {
            recipient: nft_owner.to_string(),
            token_id: "1".to_string(),
        },
        &[],
    )
    .unwrap();
    // check owner
    let owner_response: OwnerOfResponse = app
        .wrap()
        .query_wasm_smart(
            &cw721,
            &Cw721QueryMsg::<
                DefaultOptionalNftExtension,
                DefaultOptionalCollectionExtension,
                Empty,
            >::OwnerOf {
                token_id: "1".to_string(),
                include_expired: None,
            },
        )
        .unwrap();
    assert_eq!(owner_response.owner, nft_owner.to_string());

    // other user is still operator and can transfer!
    app.execute_contract(
        other.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<Empty, Empty, Empty>::TransferNft {
            recipient: other.to_string(),
            token_id: "1".to_string(),
        },
        &[],
    )
    .unwrap();
    // check other is new owner
    let owner_response: OwnerOfResponse = app
        .wrap()
        .query_wasm_smart(
            &cw721,
            &Cw721QueryMsg::<
                DefaultOptionalNftExtension,
                DefaultOptionalCollectionExtension,
                Empty,
            >::OwnerOf {
                token_id: "1".to_string(),
                include_expired: None,
            },
        )
        .unwrap();
    assert_eq!(owner_response.owner, other.to_string());

    // -- test revoke
    // transfer to previous owner
    app.execute_contract(
        other.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<Empty, Empty, Empty>::TransferNft {
            recipient: nft_owner.to_string(),
            token_id: "1".to_string(),
        },
        &[],
    )
    .unwrap();

    // revoke operator
    app.execute_contract(
        nft_owner,
        cw721.clone(),
        &Cw721ExecuteMsg::<Empty, Empty, Empty>::RevokeAll {
            operator: other.to_string(),
        },
        &[],
    )
    .unwrap();

    // other not operator anymore and cant send
    let err: Cw721ContractError = app
        .execute_contract(
            other.clone(),
            cw721,
            &Cw721ExecuteMsg::<Empty, Empty, Empty>::TransferNft {
                recipient: other.to_string(),
                token_id: "1".to_string(),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, Cw721ContractError::Ownership(OwnershipError::NotOwner));
}

/// Instantiates a 0.16 version of this contract and tests that tokens
/// can be minted, transferred, and burnred after migration.
#[test]
fn test_migration_legacy_to_latest() {
    // v0.15 migration by using existing minter addr
    {
        use cw721_base_015 as v15;
        let mut app = new();
        let admin = app.api().addr_make(ADMIN_ADDR);

        let code_id_016 = app.store_code(cw721_base_015_contract());
        let code_id_latest = app.store_code(cw721_base_latest_contract());

        let legacy_creator_and_minter = app.api().addr_make("legacy_creator_and_minter");

        let cw721 = app
            .instantiate_contract(
                code_id_016,
                legacy_creator_and_minter.clone(),
                &v15::InstantiateMsg {
                    name: "collection".to_string(),
                    symbol: "symbol".to_string(),
                    minter: legacy_creator_and_minter.to_string(),
                },
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap();

        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // migrate
        app.execute(
            admin,
            WasmMsg::Migrate {
                contract_addr: cw721.to_string(),
                new_code_id: code_id_latest,
                msg: to_json_binary(&Cw721MigrateMsg::WithUpdate {
                    minter: None,
                    creator: None,
                })
                .unwrap(),
            }
            .into(),
        )
        .unwrap();

        // non-minter user cant mint
        let other = Addr::unchecked(OTHER1_ADDR);
        let err: Cw721ContractError = app
            .execute_contract(
                other.clone(),
                cw721.clone(),
                &Cw721ExecuteMsg::<Empty, Empty, Empty>::Mint {
                    token_id: "1".to_string(),
                    owner: other.to_string(),
                    token_uri: Some("ipfs://new.uri".to_string()),
                    extension: Empty::default(),
                },
                &[],
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(err, Cw721ContractError::NotMinter {});

        // legacy minter can still mint
        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // check new mint query response works.
        #[allow(deprecated)]
        let m: MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(legacy_creator_and_minter.to_string()));

        // check that the new response is backwards compatable when minter
        // is not None.
        #[allow(deprecated)]
        let m: v15::MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, legacy_creator_and_minter.to_string());

        // check minter ownership query works
        let minter_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetMinterOwnership {},
            )
            .unwrap();
        assert_eq!(
            minter_ownership.owner,
            Some(legacy_creator_and_minter.clone())
        );

        // check creator ownership query works
        let creator_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetCreatorOwnership {},
            )
            .unwrap();
        assert_eq!(creator_ownership.owner, Some(legacy_creator_and_minter));
    }
    // v0.15 migration by providing new creator and minter addr
    {
        use cw721_base_015 as v15;
        let mut app = new();
        let admin = app.api().addr_make(ADMIN_ADDR);

        let code_id_015 = app.store_code(cw721_base_015_contract());
        let code_id_latest = app.store_code(cw721_base_latest_contract());

        let legacy_creator_and_minter = app.api().addr_make("legacy_creator_and_minter");

        let cw721 = app
            .instantiate_contract(
                code_id_015,
                legacy_creator_and_minter.clone(),
                &v15::InstantiateMsg {
                    name: "collection".to_string(),
                    symbol: "symbol".to_string(),
                    minter: legacy_creator_and_minter.to_string(),
                },
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap();

        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // migrate
        let creator = app.api().addr_make(CREATOR_ADDR);
        let minter = app.api().addr_make(MINTER_ADDR);
        app.execute(
            admin,
            WasmMsg::Migrate {
                contract_addr: cw721.to_string(),
                new_code_id: code_id_latest,
                msg: to_json_binary(&Cw721MigrateMsg::WithUpdate {
                    minter: Some(minter.to_string()),
                    creator: Some(creator.to_string()),
                })
                .unwrap(),
            }
            .into(),
        )
        .unwrap();

        // legacy minter user cant mint
        let err: Cw721ContractError = app
            .execute_contract(
                legacy_creator_and_minter.clone(),
                cw721.clone(),
                &Cw721ExecuteMsg::<Empty, Empty, Empty>::Mint {
                    token_id: "1".to_string(),
                    owner: legacy_creator_and_minter.to_string(),
                    token_uri: Some("ipfs://new.uri".to_string()),
                    extension: Empty::default(),
                },
                &[],
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(err, Cw721ContractError::NotMinter {});

        // new minter can mint
        mint_transfer_and_burn(&mut app, cw721.clone(), minter.clone(), "1".to_string());

        // check new mint query response works.
        #[allow(deprecated)]
        let m: MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(minter.to_string()));

        // check that the new response is backwards compatable when minter
        // is not None.
        #[allow(deprecated)]
        let m: v15::MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, minter.to_string());

        // check minter ownership query works
        let minter_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetMinterOwnership {},
            )
            .unwrap();
        assert_eq!(minter_ownership.owner, Some(minter));

        // check creator ownership query works
        let creator = app.api().addr_make(CREATOR_ADDR);
        let creator_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetCreatorOwnership {},
            )
            .unwrap();
        assert_eq!(creator_ownership.owner, Some(creator));
    }
    // v0.16 migration by using existing minter addr
    {
        use cw721_base_016 as v16;
        let mut app = new();
        let admin = app.api().addr_make(ADMIN_ADDR);

        let code_id_016 = app.store_code(cw721_base_016_contract());
        let code_id_latest = app.store_code(cw721_base_latest_contract());

        let legacy_creator_and_minter = app.api().addr_make("legacy_creator_and_minter");

        let cw721 = app
            .instantiate_contract(
                code_id_016,
                legacy_creator_and_minter.clone(),
                &v16::InstantiateMsg {
                    name: "collection".to_string(),
                    symbol: "symbol".to_string(),
                    minter: legacy_creator_and_minter.to_string(),
                },
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap();

        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // migrate
        app.execute(
            admin,
            WasmMsg::Migrate {
                contract_addr: cw721.to_string(),
                new_code_id: code_id_latest,
                msg: to_json_binary(&Cw721MigrateMsg::WithUpdate {
                    minter: None,
                    creator: None,
                })
                .unwrap(),
            }
            .into(),
        )
        .unwrap();

        // non-minter user cant mint
        let other = Addr::unchecked(OTHER1_ADDR);
        let err: Cw721ContractError = app
            .execute_contract(
                other.clone(),
                cw721.clone(),
                &Cw721ExecuteMsg::<Empty, Empty, Empty>::Mint {
                    token_id: "1".to_string(),
                    owner: other.to_string(),
                    token_uri: Some("ipfs://new.uri".to_string()),
                    extension: Empty::default(),
                },
                &[],
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(err, Cw721ContractError::NotMinter {});

        // legacy minter can still mint
        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // check new mint query response works.
        #[allow(deprecated)]
        let m: MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(legacy_creator_and_minter.to_string()));

        // check that the new response is backwards compatable when minter
        // is not None.
        #[allow(deprecated)]
        let m: v16::MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, legacy_creator_and_minter.to_string());

        // check minter ownership query works
        let minter_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetMinterOwnership {},
            )
            .unwrap();
        assert_eq!(
            minter_ownership.owner,
            Some(legacy_creator_and_minter.clone())
        );

        // check creator ownership query works
        let creator_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetCreatorOwnership {},
            )
            .unwrap();
        assert_eq!(creator_ownership.owner, Some(legacy_creator_and_minter));
    }
    // v0.16 migration by providing new creator and minter addr
    {
        use cw721_base_016 as v16;
        let mut app = new();
        let admin = app.api().addr_make(ADMIN_ADDR);

        let code_id_016 = app.store_code(cw721_base_016_contract());
        let code_id_latest = app.store_code(cw721_base_latest_contract());

        let legacy_creator_and_minter = app.api().addr_make("legacy_creator_and_minter");

        let cw721 = app
            .instantiate_contract(
                code_id_016,
                legacy_creator_and_minter.clone(),
                &v16::InstantiateMsg {
                    name: "collection".to_string(),
                    symbol: "symbol".to_string(),
                    minter: legacy_creator_and_minter.to_string(),
                },
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap();

        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // migrate
        let creator = app.api().addr_make(CREATOR_ADDR);
        let minter = app.api().addr_make(MINTER_ADDR);
        app.execute(
            admin,
            WasmMsg::Migrate {
                contract_addr: cw721.to_string(),
                new_code_id: code_id_latest,
                msg: to_json_binary(&Cw721MigrateMsg::WithUpdate {
                    minter: Some(minter.to_string()),
                    creator: Some(creator.to_string()),
                })
                .unwrap(),
            }
            .into(),
        )
        .unwrap();

        // legacy minter user cant mint
        let err: Cw721ContractError = app
            .execute_contract(
                legacy_creator_and_minter.clone(),
                cw721.clone(),
                &Cw721ExecuteMsg::<Empty, Empty, Empty>::Mint {
                    token_id: "1".to_string(),
                    owner: legacy_creator_and_minter.to_string(),
                    token_uri: Some("ipfs://new.uri".to_string()),
                    extension: Empty::default(),
                },
                &[],
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(err, Cw721ContractError::NotMinter {});

        // new minter can mint
        mint_transfer_and_burn(&mut app, cw721.clone(), minter.clone(), "1".to_string());

        // check new mint query response works.
        #[allow(deprecated)]
        let m: MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(minter.to_string()));

        // check that the new response is backwards compatable when minter
        // is not None.
        #[allow(deprecated)]
        let m: v16::MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, minter.to_string());

        // check minter ownership query works
        let minter_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetMinterOwnership {},
            )
            .unwrap();
        assert_eq!(minter_ownership.owner, Some(minter));

        // check creator ownership query works
        let creator = app.api().addr_make(CREATOR_ADDR);
        let creator_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetCreatorOwnership {},
            )
            .unwrap();
        assert_eq!(creator_ownership.owner, Some(creator));
    }
    // v0.17 migration by using existing minter addr
    {
        use cw721_base_017 as v17;
        let mut app = new();
        let admin = app.api().addr_make(ADMIN_ADDR);

        let code_id_017 = app.store_code(cw721_base_017_contract());
        let code_id_latest = app.store_code(cw721_base_latest_contract());

        let legacy_creator_and_minter = app.api().addr_make("legacy_creator_and_minter");

        let cw721 = app
            .instantiate_contract(
                code_id_017,
                legacy_creator_and_minter.clone(),
                &v17::InstantiateMsg {
                    name: "collection".to_string(),
                    symbol: "symbol".to_string(),
                    minter: legacy_creator_and_minter.to_string(),
                },
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap();

        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // migrate
        app.execute(
            admin,
            WasmMsg::Migrate {
                contract_addr: cw721.to_string(),
                new_code_id: code_id_latest,
                msg: to_json_binary(&Cw721MigrateMsg::WithUpdate {
                    minter: None,
                    creator: None,
                })
                .unwrap(),
            }
            .into(),
        )
        .unwrap();

        // non-minter user cant mint
        let other = Addr::unchecked(OTHER1_ADDR);
        let err: Cw721ContractError = app
            .execute_contract(
                other.clone(),
                cw721.clone(),
                &Cw721ExecuteMsg::<Empty, Empty, Empty>::Mint {
                    token_id: "1".to_string(),
                    owner: other.to_string(),
                    token_uri: Some("ipfs://new.uri".to_string()),
                    extension: Empty::default(),
                },
                &[],
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(err, Cw721ContractError::NotMinter {});

        // legacy minter can still mint
        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // check new mint query response works.
        #[allow(deprecated)]
        let m: MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(legacy_creator_and_minter.to_string()));

        // check that the new response is backwards compatable when minter
        // is not None.
        #[allow(deprecated)]
        let m: v17::MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(legacy_creator_and_minter.to_string()));

        // check minter ownership query works
        let minter_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetMinterOwnership {},
            )
            .unwrap();
        assert_eq!(
            minter_ownership.owner,
            Some(legacy_creator_and_minter.clone())
        );

        // check creator ownership query works
        let creator_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetCreatorOwnership {},
            )
            .unwrap();
        assert_eq!(creator_ownership.owner, Some(legacy_creator_and_minter));
    }
    // v0.17 migration by providing new creator and minter addr
    {
        use cw721_base_017 as v17;
        let mut app = new();
        let admin = app.api().addr_make(ADMIN_ADDR);

        let code_id_017 = app.store_code(cw721_base_017_contract());
        let code_id_latest = app.store_code(cw721_base_latest_contract());

        let legacy_creator_and_minter = app.api().addr_make("legacy_creator_and_minter");

        let cw721 = app
            .instantiate_contract(
                code_id_017,
                legacy_creator_and_minter.clone(),
                &v17::InstantiateMsg {
                    name: "collection".to_string(),
                    symbol: "symbol".to_string(),
                    minter: legacy_creator_and_minter.to_string(),
                },
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap();

        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // migrate
        let creator = app.api().addr_make(CREATOR_ADDR);
        let minter = app.api().addr_make(MINTER_ADDR);
        app.execute(
            admin,
            WasmMsg::Migrate {
                contract_addr: cw721.to_string(),
                new_code_id: code_id_latest,
                msg: to_json_binary(&Cw721MigrateMsg::WithUpdate {
                    minter: Some(minter.to_string()),
                    creator: Some(creator.to_string()),
                })
                .unwrap(),
            }
            .into(),
        )
        .unwrap();

        // legacy minter user cant mint
        let err: Cw721ContractError = app
            .execute_contract(
                legacy_creator_and_minter.clone(),
                cw721.clone(),
                &Cw721ExecuteMsg::<Empty, Empty, Empty>::Mint {
                    token_id: "1".to_string(),
                    owner: legacy_creator_and_minter.to_string(),
                    token_uri: Some("ipfs://new.uri".to_string()),
                    extension: Empty::default(),
                },
                &[],
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(err, Cw721ContractError::NotMinter {});

        // new minter can mint
        mint_transfer_and_burn(&mut app, cw721.clone(), minter.clone(), "1".to_string());

        // check new mint query response works.
        #[allow(deprecated)]
        let m: MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(minter.to_string()));

        // check that the new response is backwards compatable when minter
        // is not None.
        #[allow(deprecated)]
        let m: v17::MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(minter.to_string()));

        // check minter ownership query works
        let minter_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetMinterOwnership {},
            )
            .unwrap();
        assert_eq!(minter_ownership.owner, Some(minter));

        // check creator ownership query works
        let creator = app.api().addr_make(CREATOR_ADDR);
        let creator_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetCreatorOwnership {},
            )
            .unwrap();
        assert_eq!(creator_ownership.owner, Some(creator));
    }
    // v0.18 migration by using existing minter addr
    {
        use cw721_base_018 as v18;
        let mut app = new();
        let admin = app.api().addr_make(ADMIN_ADDR);

        let code_id_018 = app.store_code(cw721_base_018_contract());
        let code_id_latest = app.store_code(cw721_base_latest_contract());

        let legacy_creator_and_minter = app.api().addr_make("legacy_creator_and_minter");

        let cw721 = app
            .instantiate_contract(
                code_id_018,
                legacy_creator_and_minter.clone(),
                &v18::InstantiateMsg {
                    name: "collection".to_string(),
                    symbol: "symbol".to_string(),
                    minter: legacy_creator_and_minter.to_string(),
                },
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap();

        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // migrate
        app.execute(
            admin,
            WasmMsg::Migrate {
                contract_addr: cw721.to_string(),
                new_code_id: code_id_latest,
                msg: to_json_binary(&Cw721MigrateMsg::WithUpdate {
                    minter: None,
                    creator: None,
                })
                .unwrap(),
            }
            .into(),
        )
        .unwrap();

        // non-minter user cant mint
        let other = Addr::unchecked(OTHER1_ADDR);
        let err: Cw721ContractError = app
            .execute_contract(
                other.clone(),
                cw721.clone(),
                &Cw721ExecuteMsg::<Empty, Empty, Empty>::Mint {
                    token_id: "1".to_string(),
                    owner: other.to_string(),
                    token_uri: Some("ipfs://new.uri".to_string()),
                    extension: Empty::default(),
                },
                &[],
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(err, Cw721ContractError::NotMinter {});

        // legacy minter can still mint
        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // check new mint query response works.
        #[allow(deprecated)]
        let m: MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(legacy_creator_and_minter.to_string()));

        // check that the new response is backwards compatable when minter
        // is not None.
        #[allow(deprecated)]
        let m: v18::MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(legacy_creator_and_minter.to_string()));

        // check minter ownership query works
        let minter_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetMinterOwnership {},
            )
            .unwrap();
        assert_eq!(
            minter_ownership.owner,
            Some(legacy_creator_and_minter.clone())
        );

        // check creator ownership query works
        let creator_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetCreatorOwnership {},
            )
            .unwrap();
        assert_eq!(creator_ownership.owner, Some(legacy_creator_and_minter));
    }
    // v0.18 migration by providing new creator and minter addr
    {
        use cw721_base_018 as v18;
        let mut app = new();
        let admin = app.api().addr_make(ADMIN_ADDR);

        let code_id_018 = app.store_code(cw721_base_018_contract());
        let code_id_latest = app.store_code(cw721_base_latest_contract());

        let legacy_creator_and_minter = app.api().addr_make("legacy_creator_and_minter");

        let cw721 = app
            .instantiate_contract(
                code_id_018,
                legacy_creator_and_minter.clone(),
                &v18::InstantiateMsg {
                    name: "collection".to_string(),
                    symbol: "symbol".to_string(),
                    minter: legacy_creator_and_minter.to_string(),
                },
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap();

        mint_transfer_and_burn(
            &mut app,
            cw721.clone(),
            legacy_creator_and_minter.clone(),
            "1".to_string(),
        );

        // migrate
        let creator = app.api().addr_make(CREATOR_ADDR);
        let minter = app.api().addr_make(MINTER_ADDR);
        app.execute(
            admin,
            WasmMsg::Migrate {
                contract_addr: cw721.to_string(),
                new_code_id: code_id_latest,
                msg: to_json_binary(&Cw721MigrateMsg::WithUpdate {
                    minter: Some(minter.to_string()),
                    creator: Some(creator.to_string()),
                })
                .unwrap(),
            }
            .into(),
        )
        .unwrap();

        // legacy minter user cant mint
        let err: Cw721ContractError = app
            .execute_contract(
                legacy_creator_and_minter.clone(),
                cw721.clone(),
                &Cw721ExecuteMsg::<Empty, Empty, Empty>::Mint {
                    token_id: "1".to_string(),
                    owner: legacy_creator_and_minter.to_string(),
                    token_uri: Some("ipfs://new.uri".to_string()),
                    extension: Empty::default(),
                },
                &[],
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(err, Cw721ContractError::NotMinter {});

        // new minter can mint
        mint_transfer_and_burn(&mut app, cw721.clone(), minter.clone(), "1".to_string());

        // check new mint query response works.
        #[allow(deprecated)]
        let m: MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(minter.to_string()));

        // check that the new response is backwards compatable when minter
        // is not None.
        #[allow(deprecated)]
        let m: v18::MinterResponse = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::Minter {},
            )
            .unwrap();
        assert_eq!(m.minter, Some(minter.to_string()));

        // check minter ownership query works
        let minter_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetMinterOwnership {},
            )
            .unwrap();
        assert_eq!(minter_ownership.owner, Some(minter));

        // check creator ownership query works
        let creator = app.api().addr_make(CREATOR_ADDR);
        let creator_ownership: Ownership<Addr> = app
            .wrap()
            .query_wasm_smart(
                &cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetCreatorOwnership {},
            )
            .unwrap();
        assert_eq!(creator_ownership.owner, Some(creator));
    }
}

#[test]
fn test_instantiate() {
    let mut app = new();
    let admin = app.api().addr_make(ADMIN_ADDR);
    let minter = app.api().addr_make(MINTER_ADDR);
    let creator = app.api().addr_make(CREATOR_ADDR);
    let payment_address = app.api().addr_make(OTHER1_ADDR);
    let withdraw_addr = app.api().addr_make(OTHER2_ADDR);
    let init_msg = Cw721InstantiateMsg {
        name: "collection".to_string(),
        symbol: "symbol".to_string(),
        minter: Some(minter.to_string()),
        creator: Some(creator.to_string()),
        withdraw_address: Some(withdraw_addr.to_string()),
        collection_info_extension: Some(CollectionExtensionMsg {
            description: Some("description".to_string()),
            image: Some("ipfs://ark.pass".to_string()),
            explicit_content: Some(false),
            external_link: Some("https://interchain.arkprotocol.io".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(42)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: payment_address.to_string(),
                share: Decimal::bps(1000),
            }),
        }),
    };
    // test case: happy path
    {
        let code_id_latest = app.store_code(cw721_base_latest_contract());
        let cw721 = app
            .instantiate_contract(
                code_id_latest,
                admin.clone(),
                &init_msg.clone(),
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap();

        // assert withdraw address
        let withdraw_addr_result: Option<String> = app
            .wrap()
            .query_wasm_smart(
                cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetWithdrawAddress {},
            )
            .unwrap();
        assert_eq!(withdraw_addr_result, Some(withdraw_addr.to_string()));
    }
    // test case: invalid addresses
    {
        // invalid creator
        let code_id_latest = app.store_code(cw721_base_latest_contract());
        let mut invalid_init_msg = init_msg.clone();
        invalid_init_msg.creator = Some("invalid".to_string());
        let error: Cw721ContractError = app
            .instantiate_contract(
                code_id_latest,
                admin.clone(),
                &invalid_init_msg.clone(),
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(
            error,
            Cw721ContractError::Std(StdError::generic_err("Invalid input: invalid"))
        );
        // invalid minter
        let mut invalid_init_msg = init_msg.clone();
        invalid_init_msg.minter = Some("invalid".to_string());
        let error: Cw721ContractError = app
            .instantiate_contract(
                code_id_latest,
                admin.clone(),
                &invalid_init_msg.clone(),
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(
            error,
            Cw721ContractError::Std(StdError::generic_err("Invalid input: invalid"))
        );
        // invalid withdraw addr
        let mut invalid_init_msg = init_msg.clone();
        invalid_init_msg.withdraw_address = Some("invalid".to_string());
        let error: Cw721ContractError = app
            .instantiate_contract(
                code_id_latest,
                admin.clone(),
                &invalid_init_msg.clone(),
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(
            error,
            Cw721ContractError::Std(StdError::generic_err("Invalid input: invalid"))
        );
        // invalid payment addr
        let mut invalid_init_msg = init_msg.clone();
        invalid_init_msg.collection_info_extension = Some(CollectionExtensionMsg {
            description: Some("description".to_string()),
            image: Some("ipfs://ark.pass".to_string()),
            explicit_content: Some(false),
            external_link: Some("https://interchain.arkprotocol.io".to_string()),
            start_trading_time: Some(Timestamp::from_seconds(42)),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "invalid".to_string(),
                share: Decimal::bps(1000),
            }),
        });
        let error: Cw721ContractError = app
            .instantiate_contract(
                code_id_latest,
                admin.clone(),
                &invalid_init_msg.clone(),
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(
            error,
            Cw721ContractError::Std(StdError::generic_err("Invalid input: invalid"))
        );
    }
    // test case: backward compatibility using instantiate msg from a 0.16 version on latest contract.
    // This ensures existing 3rd party contracts doesnt need to update as well.
    {
        use cw721_base_016 as v16;
        let code_id_latest = app.store_code(cw721_base_latest_contract());

        let cw721 = app
            .instantiate_contract(
                code_id_latest,
                admin.clone(),
                &v16::InstantiateMsg {
                    name: "collection".to_string(),
                    symbol: "symbol".to_string(),
                    minter: admin.to_string(),
                },
                &[],
                "cw721-base",
                Some(admin.to_string()),
            )
            .unwrap();

        // assert withdraw address is None
        let withdraw_addr: Option<String> = app
            .wrap()
            .query_wasm_smart(
                cw721,
                &Cw721QueryMsg::<
                    DefaultOptionalNftExtension,
                    DefaultOptionalCollectionExtension,
                    Empty,
                >::GetWithdrawAddress {},
            )
            .unwrap();
        assert!(withdraw_addr.is_none());
    }
}

#[test]
fn test_update_nft_metadata() {
    // --- setup ---
    let mut app = new();
    let admin = app.api().addr_make(ADMIN_ADDR);
    let code_id = app.store_code(cw721_base_latest_contract());
    let creator = app.api().addr_make(CREATOR_ADDR);
    let minter_addr = app.api().addr_make(MINTER_ADDR);
    let cw721 = app
        .instantiate_contract(
            code_id,
            creator.clone(),
            &Cw721InstantiateMsg::<DefaultOptionalCollectionExtension> {
                name: "collection".to_string(),
                symbol: "symbol".to_string(),
                minter: Some(minter_addr.to_string()),
                creator: None, // in case of none, sender is creator
                collection_info_extension: None,
                withdraw_address: None,
            },
            &[],
            "cw721-base",
            Some(admin.to_string()),
        )
        .unwrap();
    // mint
    let nft_owner = app.api().addr_make(NFT_OWNER_ADDR);
    let nft_metadata_msg = NftExtensionMsg {
        image: Some("ipfs://foo.bar/image.png".to_string()),
        image_data: Some("image data".to_string()),
        external_url: Some("https://github.com".to_string()),
        description: Some("description".to_string()),
        name: Some("name".to_string()),
        attributes: Some(vec![Trait {
            trait_type: "trait_type".to_string(),
            value: "value".to_string(),
            display_type: Some("display_type".to_string()),
        }]),
        background_color: Some("background_color".to_string()),
        animation_url: Some("ssl://animation_url".to_string()),
        youtube_url: Some("file://youtube_url".to_string()),
    };
    app.execute_contract(
        minter_addr,
        cw721.clone(),
        &Cw721ExecuteMsg::<
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
        >::Mint {
            token_id: "1".to_string(),
            owner: nft_owner.to_string(),
            token_uri: Some("ipfs://foo.bar/metadata.json".to_string()),
            extension: Some(nft_metadata_msg.clone()),
        },
        &[],
    )
    .unwrap();

    // check nft info
    let nft_info = query_nft_info(app.wrap(), &cw721, "1".to_string());
    assert_eq!(
        nft_info.token_uri,
        Some("ipfs://foo.bar/metadata.json".to_string())
    );
    assert_eq!(
        nft_info.extension,
        Some(NftExtension {
            image: Some("ipfs://foo.bar/image.png".to_string()),
            image_data: Some("image data".to_string()),
            external_url: Some("https://github.com".to_string()),
            description: Some("description".to_string()),
            name: Some("name".to_string()),
            attributes: Some(vec![Trait {
                trait_type: "trait_type".to_string(),
                value: "value".to_string(),
                display_type: Some("display_type".to_string()),
            }]),
            background_color: Some("background_color".to_string()),
            animation_url: Some("ssl://animation_url".to_string()),
            youtube_url: Some("file://youtube_url".to_string()),
        })
    );

    // nft owner cant update - only creator is allowed
    let err: Cw721ContractError = app
        .execute_contract(
            nft_owner,
            cw721.clone(),
            &Cw721ExecuteMsg::<
                DefaultOptionalNftExtensionMsg,
                DefaultOptionalCollectionExtensionMsg,
                Empty,
            >::UpdateNftInfo {
                token_id: "1".to_string(),
                token_uri: None,
                extension: Some(NftExtensionMsg {
                    name: Some("new name".to_string()),
                    description: Some("new description".to_string()),
                    image: None,
                    image_data: None,
                    external_url: None,
                    attributes: None,
                    background_color: None,
                    animation_url: None,
                    youtube_url: None,
                }),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, Cw721ContractError::NotCreator {});

    // update invalid token uri
    let err: Cw721ContractError = app
        .execute_contract(
            creator.clone(),
            cw721.clone(),
            &Cw721ExecuteMsg::<
                DefaultOptionalNftExtensionMsg,
                DefaultOptionalCollectionExtensionMsg,
                Empty,
            >::UpdateNftInfo {
                token_id: "1".to_string(),
                token_uri: Some("invalid".to_string()),
                extension: Some(NftExtensionMsg {
                    name: None,
                    description: None,
                    image: None,
                    image_data: None,
                    external_url: None,
                    attributes: None,
                    background_color: None,
                    animation_url: None,
                    youtube_url: None,
                }),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(
        err,
        Cw721ContractError::ParseError(ParseError::RelativeUrlWithoutBase)
    );

    // invalid image URL
    let err: Cw721ContractError = app
        .execute_contract(
            creator.clone(),
            cw721.clone(),
            &Cw721ExecuteMsg::<
                DefaultOptionalNftExtensionMsg,
                DefaultOptionalCollectionExtensionMsg,
                Empty,
            >::UpdateNftInfo {
                token_id: "1".to_string(),
                token_uri: None,
                extension: Some(NftExtensionMsg {
                    name: None,
                    description: None,
                    image: Some("invalid".to_string()),
                    image_data: None,
                    external_url: None,
                    attributes: None,
                    background_color: None,
                    animation_url: None,
                    youtube_url: None,
                }),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(
        err,
        Cw721ContractError::ParseError(ParseError::RelativeUrlWithoutBase)
    );

    // invalid external url
    let err: Cw721ContractError = app
        .execute_contract(
            creator.clone(),
            cw721.clone(),
            &Cw721ExecuteMsg::<
                DefaultOptionalNftExtensionMsg,
                DefaultOptionalCollectionExtensionMsg,
                Empty,
            >::UpdateNftInfo {
                token_id: "1".to_string(),
                token_uri: None,
                extension: Some(NftExtensionMsg {
                    name: None,
                    description: None,
                    image: None,
                    image_data: None,
                    external_url: Some("invalid".to_string()),
                    attributes: None,
                    background_color: None,
                    animation_url: None,
                    youtube_url: None,
                }),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(
        err,
        Cw721ContractError::ParseError(ParseError::RelativeUrlWithoutBase)
    );

    // invalid animation url
    let err: Cw721ContractError = app
        .execute_contract(
            creator.clone(),
            cw721.clone(),
            &Cw721ExecuteMsg::<
                DefaultOptionalNftExtensionMsg,
                DefaultOptionalCollectionExtensionMsg,
                Empty,
            >::UpdateNftInfo {
                token_id: "1".to_string(),
                token_uri: None,
                extension: Some(NftExtensionMsg {
                    name: None,
                    description: None,
                    image: None,
                    image_data: None,
                    external_url: None,
                    attributes: None,
                    background_color: None,
                    animation_url: Some("invalid".to_string()),
                    youtube_url: None,
                }),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(
        err,
        Cw721ContractError::ParseError(ParseError::RelativeUrlWithoutBase)
    );

    // invalid youtube url
    let err: Cw721ContractError = app
        .execute_contract(
            creator.clone(),
            cw721.clone(),
            &Cw721ExecuteMsg::<
                DefaultOptionalNftExtensionMsg,
                DefaultOptionalCollectionExtensionMsg,
                Empty,
            >::UpdateNftInfo {
                token_id: "1".to_string(),
                token_uri: None,
                extension: Some(NftExtensionMsg {
                    name: None,
                    description: None,
                    image: None,
                    image_data: None,
                    external_url: None,
                    attributes: None,
                    background_color: None,
                    animation_url: None,
                    youtube_url: Some("invalid".to_string()),
                }),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(
        err,
        Cw721ContractError::ParseError(ParseError::RelativeUrlWithoutBase)
    );

    // no image data (empty)
    app.execute_contract(
        creator.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
        >::UpdateNftInfo {
            token_id: "1".to_string(),
            token_uri: None,
            extension: Some(NftExtensionMsg {
                name: None,
                description: None,
                image: None,
                image_data: Some("".to_string()),
                external_url: None,
                attributes: None,
                background_color: None,
                animation_url: None,
                youtube_url: None,
            }),
        },
        &[],
    )
    .unwrap();
    // check nft info
    let nft_info = query_nft_info(app.wrap(), &cw721, "1".to_string());
    assert_eq!(
        nft_info.token_uri,
        Some("ipfs://foo.bar/metadata.json".to_string())
    );
    assert_eq!(
        nft_info.extension,
        Some(NftExtension {
            image: Some("ipfs://foo.bar/image.png".to_string()),
            image_data: None,
            external_url: Some("https://github.com".to_string()),
            description: Some("description".to_string()),
            name: Some("name".to_string()),
            attributes: Some(vec![Trait {
                trait_type: "trait_type".to_string(),
                value: "value".to_string(),
                display_type: Some("display_type".to_string()),
            }]),
            background_color: Some("background_color".to_string()),
            animation_url: Some("ssl://animation_url".to_string()),
            youtube_url: Some("file://youtube_url".to_string()),
        })
    );

    // no description (empty)
    app.execute_contract(
        creator.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
        >::UpdateNftInfo {
            token_id: "1".to_string(),
            token_uri: None,
            extension: Some(NftExtensionMsg {
                name: None,
                description: Some("".to_string()),
                image: None,
                image_data: None,
                external_url: None,
                attributes: None,
                background_color: None,
                animation_url: None,
                youtube_url: None,
            }),
        },
        &[],
    )
    .unwrap();
    // check nft info
    let nft_info = query_nft_info(app.wrap(), &cw721, "1".to_string());
    assert_eq!(
        nft_info.token_uri,
        Some("ipfs://foo.bar/metadata.json".to_string())
    );
    assert_eq!(
        nft_info.extension,
        Some(NftExtension {
            image: Some("ipfs://foo.bar/image.png".to_string()),
            image_data: None,
            external_url: Some("https://github.com".to_string()),
            description: None,
            name: Some("name".to_string()),
            attributes: Some(vec![Trait {
                trait_type: "trait_type".to_string(),
                value: "value".to_string(),
                display_type: Some("display_type".to_string()),
            }]),
            background_color: Some("background_color".to_string()),
            animation_url: Some("ssl://animation_url".to_string()),
            youtube_url: Some("file://youtube_url".to_string()),
        })
    );

    // no metadata name (empty)
    app.execute_contract(
        creator.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
        >::UpdateNftInfo {
            token_id: "1".to_string(),
            token_uri: None,
            extension: Some(NftExtensionMsg {
                name: Some("".to_string()),
                description: None,
                image: None,
                image_data: None,
                external_url: None,
                attributes: None,
                background_color: None,
                animation_url: None,
                youtube_url: None,
            }),
        },
        &[],
    )
    .unwrap();
    // check nft info
    let nft_info = query_nft_info(app.wrap(), &cw721, "1".to_string());
    assert_eq!(
        nft_info.token_uri,
        Some("ipfs://foo.bar/metadata.json".to_string())
    );
    assert_eq!(
        nft_info.extension,
        Some(NftExtension {
            image: Some("ipfs://foo.bar/image.png".to_string()),
            image_data: None,
            external_url: Some("https://github.com".to_string()),
            description: None,
            name: None,
            attributes: Some(vec![Trait {
                trait_type: "trait_type".to_string(),
                value: "value".to_string(),
                display_type: Some("display_type".to_string()),
            }]),
            background_color: Some("background_color".to_string()),
            animation_url: Some("ssl://animation_url".to_string()),
            youtube_url: Some("file://youtube_url".to_string()),
        })
    );

    // no background color (empty)
    app.execute_contract(
        creator.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
        >::UpdateNftInfo {
            token_id: "1".to_string(),
            token_uri: None,
            extension: Some(NftExtensionMsg {
                name: None,
                description: None,
                image: None,
                image_data: None,
                external_url: None,
                attributes: None,
                background_color: Some("".to_string()),
                animation_url: None,
                youtube_url: None,
            }),
        },
        &[],
    )
    .unwrap();
    // check nft info
    let nft_info = query_nft_info(app.wrap(), &cw721, "1".to_string());
    assert_eq!(
        nft_info.token_uri,
        Some("ipfs://foo.bar/metadata.json".to_string())
    );
    assert_eq!(
        nft_info.extension,
        Some(NftExtension {
            image: Some("ipfs://foo.bar/image.png".to_string()),
            image_data: None,
            external_url: Some("https://github.com".to_string()),
            description: None,
            name: None,
            attributes: Some(vec![Trait {
                trait_type: "trait_type".to_string(),
                value: "value".to_string(),
                display_type: Some("display_type".to_string()),
            }]),
            background_color: None,
            animation_url: Some("ssl://animation_url".to_string()),
            youtube_url: Some("file://youtube_url".to_string()),
        })
    );

    // invalid trait type (empty)
    let err: Cw721ContractError = app
        .execute_contract(
            creator.clone(),
            cw721.clone(),
            &Cw721ExecuteMsg::<
                DefaultOptionalNftExtensionMsg,
                DefaultOptionalCollectionExtensionMsg,
                Empty,
            >::UpdateNftInfo {
                token_id: "1".to_string(),
                token_uri: None,
                extension: Some(NftExtensionMsg {
                    name: None,
                    description: None,
                    image: None,
                    image_data: None,
                    external_url: None,
                    attributes: Some(vec![Trait {
                        trait_type: "".to_string(),
                        value: "value".to_string(),
                        display_type: Some("display_type".to_string()),
                    }]),
                    background_color: None,
                    animation_url: None,
                    youtube_url: None,
                }),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, Cw721ContractError::TraitTypeEmpty {});

    // invalid trait value (empty)
    let err: Cw721ContractError = app
        .execute_contract(
            creator.clone(),
            cw721.clone(),
            &Cw721ExecuteMsg::<
                DefaultOptionalNftExtensionMsg,
                DefaultOptionalCollectionExtensionMsg,
                Empty,
            >::UpdateNftInfo {
                token_id: "1".to_string(),
                token_uri: None,
                extension: Some(NftExtensionMsg {
                    name: None,
                    description: None,
                    image: None,
                    image_data: None,
                    external_url: None,
                    attributes: Some(vec![Trait {
                        trait_type: "trait_type".to_string(),
                        value: "".to_string(),
                        display_type: Some("display_type".to_string()),
                    }]),
                    background_color: None,
                    animation_url: None,
                    youtube_url: None,
                }),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, Cw721ContractError::TraitValueEmpty {});

    // invalid trait display type (empty)
    let err: Cw721ContractError = app
        .execute_contract(
            creator.clone(),
            cw721.clone(),
            &Cw721ExecuteMsg::<
                DefaultOptionalNftExtensionMsg,
                DefaultOptionalCollectionExtensionMsg,
                Empty,
            >::UpdateNftInfo {
                token_id: "1".to_string(),
                token_uri: None,
                extension: Some(NftExtensionMsg {
                    name: None,
                    description: None,
                    image: None,
                    image_data: None,
                    external_url: None,
                    attributes: Some(vec![Trait {
                        trait_type: "trait_type".to_string(),
                        value: "value".to_string(),
                        display_type: Some("".to_string()),
                    }]),
                    background_color: None,
                    animation_url: None,
                    youtube_url: None,
                }),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, Cw721ContractError::TraitDisplayTypeEmpty {});

    // proper update
    let new_nft_metadata_msg = NftExtensionMsg {
        image: None, // set to none to ensure it is unchanged
        image_data: Some("image data2".to_string()),
        external_url: Some("https://github.com2".to_string()),
        description: Some("description2".to_string()),
        name: Some("name2".to_string()),
        attributes: Some(vec![Trait {
            trait_type: "trait_type2".to_string(),
            value: "value2".to_string(),
            display_type: Some("display_type2".to_string()),
        }]),
        background_color: Some("background_color2".to_string()),
        animation_url: Some("ssl://animation_url2".to_string()),
        youtube_url: Some("file://youtube_url2".to_string()),
    };
    app.execute_contract(
        creator,
        cw721.clone(),
        &Cw721ExecuteMsg::<
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
        >::UpdateNftInfo {
            token_id: "1".to_string(),
            token_uri: Some("ipfs://foo.bar/metadata2.json".to_string()),
            extension: Some(new_nft_metadata_msg.clone()),
        },
        &[],
    )
    .unwrap();
    // check token uri and extension
    let nft_info = query_nft_info(app.wrap(), &cw721, "1".to_string());
    assert_eq!(
        nft_info.token_uri,
        Some("ipfs://foo.bar/metadata2.json".to_string())
    );
    assert_eq!(
        nft_info.extension,
        Some(NftExtension {
            image: Some("ipfs://foo.bar/image.png".to_string()), // this is unchanged
            image_data: Some("image data2".to_string()),
            external_url: Some("https://github.com2".to_string()),
            description: Some("description2".to_string()),
            name: Some("name2".to_string()),
            attributes: Some(vec![Trait {
                trait_type: "trait_type2".to_string(),
                value: "value2".to_string(),
                display_type: Some("display_type2".to_string()),
            }]),
            background_color: Some("background_color2".to_string()),
            animation_url: Some("ssl://animation_url2".to_string()),
            youtube_url: Some("file://youtube_url2".to_string()),
        })
    );
    // check num tokens
    let num_tokens: NumTokensResponse = app
        .wrap()
        .query_wasm_smart(
            &cw721,
            &Cw721QueryMsg::<
                DefaultOptionalNftExtension,
                DefaultOptionalCollectionExtension,
                Empty,
            >::NumTokens {},
        )
        .unwrap();
    assert_eq!(num_tokens.count, 1);
}

#[test]
fn test_queries() {
    // --- setup ---
    let mut app = new();
    let admin = app.api().addr_make(ADMIN_ADDR);
    let code_id = app.store_code(cw721_base_latest_contract());
    let creator = app.api().addr_make(CREATOR_ADDR);
    let minter_addr = app.api().addr_make(MINTER_ADDR);
    let withdraw_addr = app.api().addr_make(OTHER2_ADDR);
    let cw721 = app
        .instantiate_contract(
            code_id,
            creator.clone(),
            &Cw721InstantiateMsg::<DefaultOptionalCollectionExtension> {
                name: "collection".to_string(),
                symbol: "symbol".to_string(),
                minter: Some(minter_addr.to_string()),
                creator: None, // in case of none, sender is creator
                collection_info_extension: None,
                withdraw_address: Some(withdraw_addr.to_string()),
            },
            &[],
            "cw721-base",
            Some(admin.to_string()),
        )
        .unwrap();
    // mint
    let nft_owner = app.api().addr_make(NFT_OWNER_ADDR);
    let nft_metadata_msg = NftExtensionMsg {
        image: Some("ipfs://foo.bar/image.png".to_string()),
        image_data: Some("image data".to_string()),
        external_url: Some("https://github.com".to_string()),
        description: Some("description".to_string()),
        name: Some("name".to_string()),
        attributes: Some(vec![Trait {
            trait_type: "trait_type".to_string(),
            value: "value".to_string(),
            display_type: Some("display_type".to_string()),
        }]),
        background_color: Some("background_color".to_string()),
        animation_url: Some("ssl://animation_url".to_string()),
        youtube_url: Some("file://youtube_url".to_string()),
    };
    app.execute_contract(
        minter_addr.clone(),
        cw721.clone(),
        &Cw721ExecuteMsg::<
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
        >::Mint {
            token_id: "1".to_string(),
            owner: nft_owner.to_string(),
            token_uri: Some("ipfs://foo.bar/metadata.json".to_string()),
            extension: Some(nft_metadata_msg.clone()),
        },
        &[],
    )
    .unwrap();

    // check nft info
    let nft_info = query_nft_info(app.wrap(), &cw721, "1".to_string());
    assert_eq!(
        nft_info.token_uri,
        Some("ipfs://foo.bar/metadata.json".to_string())
    );
    assert_eq!(
        nft_info.extension,
        Some(NftExtension {
            image: Some("ipfs://foo.bar/image.png".to_string()),
            image_data: Some("image data".to_string()),
            external_url: Some("https://github.com".to_string()),
            description: Some("description".to_string()),
            name: Some("name".to_string()),
            attributes: Some(vec![Trait {
                trait_type: "trait_type".to_string(),
                value: "value".to_string(),
                display_type: Some("display_type".to_string()),
            }]),
            background_color: Some("background_color".to_string()),
            animation_url: Some("ssl://animation_url".to_string()),
            youtube_url: Some("file://youtube_url".to_string()),
        })
    );
    // check num tokens
    let num_tokens: NumTokensResponse = app
        .wrap()
        .query_wasm_smart(
            &cw721,
            &Cw721QueryMsg::<
                DefaultOptionalNftExtension,
                DefaultOptionalCollectionExtension,
                Empty,
            >::NumTokens {},
        )
        .unwrap();
    assert_eq!(num_tokens.count, 1);
    // check withdraw address
    let withdraw_addr_result: Option<String> = app
        .wrap()
        .query_wasm_smart(
            &cw721,
            &Cw721QueryMsg::<
                DefaultOptionalNftExtension,
                DefaultOptionalCollectionExtension,
                Empty,
            >::GetWithdrawAddress {},
        )
        .unwrap();
    assert_eq!(withdraw_addr_result, Some(withdraw_addr.to_string()));
    // check all collection info
    let all_collection_info = query_all_collection_info(app.wrap(), &cw721);
    let contract_info = app.wrap().query_wasm_contract_info(&cw721).unwrap();
    assert_eq!(
        all_collection_info,
        ConfigResponse {
            minter_ownership: Ownership {
                owner: Some(minter_addr),
                pending_expiry: None,
                pending_owner: None,
            },
            creator_ownership: Ownership {
                owner: Some(creator),
                pending_expiry: None,
                pending_owner: None,
            },
            collection_info: CollectionInfo {
                name: "collection".to_string(),
                symbol: "symbol".to_string(),
                updated_at: all_collection_info.collection_info.updated_at,
            },
            collection_extension: None,
            num_tokens: 1,
            withdraw_address: Some(withdraw_addr.into_string()),
            contract_info
        }
    );
}
