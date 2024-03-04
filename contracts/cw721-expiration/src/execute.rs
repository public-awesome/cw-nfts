use cosmwasm_std::{
    Addr, Binary, Coin, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Storage,
};
use cw721::{Cw721Execute, EmptyCollectionInfoExtension, Expiration};
use cw721_base::Cw721Contract;
use cw_ownable::Action;

use crate::{
    error::ContractError, msg::ExecuteMsg, msg::InstantiateMsg, state::Cw721ExpirationContract,
    EmptyExtension,
};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;

impl<'a> Cw721ExpirationContract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg<EmptyCollectionInfoExtension>,
    ) -> Result<Response, ContractError> {
        if msg.expiration_days == 0 {
            return Err(ContractError::MinExpiration {});
        }
        self.expiration_days
            .save(deps.storage, &msg.expiration_days)?;
        Ok(self.base_contract.instantiate(
            deps,
            env,
            info,
            Cw721InstantiateMsg {
                name: msg.name,
                symbol: msg.symbol,
                collection_info_extension: msg.collection_info_extension,
                minter: msg.minter,
                creator: msg.creator,
                withdraw_address: msg.withdraw_address,
            },
        )?)
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
            } => self.mint(deps, env, info, token_id, owner, token_uri, extension),
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
            #[allow(deprecated)]
            ExecuteMsg::UpdateOwnership(action) => {
                Self::update_minter_ownership(deps, env, info, action)
            }
            ExecuteMsg::UpdateMinterOwnership(action) => {
                Self::update_minter_ownership(deps, env, info, action)
            }
            ExecuteMsg::UpdateCreatorOwnership(action) => {
                Self::update_creator_ownership(deps, env, info, action)
            }
            ExecuteMsg::Extension { msg: _ } => Ok(Response::default()),
            ExecuteMsg::SetWithdrawAddress { address } => {
                self.set_withdraw_address(deps, &info.sender, address)
            }
            ExecuteMsg::RemoveWithdrawAddress {} => {
                self.remove_withdraw_address(deps.storage, &info.sender)
            }
            ExecuteMsg::WithdrawFunds { amount } => self.withdraw_funds(deps.storage, &amount),
        }
    }
}

impl<'a> Cw721ExpirationContract<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn mint(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: EmptyExtension,
    ) -> Result<Response, ContractError> {
        let mint_timstamp = env.block.time;
        self.mint_timestamps
            .save(deps.storage, &token_id, &mint_timstamp)?;
        let res = self
            .base_contract
            .mint(deps, info, token_id, owner, token_uri, extension)?
            .add_attribute("mint_timestamp", mint_timstamp.to_string());
        Ok(res)
    }

    pub fn update_minter_ownership(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        action: Action,
    ) -> Result<Response, ContractError> {
        Ok(Cw721Contract::<
            EmptyExtension,
            Empty,
            Empty,
            Empty,
            EmptyCollectionInfoExtension,
        >::update_minter_ownership(deps, env, info, action)?)
    }

    pub fn update_creator_ownership(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        action: Action,
    ) -> Result<Response, ContractError> {
        Ok(Cw721Contract::<
            EmptyExtension,
            Empty,
            Empty,
            Empty,
            EmptyCollectionInfoExtension,
        >::update_creator_ownership(deps, env, info, action)?)
    }

    pub fn set_withdraw_address(
        &self,
        deps: DepsMut,
        sender: &Addr,
        address: String,
    ) -> Result<Response, ContractError> {
        Ok(self
            .base_contract
            .set_withdraw_address(deps, sender, address)?)
    }

    pub fn remove_withdraw_address(
        &self,
        storage: &mut dyn Storage,
        sender: &Addr,
    ) -> Result<Response, ContractError> {
        Ok(self
            .base_contract
            .remove_withdraw_address(storage, sender)?)
    }

    pub fn withdraw_funds(
        &self,
        storage: &mut dyn Storage,
        amount: &Coin,
    ) -> Result<Response, ContractError> {
        Ok(self.base_contract.withdraw_funds(storage, amount)?)
    }
}

// execute
impl<'a> Cw721ExpirationContract<'a> {
    fn transfer_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<Empty>, ContractError> {
        self.assert_valid_nft(deps.as_ref(), &env, &token_id)?;
        Ok(self
            .base_contract
            .transfer_nft(deps, env, info, recipient, token_id)?)
    }

    fn send_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<Empty>, ContractError> {
        self.assert_valid_nft(deps.as_ref(), &env, &token_id)?;
        Ok(self
            .base_contract
            .send_nft(deps, env, info, contract, token_id, msg)?)
    }

    fn approve(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<Empty>, ContractError> {
        self.assert_valid_nft(deps.as_ref(), &env, &token_id)?;
        Ok(self
            .base_contract
            .approve(deps, env, info, spender, token_id, expires)?)
    }

    fn revoke(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<Empty>, ContractError> {
        self.assert_valid_nft(deps.as_ref(), &env, &token_id)?;
        Ok(self
            .base_contract
            .revoke(deps, env, info, spender, token_id)?)
    }

    fn approve_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response, ContractError> {
        Ok(self
            .base_contract
            .approve_all(deps, env, info, operator, expires)?)
    }

    fn revoke_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response, ContractError> {
        Ok(self.base_contract.revoke_all(deps, env, info, operator)?)
    }

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response, ContractError> {
        self.assert_valid_nft(deps.as_ref(), &env, &token_id)?;
        Ok(self.base_contract.burn(deps, env, info, token_id)?)
    }
}

// helpers
impl<'a> Cw721ExpirationContract<'a> {
    /// throws contract error if nft is expired
    pub fn is_valid_nft(&self, deps: Deps, env: &Env, token_id: &str) -> StdResult<bool> {
        // any non-expired token approval can send
        let mint_date = self.mint_timestamps.load(deps.storage, token_id)?;
        let expiration_days = self.expiration_days.load(deps.storage)?;
        let expiration = mint_date.plus_days(expiration_days.into());
        if env.block.time >= expiration {
            return Ok(false);
        }
        Ok(true)
    }

    /// throws contract error if nft is expired
    pub fn assert_valid_nft(
        &self,
        deps: Deps,
        env: &Env,
        token_id: &str,
    ) -> Result<(), ContractError> {
        // any non-expired token approval can send
        let mint_date = self.mint_timestamps.load(deps.storage, token_id)?;
        let expiration_days = self.expiration_days.load(deps.storage)?;
        let expiration = mint_date.plus_days(expiration_days.into());
        if env.block.time >= expiration {
            return Err(ContractError::NftExpired {
                token_id: token_id.to_string(),
                mint_date,
                expiration,
            });
        }
        Ok(())
    }
}
