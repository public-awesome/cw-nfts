use std::fmt::Debug;

use cosmwasm_std::{
    to_json_binary, Addr, Api, Binary, Coin, CustomMsg, Deps, DepsMut, Empty, Env, MessageInfo,
    Response, StdResult, Storage,
};
use cw_ownable::{Action, Ownership};
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    error::Cw721ContractError,
    execute::{
        approve, approve_all, burn_nft, initialize_creator, initialize_minter, instantiate,
        instantiate_with_version, migrate, mint, remove_withdraw_address, revoke, revoke_all,
        send_nft, set_withdraw_address, transfer_nft, update_collection_metadata,
        update_creator_ownership, update_minter_ownership, update_nft_info, withdraw_funds,
    },
    msg::{
        AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, CollectionMetadataMsg,
        Cw721ExecuteMsg, Cw721InstantiateMsg, Cw721MigrateMsg, Cw721QueryMsg, MinterResponse,
        NftInfoResponse, NumTokensResponse, OperatorResponse, OperatorsResponse, OwnerOfResponse,
        TokensResponse,
    },
    query::{
        query_all_nft_info, query_all_tokens, query_approval, query_approvals,
        query_collection_metadata, query_collection_metadata_and_extension,
        query_collection_metadata_extension, query_creator_ownership, query_minter,
        query_minter_ownership, query_nft_info, query_num_tokens, query_operator, query_operators,
        query_owner_of, query_tokens, query_withdraw_address,
    },
    state::CollectionMetadata,
    Attribute, CollectionMetadataAndExtension,
};

/// This is an exact copy of `CustomMsg`, since implementing a trait for a type from another crate is not possible.
///
/// Possible:
/// `impl<T> Cw721CustomMsg for Option<T> where T: Cw721CustomMsg {}`
///
/// Not possible:
/// `impl<T> CustomMsg for Option<T> where T: CustomMsg {}`
///
/// This will be removed once the `CustomMsg` trait is moved to the `cosmwasm_std` crate: https://github.com/CosmWasm/cosmwasm/issues/2056
pub trait Cw721CustomMsg: Serialize + Clone + Debug + PartialEq + JsonSchema {}

pub trait Cw721State: Serialize + DeserializeOwned + Clone + Debug {}

impl Cw721State for Empty {}
impl<T> Cw721State for Option<T> where T: Cw721State {}
impl Cw721CustomMsg for Empty {}
impl<T> Cw721CustomMsg for Option<T> where T: Cw721CustomMsg {}

pub trait StateFactory<TState> {
    fn create(
        &self,
        deps: Option<Deps>,
        env: Option<&Env>,
        info: Option<&MessageInfo>,
        current: Option<&TState>,
    ) -> Result<TState, Cw721ContractError>;
    fn validate(
        &self,
        deps: Option<Deps>,
        env: Option<&Env>,
        info: Option<&MessageInfo>,
        current: Option<&TState>,
    ) -> Result<(), Cw721ContractError>;
}

impl StateFactory<Empty> for Empty {
    fn create(
        &self,
        _deps: Option<Deps>,
        _env: Option<&Env>,
        _info: Option<&MessageInfo>,
        _current: Option<&Empty>,
    ) -> Result<Empty, Cw721ContractError> {
        Ok(Empty {})
    }

    fn validate(
        &self,
        _deps: Option<Deps>,
        _env: Option<&Env>,
        _info: Option<&MessageInfo>,
        _current: Option<&Empty>,
    ) -> Result<(), Cw721ContractError> {
        Ok(())
    }
}

pub trait ToAttributesState {
    fn to_attributes_states(&self) -> Result<Vec<Attribute>, Cw721ContractError>;
}

impl<T> ToAttributesState for Option<T>
where
    T: ToAttributesState,
{
    fn to_attributes_states(&self) -> Result<Vec<Attribute>, Cw721ContractError> {
        match self {
            Some(inner) => inner.to_attributes_states(),
            None => Ok(vec![]),
        }
    }
}

pub trait FromAttributesState: Sized {
    fn from_attributes_state(value: &[Attribute]) -> Result<Self, Cw721ContractError>;
}

impl<T> FromAttributesState for Option<T>
where
    T: FromAttributesState,
{
    fn from_attributes_state(value: &[Attribute]) -> Result<Self, Cw721ContractError> {
        if value.is_empty() {
            Ok(None)
        } else {
            T::from_attributes_state(value).map(Some)
        }
    }
}

