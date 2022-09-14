use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use cw2::set_contract_version;
pub use cw721_base::{ContractError, InstantiateMsg, MigrateMsg, MintMsg, MinterResponse};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw721-metadata-onchain";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

pub type Extension = Option<Metadata>;

pub type Cw721MetadataContract<'a> =
    cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw721_base::msg::MigrateMsg;

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg<Empty>,
    ) -> Result<Response, ContractError> {
        let res = Cw721MetadataContract::default().instantiate(deps.branch(), env, info, msg)?;
        // Explicitly set contract name and version, otherwise set to cw721-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721MetadataContract::default().query(deps, env, msg)
    }

    #[entry_point]
    pub fn migrate(
        deps: DepsMut,
        env: Env,
        msg: MigrateMsg<Empty>,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().migrate(deps, env, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        to_binary, Addr, CosmosMsg, WasmMsg,
    };
    use cw721::Cw721Query;
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    fn cw721_metadata_onchain_v0134_contract_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw721_metadata_onchain_v0134_contract::entry::execute,
            cw721_metadata_onchain_v0134_contract::entry::instantiate,
            cw721_metadata_onchain_v0134_contract::entry::query,
        );
        Box::new(contract)
    }

    fn cw721_base_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::entry::execute,
            crate::entry::instantiate,
            crate::entry::query,
        )
        .with_migrate(crate::entry::migrate);
        Box::new(contract)
    }

    const CREATOR: &str = "creator";
    const CONTRACT_NAME: &str = "Magic Power";
    const CONTRACT_URI: &str = "https://example.com/example.jpg";
    const SYMBOL: &str = "MGK";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg::<Empty> {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
            collection_uri: Some(String::from(CONTRACT_URI)),
            metadata: Empty {},
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let mint_msg = MintMsg {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: Some(Metadata {
                description: Some("Spaceship with Warp Drive".into()),
                name: Some("Starship USS Enterprise".to_string()),
                ..Metadata::default()
            }),
        };
        let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, mint_msg.token_uri);
        assert_eq!(res.extension, mint_msg.extension);
    }

    #[test]
    fn test_migrate_from_v0134() {
        const CREATOR: &str = "creator";

        let mut app = App::default();
        let v0134_code_id = app.store_code(cw721_metadata_onchain_v0134_contract_contract());

        // Instantiate old NFT contract
        let v0134_addr = app
            .instantiate_contract(
                v0134_code_id,
                Addr::unchecked(CREATOR),
                &cw721_metadata_onchain_v0134_contract::InstantiateMsg {
                    name: "Test".to_string(),
                    symbol: "TEST".to_string(),
                    minter: CREATOR.to_string(),
                },
                &[],
                "Old cw721-base",
                Some(CREATOR.to_string()),
            )
            .unwrap();

        let cw721_base_code_id = app.store_code(cw721_base_contract());

        // Now we can migrate!
        app.execute(
            Addr::unchecked(CREATOR),
            CosmosMsg::Wasm(WasmMsg::Migrate {
                contract_addr: v0134_addr.to_string(),
                new_code_id: cw721_base_code_id,
                msg: to_binary(&MigrateMsg::<Empty> {
                    name: "Test".to_string(),
                    symbol: "TEST".to_string(),
                    collection_uri: Some("https://ipfs.io/hash".to_string()),
                    metadata: Empty {},
                })
                .unwrap(),
            }),
        )
        .unwrap();
    }
}
