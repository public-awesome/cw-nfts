use cosmwasm_std::{Binary, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw721::{Cw721Execute, Expiration};

use crate::{state::Cw721ExpirationContract, ContractError, ExecuteMsg, Extension, InstantiateMsg};

impl<'a> Cw721ExpirationContract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        self.base_contract.instantiate(deps, env, info, msg)
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        self.base_contract.execute(deps, env, info, msg)
    }
}

impl<'a> Cw721Execute<Extension, Empty> for Cw721ExpirationContract<'a> {
    type Err = ContractError;

    fn transfer_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<Empty>, Self::Err> {
        self.base_contract
            .transfer_nft(deps, env, info, recipient, token_id)
    }

    fn send_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<Empty>, Self::Err> {
        self.base_contract
            .send_nft(deps, env, info, contract, token_id, msg)
    }

    fn approve(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<Empty>, Self::Err> {
        self.base_contract
            .approve(deps, env, info, spender, token_id, expires)
    }

    fn revoke(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<Empty>, Self::Err> {
        self.base_contract
            .revoke(deps, env, info, spender, token_id)
    }

    fn approve_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response, ContractError> {
        self.base_contract
            .approve_all(deps, env, info, operator, expires)
    }

    fn revoke_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response, ContractError> {
        self.base_contract.revoke_all(deps, env, info, operator)
    }

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response, ContractError> {
        self.base_contract.burn(deps, env, info, token_id)
    }
}
