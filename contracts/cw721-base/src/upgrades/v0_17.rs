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
    // remove old minter info
    let tract16 = v16::Cw721Contract::<T, C, E, Q>::default();
    let minter = tract16.minter.load(deps.storage)?;
    tract16.minter.remove(deps.storage);

    // save new ownership info
    let ownership = cw_ownable::initialize_owner(deps.storage, deps.api, Some(minter.as_str()))?;

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("from_version", "0.16.0")
        .add_attribute("to_version", "0.17.0")
        .add_attribute("old_minter", minter)
        .add_attributes(ownership.into_attributes()))
}
