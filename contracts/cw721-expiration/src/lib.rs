mod execute;
pub mod msg;
mod query;
pub mod state;

#[cfg(test)]
mod contract_tests;

use cosmwasm_std::Empty;
pub use cw721_base::MinterResponse;

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw721-expiration";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub type Extension = Option<Empty>;

pub type ContractError = cw721_base::ContractError;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;
pub type InstantiateMsg = cw721_base::InstantiateMsg;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

pub mod entry {
    use crate::state::Cw721ExpirationContract;

    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        Cw721ExpirationContract::default().instantiate(deps.branch(), env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721ExpirationContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721ExpirationContract::default().query(deps, env, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
        Ok(Response::default())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw2::ContractVersion;

    use super::*;

    /// Make sure cw2 version info is properly initialized during instantiation.
    #[test]
    fn proper_cw2_initialization() {
        let mut deps = mock_dependencies();

        entry::instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("larry", &[]),
            InstantiateMsg {
                name: "".into(),
                symbol: "".into(),
                minter: "larry".into(),
            },
        )
        .unwrap();

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(
            version,
            ContractVersion {
                contract: CONTRACT_NAME.into(),
                version: CONTRACT_VERSION.into(),
            },
        );
    }
}
