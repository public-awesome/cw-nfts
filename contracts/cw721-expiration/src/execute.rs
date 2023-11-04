use cosmwasm_std::{Binary, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw721::{Cw721Execute, Expiration};
use cw721_base::Cw721Contract;

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
        match msg {
            ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => self.mint(deps, info, token_id, owner, token_uri, extension),
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => self.approve(deps, env, info, spender, token_id, expires),
            ExecuteMsg::Revoke { spender, token_id } => {
                self.revoke(deps, env, info, spender, token_id)
            }
            ExecuteMsg::ApproveAll { operator, expires } => {
                self.approve_all(deps, env, info, operator, expires)
            }
            ExecuteMsg::RevokeAll { operator } => self.revoke_all(deps, env, info, operator),
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => self.transfer_nft(deps, env, info, recipient, token_id),
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => self.send_nft(deps, env, info, contract, token_id, msg),
            ExecuteMsg::Burn { token_id } => self.burn(deps, env, info, token_id),
            ExecuteMsg::UpdateOwnership(action) => Self::update_ownership(deps, env, info, action),
            ExecuteMsg::Extension { msg: _ } => Ok(Response::default()),
        }
    }

}

impl<'a> Cw721ExpirationContract<'a>
{
    pub fn mint(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: Extension,
    ) -> Result<Response, ContractError> {
        self.base_contract.mint(deps, info, token_id, owner, token_uri, extension)
    }

    pub fn update_ownership(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        action: cw_ownable::Action,
    ) -> Result<Response, ContractError> {
        Cw721Contract::<Extension, Empty, Empty, Empty>::update_ownership(deps, env, info, action)
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
