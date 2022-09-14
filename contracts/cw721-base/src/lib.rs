mod error;
mod execute;
pub mod helpers;
pub mod msg;
mod query;
pub mod state;
mod tests;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, MintMsg, MinterResponse, QueryMsg};
pub use crate::state::Cw721Contract;
use cosmwasm_std::Empty;

// This is a simple type to let us handle empty extensions
pub type Extension = Option<Empty>;

pub mod entry {
    use crate::msg::MigrateMsg;

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
        msg: InstantiateMsg<Empty>,
    ) -> StdResult<Response> {
        let contract = Cw721Contract::<Extension, Empty, Empty, Empty, Empty>::default();
        contract.instantiate(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension, Empty>,
    ) -> Result<Response, ContractError> {
        let contract = Cw721Contract::<Extension, Empty, Empty, Empty, Empty>::default();
        contract.execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        let contract = Cw721Contract::<Extension, Empty, Empty, Empty, Empty>::default();
        contract.query(deps, env, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(
        deps: DepsMut,
        env: Env,
        msg: MigrateMsg<Empty>,
    ) -> Result<Response, ContractError> {
        let contract = Cw721Contract::<Extension, Empty, Empty, Empty, Empty>::default();
        contract.migrate(deps, env, msg)
    }
}
