use crate::{
    error::ContractError, msg::InstantiateMsg, state::DefaultCw721ExpirationContract,
    CONTRACT_NAME, CONTRACT_VERSION,
};
use cosmwasm_std::{Binary, DepsMut, Empty, Env, MessageInfo, Response};
use cw721::{
    msg::{Cw721ExecuteMsg, Cw721InstantiateMsg, Cw721MigrateMsg},
    traits::Cw721Execute,
    Expiration,
};
use cw721::{DefaultOptionalCollectionExtensionMsg, DefaultOptionalNftExtensionMsg};

impl DefaultCw721ExpirationContract<'static> {
    // -- instantiate --
    pub fn instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response<Empty>, ContractError> {
        if msg.expiration_days == 0 {
            return Err(ContractError::MinExpiration {});
        }
        let contract = DefaultCw721ExpirationContract::default();
        contract
            .expiration_days
            .save(deps.storage, &msg.expiration_days)?;
        Ok(contract.base_contract.instantiate_with_version(
            deps,
            &env,
            &info,
            Cw721InstantiateMsg {
                name: msg.name,
                symbol: msg.symbol,
                collection_info_extension: msg.collection_info_extension,
                minter: msg.minter,
                creator: msg.creator,
                withdraw_address: msg.withdraw_address,
            },
            CONTRACT_NAME,
            CONTRACT_VERSION,
        )?)
    }

    // -- execute --
    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721ExecuteMsg<
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
        >,
    ) -> Result<Response<Empty>, ContractError> {
        let contract = DefaultCw721ExpirationContract::default();
        match msg {
            Cw721ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => {
                contract.mint_with_timestamp(deps, env, info, token_id, owner, token_uri, extension)
            }
            Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => contract.approve_include_nft_expired(deps, env, info, spender, token_id, expires),
            Cw721ExecuteMsg::Revoke { spender, token_id } => {
                contract.revoke_include_nft_expired(deps, env, info, spender, token_id)
            }
            Cw721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => contract.transfer_nft_include_nft_expired(deps, env, info, recipient, token_id),
            Cw721ExecuteMsg::SendNft {
                contract: recipient,
                token_id,
                msg,
            } => contract.send_nft_include_nft_expired(deps, env, info, recipient, token_id, msg),
            Cw721ExecuteMsg::Burn { token_id } => {
                contract.burn_nft_include_nft_expired(deps, env, info, token_id)
            }
            _ => {
                let response = contract.base_contract.execute(deps, &env, &info, msg)?;
                Ok(response)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn mint_with_timestamp(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: DefaultOptionalNftExtensionMsg,
    ) -> Result<Response<Empty>, ContractError> {
        let mint_timstamp = env.block.time;
        self.mint_timestamps
            .save(deps.storage, &token_id, &mint_timstamp)?;
        let res = self
            .base_contract
            .mint(deps, &env, &info, token_id, owner, token_uri, extension)?
            .add_attribute("mint_timestamp", mint_timstamp.to_string());
        Ok(res)
    }

    pub fn approve_include_nft_expired(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<Empty>, ContractError> {
        self.assert_nft_expired(deps.as_ref(), &env, token_id.as_str())?;
        Ok(self
            .base_contract
            .approve(deps, &env, &info, spender, token_id, expires)?)
    }

    pub fn revoke_include_nft_expired(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<Empty>, ContractError> {
        self.assert_nft_expired(deps.as_ref(), &env, token_id.as_str())?;
        Ok(self
            .base_contract
            .revoke(deps, &env, &info, spender, token_id)?)
    }

    pub fn transfer_nft_include_nft_expired(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<Empty>, ContractError> {
        self.assert_nft_expired(deps.as_ref(), &env, token_id.as_str())?;
        Ok(self
            .base_contract
            .transfer_nft(deps, &env, &info, recipient, token_id)?)
    }

    pub fn send_nft_include_nft_expired(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<Empty>, ContractError> {
        self.assert_nft_expired(deps.as_ref(), &env, token_id.as_str())?;
        Ok(self
            .base_contract
            .send_nft(deps, &env, &info, contract, token_id, msg)?)
    }

    pub fn burn_nft_include_nft_expired(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<Empty>, ContractError> {
        self.assert_nft_expired(deps.as_ref(), &env, token_id.as_str())?;
        Ok(self.base_contract.burn_nft(deps, &env, &info, token_id)?)
    }

    // -- migrate --
    pub fn migrate(
        &self,
        deps: DepsMut,
        env: Env,
        msg: Cw721MigrateMsg,
        contract_name: &str,
        contract_version: &str,
    ) -> Result<Response<Empty>, ContractError> {
        Ok(self
            .base_contract
            .migrate(deps, env, msg, contract_name, contract_version)?)
    }
}
