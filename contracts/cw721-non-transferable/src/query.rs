use crate::{msg::GetAdminResponse, state::CONFIG};
use cosmwasm_std::{Deps, Env, StdResult};

pub fn admin(deps: Deps, _env: Env) -> StdResult<GetAdminResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(GetAdminResponse {
        admin: config.admin,
    })
}
