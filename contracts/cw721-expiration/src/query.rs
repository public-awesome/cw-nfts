use cosmwasm_std::{to_binary, Addr, Binary, Deps, Env, StdResult};
use cw721::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, ContractInfoResponse, Cw721Query,
    NftInfoResponse, NumTokensResponse, OperatorResponse, OperatorsResponse, OwnerOfResponse,
    TokensResponse,
};
use cw721_base::MinterResponse;

use crate::{state::Cw721ExpirationContract, Extension, QueryMsg};

impl<'a> Cw721ExpirationContract<'a> {
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Minter {} => to_binary(&self.minter(deps)?),
            QueryMsg::ContractInfo {} => to_binary(&self.contract_info(deps)?),
            QueryMsg::NftInfo { token_id } => to_binary(&self.nft_info(deps, token_id)?),
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => {
                to_binary(&self.owner_of(deps, env, token_id, include_expired.unwrap_or(false))?)
            }
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => to_binary(&self.all_nft_info(
                deps,
                env,
                token_id,
                include_expired.unwrap_or(false),
            )?),
            QueryMsg::Operator {
                owner,
                operator,
                include_expired,
            } => to_binary(&self.operator(
                deps,
                env,
                owner,
                operator,
                include_expired.unwrap_or(false),
            )?),
            QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => to_binary(&self.operators(
                deps,
                env,
                owner,
                include_expired.unwrap_or(false),
                start_after,
                limit,
            )?),
            QueryMsg::NumTokens {} => to_binary(&self.num_tokens(deps)?),
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => to_binary(&self.tokens(deps, owner, start_after, limit)?),
            QueryMsg::AllTokens { start_after, limit } => {
                to_binary(&self.all_tokens(deps, start_after, limit)?)
            }
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => to_binary(&self.approval(
                deps,
                env,
                token_id,
                spender,
                include_expired.unwrap_or(false),
            )?),
            QueryMsg::Approvals {
                token_id,
                include_expired,
            } => {
                to_binary(&self.approvals(deps, env, token_id, include_expired.unwrap_or(false))?)
            }
            QueryMsg::Ownership {} => to_binary(&Self::ownership(deps)?),
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

impl<'a> Cw721Query<Extension> for Cw721ExpirationContract<'a> {
    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse> {
        self.base_contract.contract_info(deps)
    }

    fn num_tokens(&self, deps: Deps) -> StdResult<NumTokensResponse> {
        self.base_contract.num_tokens(deps)
    }

    fn nft_info(&self, deps: Deps, token_id: String) -> StdResult<NftInfoResponse<Extension>> {
        self.base_contract.nft_info(deps, token_id)
    }

    fn owner_of(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<OwnerOfResponse> {
        self.base_contract
            .owner_of(deps, env, token_id, include_expired)
    }

    /// operator returns the approval status of an operator for a given owner if exists
    fn operator(
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
    fn operators(
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

    fn approval(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        spender: String,
        include_expired: bool,
    ) -> StdResult<ApprovalResponse> {
        self.base_contract
            .approval(deps, env, token_id, spender, include_expired)
    }

    /// approvals returns all approvals owner given access to
    fn approvals(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<ApprovalsResponse> {
        self.base_contract
            .approvals(deps, env, token_id, include_expired)
    }

    fn tokens(
        &self,
        deps: Deps,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        self.base_contract.tokens(deps, owner, start_after, limit)
    }

    fn all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        self.base_contract.all_tokens(deps, start_after, limit)
    }

    fn all_nft_info(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<Extension>> {
        self.base_contract
            .all_nft_info(deps, env, token_id, include_expired)
    }
}
