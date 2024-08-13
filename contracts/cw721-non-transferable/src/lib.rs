pub use crate::msg::{InstantiateMsg, QueryMsg};
use cosmwasm_std::Empty;
use cw721::extension::Cw721EmptyExtensions;

#[allow(deprecated)]
pub mod msg;
pub mod query;
pub mod state;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-non-transferable";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Cw721NonTransferableContract<'a> = Cw721EmptyExtensions<'a>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;
    use crate::query::admin;
    use crate::state::{Config, CONFIG};
    use cosmwasm_std::{
        entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    };
    use cw721::error::Cw721ContractError;
    use cw721::msg::{Cw721ExecuteMsg, Cw721InstantiateMsg};
    use cw721::traits::{Cw721Execute, Cw721Query};
    use cw721::{EmptyOptionalCollectionExtensionMsg, EmptyOptionalNftExtensionMsg};

    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg<EmptyOptionalCollectionExtensionMsg>,
    ) -> Result<Response, Cw721ContractError> {
        let admin_addr: Option<Addr> = msg
            .admin
            .as_deref()
            .map(|s| deps.api.addr_validate(s))
            .transpose()?;

        let config = Config { admin: admin_addr };

        CONFIG.save(deps.storage, &config)?;

        let cw721_instantiate_msg = Cw721InstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            collection_info_extension: msg.collection_info_extension,
            minter: msg.minter,
            creator: msg.creator,
            withdraw_address: msg.withdraw_address,
        };

        Cw721NonTransferableContract::default().instantiate_with_version(
            deps.branch(),
            &env,
            &info,
            cw721_instantiate_msg,
            "contract_name",
            "contract_version",
        )?;

        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default()
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721ExecuteMsg<
            EmptyOptionalNftExtensionMsg,
            EmptyOptionalCollectionExtensionMsg,
            Empty,
        >,
    ) -> Result<Response, Cw721ContractError> {
        let config = CONFIG.load(deps.storage)?;
        match config.admin {
            Some(admin) => {
                if admin == info.sender {
                    Cw721EmptyExtensions::default().execute(deps, &env, &info, msg)
                } else {
                    Err(Cw721ContractError::Ownership(
                        cw721::OwnershipError::NotOwner,
                    ))
                }
            }
            None => match msg {
                Cw721ExecuteMsg::Mint {
                    token_id,
                    owner,
                    token_uri,
                    extension,
                } => Cw721NonTransferableContract::default()
                    .mint(deps, &env, &info, token_id, owner, token_uri, extension),
                _ => Err(Cw721ContractError::Ownership(
                    cw721::OwnershipError::NotOwner,
                )),
            },
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, Cw721ContractError> {
        match msg {
            QueryMsg::Admin {} => Ok(to_json_binary(&admin(deps)?)?),
            _ => Cw721EmptyExtensions::default().query(deps, &env, msg.into()),
        }
    }

    #[entry_point]
    pub fn migrate(
        deps: DepsMut,
        env: Env,
        msg: cw721::msg::Cw721MigrateMsg,
    ) -> Result<Response, Cw721ContractError> {
        let contract = Cw721EmptyExtensions::default();
        contract.migrate(deps, env, msg, CONTRACT_NAME, CONTRACT_VERSION)
    }
}
