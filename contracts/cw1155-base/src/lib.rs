mod contract_tests;
mod execute;
mod query;
pub mod state;

pub use crate::state::Cw1155Contract;
use cosmwasm_std::Empty;
use cw1155::msg::{Cw1155ExecuteMsg, Cw1155QueryMsg};
use cw1155::state::{Cw1155Config, DefaultOptionMetadataExtension};

// Version info for migration
pub const CONTRACT_NAME: &str = "crates.io:cw1155-base";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Cw1155BaseContract<'a> =
    Cw1155Contract<'a, DefaultOptionMetadataExtension, Empty, Empty, Empty>;
pub type Cw1155BaseExecuteMsg = Cw1155ExecuteMsg<DefaultOptionMetadataExtension, Empty>;
pub type Cw1155BaseQueryMsg = Cw1155QueryMsg<DefaultOptionMetadataExtension, Empty>;
pub type Cw1155BaseConfig<'a> =
    Cw1155Config<'a, DefaultOptionMetadataExtension, Empty, Empty, Empty>;

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw1155::error::Cw1155ContractError;
    use cw1155::execute::Cw1155Execute;
    use cw1155::msg::{Cw1155ExecuteMsg, Cw1155InstantiateMsg, Cw1155QueryMsg};
    use cw1155::query::Cw1155Query;
    use cw1155::state::DefaultOptionMetadataExtension;

    // This makes a conscious choice on the various generics used by the contract
    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw1155InstantiateMsg,
    ) -> Result<Response, Cw1155ContractError> {
        let tract = Cw1155BaseContract::default();
        tract.instantiate(deps, env, info, msg, CONTRACT_NAME, CONTRACT_VERSION)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw1155ExecuteMsg<DefaultOptionMetadataExtension, Empty>,
    ) -> Result<Response, Cw1155ContractError> {
        let tract = Cw1155BaseContract::default();
        tract.execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(
        deps: Deps,
        env: Env,
        msg: Cw1155QueryMsg<DefaultOptionMetadataExtension, Empty>,
    ) -> StdResult<Binary> {
        let tract = Cw1155BaseContract::default();
        tract.query(deps, env, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(deps: DepsMut, env: Env, msg: Empty) -> Result<Response, Cw1155ContractError> {
        let contract = Cw1155BaseContract::default();
        contract.migrate(deps, env, msg, CONTRACT_NAME, CONTRACT_VERSION)
    }
}
