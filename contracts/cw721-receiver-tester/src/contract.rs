#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{from_json, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InnerMsg, InstantiateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ReceiveNft(receive_msg) => {
            let inner: InnerMsg = from_json(&receive_msg.msg)?;
            match inner {
                InnerMsg::Succeed => Ok(Response::new()
                    .add_attributes([
                        ("action", "receive_nft"),
                        ("token_id", receive_msg.token_id.as_str()),
                        ("sender", receive_msg.sender.as_str()),
                        ("msg", receive_msg.msg.to_base64().as_str()),
                    ])
                    .set_data(
                        [
                            receive_msg.token_id,
                            receive_msg.sender,
                            receive_msg.msg.to_base64(),
                        ]
                        .concat()
                        .as_bytes(),
                    )),
                InnerMsg::Fail => Err(ContractError::Failed {}),
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}
