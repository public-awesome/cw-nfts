use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, StdResult};
use cw721::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, ContractInfoResponse, Cw721Query,
    NftInfoResponse, NumTokensResponse, OperatorResponse, OperatorsResponse, OwnerOfResponse,
    TokensResponse,
};
use cw721_base::MinterResponse;

use crate::{error::ContractError, msg::QueryMsg, state::Cw721ExpirationContract, Extension};

impl<'a> Cw721ExpirationContract<'a> {
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
        match msg {
            QueryMsg::Minter {} => Ok(to_json_binary(&self.minter(deps)?)?),
            QueryMsg::ContractInfo {} => Ok(to_json_binary(&self.contract_info(deps)?)?),
            QueryMsg::NftInfo {
                token_id,
                include_invalid,
            } => Ok(to_json_binary(&self.nft_info(
                deps,
                env,
                token_id,
                include_invalid.unwrap_or(false),
            )?)?),
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
                include_invalid,
            } => Ok(to_json_binary(&self.owner_of(
                deps,
                env,
                token_id,
                include_expired.unwrap_or(false),
                include_invalid.unwrap_or(false),
            )?)?),
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
                include_invalid,
            } => Ok(to_json_binary(&self.all_nft_info(
                deps,
                env,
                token_id,
                include_expired.unwrap_or(false),
                include_invalid.unwrap_or(false),
            )?)?),
            QueryMsg::Operator {
                owner,
                operator,
                include_expired,
            } => Ok(to_json_binary(&self.operator(
                deps,
                env,
                owner,
                operator,
                include_expired.unwrap_or(false),
            )?)?),
            QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => Ok(to_json_binary(&self.operators(
                deps,
                env,
                owner,
                include_expired.unwrap_or(false),
                start_after,
                limit,
            )?)?),
            QueryMsg::NumTokens {} => Ok(to_json_binary(&self.num_tokens(deps)?)?),
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
                include_invalid,
            } => Ok(to_json_binary(&self.tokens(
                deps,
                env,
                owner,
                start_after,
                limit,
                include_invalid.unwrap_or(false),
            )?)?),
            QueryMsg::AllTokens {
                start_after,
                limit,
                include_invalid,
            } => Ok(to_json_binary(&self.all_tokens(
                deps,
                env,
                start_after,
                limit,
                include_invalid.unwrap_or(false),
            )?)?),
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
                include_invalid,
            } => Ok(to_json_binary(&self.approval(
                deps,
                env,
                token_id,
                spender,
                include_expired.unwrap_or(false),
                include_invalid.unwrap_or(false),
            )?)?),
            QueryMsg::Approvals {
                token_id,
                include_expired,
                include_invalid,
            } => Ok(to_json_binary(&self.approvals(
                deps,
                env,
                token_id,
                include_expired.unwrap_or(false),
                include_invalid.unwrap_or(false),
            )?)?),
            QueryMsg::Ownership {} => Ok(to_json_binary(&Self::ownership(deps)?)?),
            QueryMsg::Extension { msg: _ } => Ok(Binary::default()),
        }
    }

    pub fn minter(&self, deps: Deps) -> StdResult<MinterResponse> {
        self.base_contract.minter(deps)
    }

    pub fn ownership(deps: Deps) -> StdResult<cw_ownable::Ownership<Addr>> {
        cw_ownable::get_ownership(deps.storage)
    }
}

// queries
impl<'a> Cw721ExpirationContract<'a> {
    pub fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse> {
        self.base_contract.contract_info(deps)
    }

    pub fn num_tokens(&self, deps: Deps) -> StdResult<NumTokensResponse> {
        self.base_contract.num_tokens(deps)
    }

    pub fn nft_info(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_invalid: bool,
    ) -> Result<NftInfoResponse<Extension>, ContractError> {
        if !include_invalid {
            self.assert_valid_nft(deps, &env, token_id.as_str())?;
        }
        Ok(self.base_contract.nft_info(deps, token_id)?)
    }

    pub fn owner_of(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
        include_invalid: bool,
    ) -> Result<OwnerOfResponse, ContractError> {
        if !include_invalid {
            self.assert_valid_nft(deps, &env, token_id.as_str())?;
        }
        Ok(self
            .base_contract
            .owner_of(deps, env, token_id, include_expired)?)
    }

    /// operator returns the approval status of an operator for a given owner if exists
    pub fn operator(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        operator: String,
        include_expired: bool,
    ) -> StdResult<OperatorResponse> {
        self.base_contract
            .operator(deps, env, owner, operator, include_expired)
    }

    /// operators returns all operators owner given access to
    pub fn operators(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse> {
        self.base_contract
            .operators(deps, env, owner, include_expired, start_after, limit)
    }

    pub fn approval(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        spender: String,
        include_expired: bool,
        include_invalid: bool,
    ) -> Result<ApprovalResponse, ContractError> {
        if !include_invalid {
            self.assert_valid_nft(deps, &env, token_id.as_str())?;
        }
        Ok(self
            .base_contract
            .approval(deps, env, token_id, spender, include_expired)?)
    }

    /// approvals returns all approvals owner given access to
    pub fn approvals(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
        include_invalid: bool,
    ) -> Result<ApprovalsResponse, ContractError> {
        if !include_invalid {
            self.assert_valid_nft(deps, &env, token_id.as_str())?;
        }
        Ok(self
            .base_contract
            .approvals(deps, env, token_id, include_expired)?)
    }

    pub fn tokens(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
        include_invalid: bool,
    ) -> StdResult<TokensResponse> {
        let tokens = self.base_contract.tokens(deps, owner, start_after, limit)?;
        if include_invalid {
            return Ok(tokens);
        }
        let filtered: Vec<_> = tokens
            .tokens
            .iter()
            .filter(|token_id| {
                self.is_valid_nft(deps, &env, token_id).unwrap_or(false) // Convert Option<bool> to bool
            })
            .map(|token_id| token_id.to_string())
            .collect();
        Ok(TokensResponse { tokens: filtered })
    }

    pub fn all_tokens(
        &self,
        deps: Deps,
        env: Env,
        start_after: Option<String>,
        limit: Option<u32>,
        include_invalid: bool,
    ) -> Result<TokensResponse, ContractError> {
        let tokens = self.base_contract.all_tokens(deps, start_after, limit)?;
        if include_invalid {
            return Ok(tokens);
        }
        let filtered: Vec<_> = tokens
            .tokens
            .iter()
            .filter(|token_id| {
                self.is_valid_nft(deps, &env, token_id).unwrap_or(false) // Convert Option<bool> to bool
            })
            .map(|token_id| token_id.to_string())
            .collect();
        Ok(TokensResponse { tokens: filtered })
    }

    pub fn all_nft_info(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
        include_invalid: bool,
    ) -> Result<AllNftInfoResponse<Extension>, ContractError> {
        if !include_invalid {
            self.assert_valid_nft(deps, &env, token_id.as_str())?;
        }
        Ok(self
            .base_contract
            .all_nft_info(deps, env, token_id, include_expired)?)
    }
}
