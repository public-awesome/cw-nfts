use cosmwasm_std::{to_json_binary, Binary, CustomMsg, Deps, Env, StdResult};
use cw721::query::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, Cw721Query, NftInfoResponse,
    OwnerOfResponse, TokensResponse,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{
    error::ContractError, msg::QueryMsg, state::Cw721ExpirationContract,
};

impl<
        'a,
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
        TCollectionInfoExtension,
    >
    Cw721ExpirationContract<
        'a,
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
        TCollectionInfoExtension,
    >
where
    TMetadata: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TExtensionExecuteMsg: CustomMsg,
    TMetadataResponse: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
    pub fn query(
        &self,
        deps: Deps,
        env: Env,
        msg: QueryMsg<TMetadataResponse, TCollectionInfoExtension>,
    ) -> Result<Binary, ContractError> {
        let contract = Cw721ExpirationContract::<
            TMetadata,
            TCustomResponseMessage,
            TExtensionExecuteMsg,
            TMetadataResponse,
            TCollectionInfoExtension,
        >::default();
        match msg {
            // -------- msgs with `include_expired_nft` prop --------
            QueryMsg::OwnerOf {
                token_id,
                include_expired: include_expired_approval,
                include_expired_nft,
            } => Ok(to_json_binary(
                &contract.query_owner_of_include_expired_nft(
                    deps,
                    env,
                    token_id,
                    include_expired_approval.unwrap_or(false),
                    include_expired_nft.unwrap_or(false),
                )?,
            )?),
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
                include_expired_nft: include_invalid,
            } => Ok(to_json_binary(
                &contract.query_approval_include_expired_nft(
                    deps,
                    env,
                    token_id,
                    spender,
                    include_expired.unwrap_or(false),
                    include_invalid.unwrap_or(false),
                )?,
            )?),
            QueryMsg::Approvals {
                token_id,
                include_expired,
                include_expired_nft: include_invalid,
            } => Ok(to_json_binary(
                &contract.query_approvals_include_expired_nft(
                    deps,
                    env,
                    token_id,
                    include_expired.unwrap_or(false),
                    include_invalid.unwrap_or(false),
                )?,
            )?),
            QueryMsg::NftInfo {
                token_id,
                include_expired_nft: include_expired,
            } => Ok(to_json_binary(
                &contract.query_nft_info_include_expired_nft(
                    deps,
                    env,
                    token_id,
                    include_expired.unwrap_or(false),
                )?,
            )?),
            QueryMsg::AllNftInfo {
                token_id,
                include_expired: include_expired_approval,
                include_expired_nft,
            } => Ok(to_json_binary(
                &contract.query_all_nft_info_include_expired_nft(
                    deps,
                    env,
                    token_id,
                    include_expired_approval.unwrap_or(false),
                    include_expired_nft.unwrap_or(false),
                )?,
            )?),
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
                include_expired_nft: include_invalid,
            } => Ok(to_json_binary(
                &contract.query_tokens_include_expired_nft(
                    deps,
                    env,
                    owner,
                    start_after,
                    limit,
                    include_invalid.unwrap_or(false),
                )?,
            )?),
            QueryMsg::AllTokens {
                start_after,
                limit,
                include_expired_nft: include_invalid,
            } => Ok(to_json_binary(
                &contract.query_all_tokens_include_expired_nft(
                    deps,
                    env,
                    start_after,
                    limit,
                    include_invalid.unwrap_or(false),
                )?,
            )?),
            // -------- below is from cw721/src/msg.rs --------
            QueryMsg::Operator {
                owner,
                operator,
                include_expired: include_expired_approval,
            } => Ok(to_json_binary(&contract.base_contract.query_operator(
                deps,
                env,
                owner,
                operator,
                include_expired_approval.unwrap_or(false),
            )?)?),
            QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => Ok(to_json_binary(&contract.base_contract.query_operators(
                deps,
                env,
                owner,
                include_expired.unwrap_or(false),
                start_after,
                limit,
            )?)?),
            QueryMsg::NumTokens {} => Ok(to_json_binary(
                &contract.base_contract.query_num_tokens(deps, env)?,
            )?),
            #[allow(deprecated)]
            QueryMsg::ContractInfo {} => Ok(to_json_binary(
                &contract.base_contract.query_collection_info(deps, env)?,
            )?),
            QueryMsg::GetCollectionInfo {} => Ok(to_json_binary(
                &contract.base_contract.query_collection_info(deps, env)?,
            )?),
            #[allow(deprecated)]
            QueryMsg::Ownership {} => Ok(to_json_binary(
                &contract
                    .base_contract
                    .query_minter_ownership(deps.storage)?,
            )?),
            QueryMsg::GetMinterOwnership {} => Ok(to_json_binary(
                &contract
                    .base_contract
                    .query_minter_ownership(deps.storage)?,
            )?),
            QueryMsg::GetCreatorOwnership {} => Ok(to_json_binary(
                &contract
                    .base_contract
                    .query_creator_ownership(deps.storage)?,
            )?),
            #[allow(deprecated)]
            QueryMsg::Minter {} => Ok(to_json_binary(
                &contract.base_contract.query_minter(deps.storage)?,
            )?),
            QueryMsg::Extension { msg } => Ok(to_json_binary(
                &contract.base_contract.query_extension(deps, env, msg)?,
            )?),
            QueryMsg::GetCollectionInfoExtension { msg } => Ok(to_json_binary(
                &contract
                    .base_contract
                    .query_collection_info_extension(deps, env, msg)?,
            )?),
            QueryMsg::GetWithdrawAddress {} => Ok(to_json_binary(
                &contract.base_contract.query_withdraw_address(deps)?,
            )?),
        }
    }

    pub fn query_nft_info_include_expired_nft(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired_nft: bool,
    ) -> Result<NftInfoResponse<TMetadata>, ContractError> {
        if !include_expired_nft {
            self.assert_nft_expired(deps, &env, token_id.as_str())?;
        }
        Ok(self.base_contract.query_nft_info(deps, env, token_id)?)
    }

    pub fn query_owner_of_include_expired_nft(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired_approval: bool,
        include_expired_nft: bool,
    ) -> Result<OwnerOfResponse, ContractError> {
        if !include_expired_nft {
            self.assert_nft_expired(deps, &env, token_id.as_str())?;
        }
        Ok(self
            .base_contract
            .query_owner_of(deps, env, token_id, include_expired_approval)?)
    }

    pub fn query_approval_include_expired_nft(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        spender: String,
        include_expired_approval: bool,
        include_expired_nft: bool,
    ) -> Result<ApprovalResponse, ContractError> {
        if !include_expired_nft {
            self.assert_nft_expired(deps, &env, token_id.as_str())?;
        }
        Ok(self.base_contract.query_approval(
            deps,
            env,
            token_id,
            spender,
            include_expired_approval,
        )?)
    }

    /// approvals returns all approvals owner given access to
    pub fn query_approvals_include_expired_nft(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired_approval: bool,
        include_expired_nft: bool,
    ) -> Result<ApprovalsResponse, ContractError> {
        if !include_expired_nft {
            self.assert_nft_expired(deps, &env, token_id.as_str())?;
        }
        Ok(self
            .base_contract
            .query_approvals(deps, env, token_id, include_expired_approval)?)
    }

    pub fn query_tokens_include_expired_nft(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
        include_expired_nft: bool,
    ) -> StdResult<TokensResponse> {
        let tokens =
            self.base_contract
                .query_tokens(deps, env.clone(), owner, start_after, limit)?;
        if include_expired_nft {
            return Ok(tokens);
        }
        let filtered: Vec<_> = tokens
            .tokens
            .iter()
            .filter(|token_id| {
                self.is_nft_expired(deps, &env, token_id).unwrap_or(false) // Convert Option<bool> to bool
            })
            .map(|token_id| token_id.to_string())
            .collect();
        Ok(TokensResponse { tokens: filtered })
    }

    pub fn query_all_tokens_include_expired_nft(
        &self,
        deps: Deps,
        env: Env,
        start_after: Option<String>,
        limit: Option<u32>,
        include_expired_nft: bool,
    ) -> Result<TokensResponse, ContractError> {
        let tokens = self
            .base_contract
            .query_all_tokens(deps, env.clone(), start_after, limit)?;
        if include_expired_nft {
            return Ok(tokens);
        }
        let filtered: Vec<_> = tokens
            .tokens
            .iter()
            .filter(|token_id| {
                self.is_nft_expired(deps, &env, token_id).unwrap_or(false) // Convert Option<bool> to bool
            })
            .map(|token_id| token_id.to_string())
            .collect();
        Ok(TokensResponse { tokens: filtered })
    }

    pub fn query_all_nft_info_include_expired_nft(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired_approval: bool,
        include_expired_nft: bool,
    ) -> Result<AllNftInfoResponse<TMetadata>, ContractError> {
        if !include_expired_nft {
            self.assert_nft_expired(deps, &env, token_id.as_str())?;
        }
        Ok(self
            .base_contract
            .query_all_nft_info(deps, env, token_id, include_expired_approval)?)
    }

    // --- helpers ---
    pub fn is_nft_expired(&self, deps: Deps, env: &Env, token_id: &str) -> StdResult<bool> {
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
    pub fn assert_nft_expired(
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