pub trait Cw721Execute<
    // Metadata defined in NftInfo (used for mint).
    TNftMetadataExtension,
    // Message passed for updating metadata.
    TNftMetadataExtensionMsg,
    // Extension defined in CollectionMetadata.
    TCollectionMetadataExtension,
    // Message passed for updating collection metadata extension.
    TCollectionMetadataExtensionMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TNftMetadataExtension>,
    TCollectionMetadataExtension: Cw721State + ToAttributesState + FromAttributesState,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TCollectionMetadataExtension>,
    TCustomResponseMsg: CustomMsg,
{
    fn instantiate_with_version(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        msg: Cw721InstantiateMsg<TCollectionMetadataExtensionMsg>,
        contract_name: &str,
        contract_version: &str,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        instantiate_with_version(deps, env, info, msg, contract_name, contract_version)
    }

    fn instantiate(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        msg: Cw721InstantiateMsg<TCollectionMetadataExtensionMsg>,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        instantiate(deps, env, info, msg)
    }

    fn execute(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        msg: Cw721ExecuteMsg<TNftMetadataExtensionMsg, TCollectionMetadataExtensionMsg>,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        match msg {
            Cw721ExecuteMsg::UpdateCollectionMetadata {
                collection_metadata,
            } => {
                self.update_collection_metadata(deps, info.into(), env.into(), collection_metadata)
            }
            Cw721ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => self.mint(deps, env, info, token_id, owner, token_uri, extension),
            Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => self.approve(deps, env, info, spender, token_id, expires),
            Cw721ExecuteMsg::Revoke { spender, token_id } => {
                self.revoke(deps, env, info, spender, token_id)
            }
            Cw721ExecuteMsg::ApproveAll { operator, expires } => {
                self.approve_all(deps, env, info, operator, expires)
            }
            Cw721ExecuteMsg::RevokeAll { operator } => self.revoke_all(deps, env, info, operator),
            Cw721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => self.transfer_nft(deps, env, info, recipient, token_id),
            Cw721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => self.send_nft(deps, env, info, contract, token_id, msg),
            Cw721ExecuteMsg::Burn { token_id } => self.burn_nft(deps, env, info, token_id),
            #[allow(deprecated)]
            Cw721ExecuteMsg::UpdateOwnership(action) => {
                self.update_minter_ownership(deps.api, deps.storage, env, info, action)
            }
            Cw721ExecuteMsg::UpdateMinterOwnership(action) => {
                self.update_minter_ownership(deps.api, deps.storage, env, info, action)
            }
            Cw721ExecuteMsg::UpdateCreatorOwnership(action) => {
                self.update_creator_ownership(deps.api, deps.storage, env, info, action)
            }
            #[allow(deprecated)]
            Cw721ExecuteMsg::Extension { msg } => {
                self.update_legacy_extension(deps, env, info, msg)
            }
            Cw721ExecuteMsg::UpdateNftInfo {
                token_id,
                token_uri,
                extension,
            } => self.update_nft_info(deps, env, info, token_id, token_uri, extension),
            Cw721ExecuteMsg::SetWithdrawAddress { address } => {
                self.set_withdraw_address(deps, &info.sender, address)
            }
            Cw721ExecuteMsg::RemoveWithdrawAddress {} => {
                self.remove_withdraw_address(deps.storage, &info.sender)
            }
            Cw721ExecuteMsg::WithdrawFunds { amount } => self.withdraw_funds(deps.storage, &amount),
        }
    }

    fn migrate(
        &self,
        deps: DepsMut,
        env: Env,
        msg: Cw721MigrateMsg,
        contract_name: &str,
        contract_version: &str,
    ) -> Result<Response, Cw721ContractError> {
        migrate(deps, env, msg, contract_name, contract_version)
    }

    // ------- ERC721-based functions -------
    fn transfer_nft(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        transfer_nft::<TNftMetadataExtension>(deps, env, info, &recipient, &token_id)?;

        Ok(Response::new()
            .add_attribute("action", "transfer_nft")
            .add_attribute("sender", info.sender.to_string())
            .add_attribute("recipient", recipient)
            .add_attribute("token_id", token_id))
    }

    fn send_nft(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        send_nft::<TNftMetadataExtension, TCustomResponseMsg>(
            deps, env, info, contract, token_id, msg,
        )
    }

    fn approve(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        approve::<TNftMetadataExtension, TCustomResponseMsg>(
            deps, env, info, spender, token_id, expires,
        )
    }

    fn revoke(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        revoke::<TNftMetadataExtension, TCustomResponseMsg>(deps, env, info, spender, token_id)
    }

    fn approve_all(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        approve_all::<TCustomResponseMsg>(deps, env, info, operator, expires)
    }

    fn revoke_all(
        &self,
        deps: DepsMut,
        _env: &Env,
        info: &MessageInfo,
        operator: String,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        revoke_all::<TCustomResponseMsg>(deps, _env, info, operator)
    }

    fn burn_nft(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        token_id: String,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        burn_nft::<TCustomResponseMsg>(deps, env, info, token_id)
    }

    // ------- opionated cw721 functions -------
    fn initialize_creator(
        &self,
        storage: &mut dyn Storage,
        api: &dyn Api,
        creator: Option<&str>,
    ) -> StdResult<Ownership<Addr>> {
        initialize_creator(storage, api, creator)
    }

    fn initialize_minter(
        &self,
        storage: &mut dyn Storage,
        api: &dyn Api,
        minter: Option<&str>,
    ) -> StdResult<Ownership<Addr>> {
        initialize_minter(storage, api, minter)
    }

    fn update_collection_metadata(
        &self,
        deps: DepsMut,
        info: Option<&MessageInfo>,
        env: Option<&Env>,
        msg: CollectionMetadataMsg<TCollectionMetadataExtensionMsg>,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        update_collection_metadata::<
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            TCustomResponseMsg,
        >(deps, info, env, msg)
    }

    #[allow(clippy::too_many_arguments)]
    fn mint(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: TNftMetadataExtensionMsg,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        mint::<TNftMetadataExtension, TNftMetadataExtensionMsg, TCustomResponseMsg>(
            deps, env, info, token_id, owner, token_uri, extension,
        )
    }

    fn update_minter_ownership(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        env: &Env,
        info: &MessageInfo,
        action: Action,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        update_minter_ownership::<TCustomResponseMsg>(api, storage, env, info, action)
    }

    fn update_creator_ownership(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        env: &Env,
        info: &MessageInfo,
        action: Action,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        update_creator_ownership::<TCustomResponseMsg>(api, storage, env, info, action)
    }

    /// Allows creator to update onchain metadata. For now this is a no-op.
    fn update_legacy_extension(
        &self,
        _deps: DepsMut,
        _env: &Env,
        _info: &MessageInfo,
        _msg: TNftMetadataExtensionMsg,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        panic!("deprecated. pls use update_metadata instead.")
    }

    /// The creator is the only one eligible to update NFT's token uri and onchain metadata (`NftInfo.extension`).
    /// NOTE: approvals and owner are not affected by this call, since they belong to the NFT owner.
    fn update_nft_info(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        token_id: String,
        token_uri: Option<String>,
        msg: TNftMetadataExtensionMsg,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        update_nft_info::<TNftMetadataExtension, TNftMetadataExtensionMsg, TCustomResponseMsg>(
            deps,
            env.into(),
            info.into(),
            token_id,
            token_uri,
            msg,
        )
    }

    fn set_withdraw_address(
        &self,
        deps: DepsMut,
        sender: &Addr,
        address: String,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        set_withdraw_address::<TCustomResponseMsg>(deps, sender, address)
    }

    fn remove_withdraw_address(
        &self,
        storage: &mut dyn Storage,
        sender: &Addr,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        remove_withdraw_address::<TCustomResponseMsg>(storage, sender)
    }

    fn withdraw_funds(
        &self,
        storage: &mut dyn Storage,
        amount: &Coin,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        withdraw_funds::<TCustomResponseMsg>(storage, amount)
    }
}

