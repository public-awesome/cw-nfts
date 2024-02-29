use cosmwasm_std::Empty;
use cw721::MetadataExtension;
pub use cw721_base::{ContractError, EmptyCollectionInfoExtension, InstantiateMsg, MinterResponse};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw721-metadata-onchain";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<
    'a,
    MetadataExtension,
    Empty,
    Empty,
    Empty,
    EmptyCollectionInfoExtension,
>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<MetadataExtension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg<EmptyCollectionInfoExtension>,
    ) -> Result<Response, ContractError> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Cw721MetadataContract::default().instantiate(deps.branch(), env, info, msg)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::{Cw721Query, Metadata};

    const CREATOR: &str = "creator";

    /// Make sure cw2 version info is properly initialized during instantiation,
    /// and NOT overwritten by the base contract.
    #[test]
    fn proper_cw2_initialization() {
        let mut deps = mock_dependencies();

        entry::instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("larry", &[]),
            InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_info_extension: None,
                minter: Some("minter".into()),
                creator: Some("creator".into()),
                withdraw_address: None,
            },
        )
        .unwrap();

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(version.contract, CONTRACT_NAME);
        assert_ne!(version.contract, cw721_base::CONTRACT_NAME);
    }

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "collection_name".into(),
            symbol: "collection_symbol".into(),
            collection_info_extension: None,
            minter: None,
            creator: None,
            withdraw_address: None,
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let token_uri = Some("https://starships.example.com/Starship/Enterprise.json".into());
        let extension = Some(Metadata {
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..Metadata::default()
        });
        let exec_msg = ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: token_uri.clone(),
            extension: extension.clone(),
        };
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, token_uri);
        assert_eq!(res.extension, extension);
    }
}
