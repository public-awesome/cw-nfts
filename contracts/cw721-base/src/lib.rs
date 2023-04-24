mod error;
mod execute;
pub mod helpers;
pub mod msg;
mod query;
pub mod state;
pub mod upgrades;

#[cfg(test)]
mod contract_tests;
#[cfg(test)]
mod multi_tests;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, MinterResponse, QueryMsg};
pub use crate::state::Cw721Contract;

// These types are re-exported so that contracts interacting with this
// one don't need a direct dependency on cw_ownable to use the API.
//
// `Action` is used in `ExecuteMsg::UpdateOwnership`, `Ownership` is
// used in `QueryMsg::Ownership`, and `OwnershipError` is used in
// `ContractError::Ownership`.
pub use cw_ownable::{Action, Ownership, OwnershipError};

use cosmwasm_std::Empty;

// This is a simple type to let us handle empty extensions
pub type Extension = Option<Empty>;

// Version info for migration
pub const CONTRACT_NAME: &str = "crates.io:cw721-base";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// currently we only support migrating from 0.16.0. this is ok for now because
// we have not released any 0.16.x where x != 0
//
// TODO: parse semvar so that any version 0.16.x can be migrated from
pub const EXPECTED_FROM_VERSION: &str = "0.16.0";

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
        tract.instantiate(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension, Empty>,
    ) -> Result<Response, ContractError> {
        let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
        tract.execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
        tract.query(deps, env, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
        // make sure the correct contract is being upgraded, and it's being
        // upgraded from the correct version.
        cw2::assert_contract_version(deps.as_ref().storage, CONTRACT_NAME, EXPECTED_FROM_VERSION)?;

        // update contract version
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        // perform the upgrade
        upgrades::v0_17::migrate::<Extension, Empty, Empty, Empty>(deps)
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