pub trait Cw721Query<
    // Metadata defined in NftInfo.
    TNftMetadataExtension,
    // Extension defined in CollectionMetadata.
    TCollectionMetadataExtension,
> where
    TNftMetadataExtension: Cw721State,
    TCollectionMetadataExtension: Cw721State + FromAttributesState,
{
    fn query(
        &self,
        deps: Deps,
        env: &Env,
        msg: Cw721QueryMsg<TNftMetadataExtension, TCollectionMetadataExtension>,
    ) -> Result<Binary, Cw721ContractError> {
        match msg {
            #[allow(deprecated)]
            Cw721QueryMsg::Minter {} => Ok(to_json_binary(&self.query_minter(deps.storage)?)?),
            #[allow(deprecated)]
            Cw721QueryMsg::ContractInfo {} => Ok(to_json_binary(
                &self.query_collection_metadata_and_extension(deps)?,
            )?),
            Cw721QueryMsg::GetCollectionMetadata {} => Ok(to_json_binary(
                &self.query_collection_metadata_and_extension(deps)?,
            )?),
            Cw721QueryMsg::NftInfo { token_id } => {
                Ok(to_json_binary(&self.query_nft_info(deps, env, token_id)?)?)
            }
            Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Ok(to_json_binary(&self.query_owner_of(
                deps,
                env,
                token_id,
                include_expired.unwrap_or(false),
            )?)?),
            Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Ok(to_json_binary(&self.query_all_nft_info(
                deps,
                env,
                token_id,
                include_expired.unwrap_or(false),
            )?)?),
            Cw721QueryMsg::Operator {
                owner,
                operator,
                include_expired,
            } => Ok(to_json_binary(&self.query_operator(
                deps,
                env,
                owner,
                operator,
                include_expired.unwrap_or(false),
            )?)?),
            Cw721QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => Ok(to_json_binary(&self.query_operators(
                deps,
                env,
                owner,
                include_expired.unwrap_or(false),
                start_after,
                limit,
            )?)?),
            Cw721QueryMsg::NumTokens {} => Ok(to_json_binary(&self.query_num_tokens(deps, env)?)?),
            Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Ok(to_json_binary(&self.query_tokens(
                deps,
                env,
                owner,
                start_after,
                limit,
            )?)?),
            Cw721QueryMsg::AllTokens { start_after, limit } => Ok(to_json_binary(
                &self.query_all_tokens(deps, env, start_after, limit)?,
            )?),
            Cw721QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => Ok(to_json_binary(&self.query_approval(
                deps,
                env,
                token_id,
                spender,
                include_expired.unwrap_or(false),
            )?)?),
            Cw721QueryMsg::Approvals {
                token_id,
                include_expired,
            } => Ok(to_json_binary(&self.query_approvals(
                deps,
                env,
                token_id,
                include_expired.unwrap_or(false),
            )?)?),
            #[allow(deprecated)]
            Cw721QueryMsg::Ownership {} => {
                Ok(to_json_binary(&self.query_minter_ownership(deps.storage)?)?)
            }
            Cw721QueryMsg::GetMinterOwnership {} => {
                Ok(to_json_binary(&self.query_minter_ownership(deps.storage)?)?)
            }
            Cw721QueryMsg::GetCreatorOwnership {} => Ok(to_json_binary(
                &self.query_creator_ownership(deps.storage)?,
            )?),
            #[allow(deprecated)]
            Cw721QueryMsg::Extension { msg } => {
                Ok(to_json_binary(&self.query_nft_metadata(deps, env, msg)?)?)
            }
            Cw721QueryMsg::GetNftMetadata { msg } => {
                Ok(to_json_binary(&self.query_nft_metadata(deps, env, msg)?)?)
            }
            Cw721QueryMsg::GetCollectionMetadataExtension { msg } => Ok(to_json_binary(
                &self.query_custom_collection_metadata_extension(deps, env, msg)?,
            )?),
            Cw721QueryMsg::GetWithdrawAddress {} => {
                Ok(to_json_binary(&self.query_withdraw_address(deps)?)?)
            }
        }
    }

    #[deprecated(since = "0.19.0", note = "Please use query_minter_ownership instead")]
    /// Deprecated: use query_minter_ownership instead! Will be removed in next release!
    fn query_minter(&self, storage: &dyn Storage) -> StdResult<MinterResponse> {
        #[allow(deprecated)]
        query_minter(storage)
    }

    fn query_minter_ownership(&self, storage: &dyn Storage) -> StdResult<Ownership<Addr>> {
        query_minter_ownership(storage)
    }

    fn query_creator_ownership(&self, storage: &dyn Storage) -> StdResult<Ownership<Addr>> {
        query_creator_ownership(storage)
    }

    fn query_collection_metadata(deps: Deps) -> StdResult<CollectionMetadata> {
        query_collection_metadata(deps.storage)
    }

    fn query_collection_metadata_extension(deps: Deps) -> StdResult<Vec<Attribute>> {
        query_collection_metadata_extension(deps)
    }

    fn query_collection_metadata_and_extension(
        &self,
        deps: Deps,
    ) -> Result<CollectionMetadataAndExtension<TCollectionMetadataExtension>, Cw721ContractError>
    where
        TCollectionMetadataExtension: FromAttributesState,
    {
        query_collection_metadata_and_extension(deps)
    }

    fn query_num_tokens(&self, deps: Deps, env: &Env) -> StdResult<NumTokensResponse> {
        query_num_tokens(deps, env)
    }

    fn query_nft_info(
        &self,
        deps: Deps,
        env: &Env,
        token_id: String,
    ) -> StdResult<NftInfoResponse<TNftMetadataExtension>> {
        query_nft_info::<TNftMetadataExtension>(deps, env, token_id)
    }

    fn query_owner_of(
        &self,
        deps: Deps,
        env: &Env,
        token_id: String,
        include_expired_approval: bool,
    ) -> StdResult<OwnerOfResponse> {
        query_owner_of(deps, env, token_id, include_expired_approval)
    }

    /// operator returns the approval status of an operator for a given owner if exists
    fn query_operator(
        &self,
        deps: Deps,
        env: &Env,
        owner: String,
        operator: String,
        include_expired_approval: bool,
    ) -> StdResult<OperatorResponse> {
        query_operator(deps, env, owner, operator, include_expired_approval)
    }

    /// operators returns all operators owner given access to
    fn query_operators(
        &self,
        deps: Deps,
        env: &Env,
        owner: String,
        include_expired_approval: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse> {
        query_operators(
            deps,
            env,
            owner,
            include_expired_approval,
            start_after,
            limit,
        )
    }

    fn query_approval(
        &self,
        deps: Deps,
        env: &Env,
        token_id: String,
        spender: String,
        include_expired_approval: bool,
    ) -> StdResult<ApprovalResponse> {
        query_approval(deps, env, token_id, spender, include_expired_approval)
    }

    /// approvals returns all approvals owner given access to
    fn query_approvals(
        &self,
        deps: Deps,
        env: &Env,
        token_id: String,
        include_expired_approval: bool,
    ) -> StdResult<ApprovalsResponse> {
        query_approvals(deps, env, token_id, include_expired_approval)
    }

    fn query_tokens(
        &self,
        deps: Deps,
        _env: &Env,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        query_tokens(deps, _env, owner, start_after, limit)
    }

    fn query_all_tokens(
        &self,
        deps: Deps,
        _env: &Env,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        query_all_tokens(deps, _env, start_after, limit)
    }

    fn query_all_nft_info(
        &self,
        deps: Deps,
        env: &Env,
        token_id: String,
        include_expired_approval: bool,
    ) -> StdResult<AllNftInfoResponse<TNftMetadataExtension>> {
        query_all_nft_info::<TNftMetadataExtension>(deps, env, token_id, include_expired_approval)
    }

    /// Use NftInfo instead.
    /// No-op / empty extension query returning empty binary, needed for inferring type parameter during compile.
    ///
    /// Note: it may be extended in case there are use cases e.g. for specific NFT metadata query.
    fn query_nft_metadata(
        &self,
        _deps: Deps,
        _env: &Env,
        _msg: TNftMetadataExtension,
    ) -> StdResult<Binary> {
        Ok(Binary::default())
    }

    /// Use GetCollectionMetadata instead.
    /// No-op / empty extension query returning empty binary, needed for inferring type parameter during compile
    ///
    /// Note: it may be extended in case there are use cases e.g. for specific NFT metadata query.
    fn query_custom_collection_metadata_extension(
        &self,
        _deps: Deps,
        _env: &Env,
        _msg: TCollectionMetadataExtension,
    ) -> StdResult<Binary> {
        Ok(Binary::default())
    }

    fn query_withdraw_address(&self, deps: Deps) -> StdResult<Option<String>> {
        query_withdraw_address(deps)
    }
}
