use cosmwasm_std::{CustomMsg, DepsMut, Response};
use cw721_base_016 as v16;
use serde::{de::DeserializeOwned, Serialize};

use crate::ContractError;

pub fn migrate<T, C, E, Q>(deps: DepsMut) -> Result<Response<C>, ContractError>
where
    T: Serialize + DeserializeOwned + Clone,
    Q: CustomMsg,
    E: CustomMsg,
{
    let tract16 = v16::Cw721Contract::<T, C, E, Q>::default();
    let minter = tract16.minter.load(deps.storage)?;
    tract16.minter.remove(deps.storage);
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(minter.as_str()))?;
    let ownership = cw_ownable::get_ownership(deps.storage)?;
    Ok(Response::new()
        .add_attribute("action", "migrate_016_017")
        .add_attribute("old_minter", minter)
        .add_attributes(ownership.into_attributes()))
}
