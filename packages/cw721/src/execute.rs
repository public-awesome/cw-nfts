use cosmwasm_std::{
    Addr, Api, BankMsg, Binary, Coin, CustomMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult, Storage,
};
use cw_ownable::{none_or, Action, Ownership, OwnershipError};
use cw_storage_plus::Item;
use cw_utils::Expiration;

use crate::{
    error::Cw721ContractError,
    msg::{
        CollectionMetadataMsg, Cw721ExecuteMsg, Cw721InstantiateMsg, Cw721MigrateMsg, NftInfoMsg,
    },
    receiver::Cw721ReceiveMsg,
    state::{CollectionMetadata, Cw721Config, NftInfo, CREATOR, MINTER},
    traits::{Cw721CustomMsg, Cw721State, StateFactory},
    Approval, DefaultOptionCollectionMetadataExtension,
    DefaultOptionCollectionMetadataExtensionMsg, DefaultOptionNftMetadataExtension,
    DefaultOptionNftMetadataExtensionMsg,
};

pub trait Cw721Execute<
    // Metadata defined in NftInfo (used for mint).
    TNftMetadataExtension,
    // Message passed for updating metadata.
    TNftMetadataExtensionMsg,
    // Extension defined in CollectionMetadata.
    TCollectionMetadataExtension,
    // Message passed for updating collection info extension.
    TCollectionMetadataExtensionMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TNftMetadataExtension>,
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TCollectionMetadataExtension>,
    TCustomResponseMsg: CustomMsg,
{
    fn instantiate(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        msg: Cw721InstantiateMsg<TCollectionMetadataExtensionMsg>,
        contract_name: &str,
        contract_version: &str,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        cw2::set_contract_version(deps.storage, contract_name, contract_version)?;
        let config = Cw721Config::<
            TNftMetadataExtension,
            TNftMetadataExtensionMsg,
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            TCustomResponseMsg,
        >::default();

        // ---- update collection metadata before(!) creator and minter is set ----
        let collectin_metadata_msg = CollectionMetadataMsg {
            name: Some(msg.name),
            symbol: Some(msg.symbol),
            extension: msg.collection_metadata_extension,
        };
        let collection_metadata = collectin_metadata_msg.create(deps.as_ref(), env, info, None)?;
        config
            .collection_metadata
            .save(deps.storage, &collection_metadata)?;

        // ---- set minter and creator ----
        // use info.sender if None is passed
        let minter: &str = match msg.minter.as_deref() {
            Some(minter) => minter,
            None => info.sender.as_str(),
        };
        self.initialize_minter(deps.storage, deps.api, Some(minter))?;

        // use info.sender if None is passed
        let creator: &str = match msg.creator.as_deref() {
            Some(creator) => creator,
            None => info.sender.as_str(),
        };
        self.initialize_creator(deps.storage, deps.api, Some(creator))?;

        if let Some(withdraw_address) = msg.withdraw_address.clone() {
            let creator = deps.api.addr_validate(creator)?;
            self.set_withdraw_address(deps, &creator, withdraw_address)?;
        }

        Ok(Response::default()
            .add_attribute("minter", minter)
            .add_attribute("creator", creator))
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
            } => self.update_collection_metadata(deps, info, env, collection_metadata),
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
        let response = Response::<Empty>::default();
        // first migrate legacy data ...
        let response =
            migrate_legacy_minter_and_creator(deps.storage, deps.api, &env, &msg, response)?;
        let response = migrate_legacy_collection_metadata(deps.storage, &env, &msg, response)?;
        // ... then migrate
        let response = migrate_version(deps.storage, contract_name, contract_version, response)?;
        // ... and update creator and minter AFTER legacy migration
        let response = migrate_creator(deps.storage, deps.api, &env, &msg, response)?;
        let response = migrate_minter(deps.storage, deps.api, &env, &msg, response)?;
        Ok(response)
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
        _transfer_nft::<TNftMetadataExtension>(deps, env, info, &recipient, &token_id)?;

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
        // Transfer token
        _transfer_nft::<TNftMetadataExtension>(deps, env, info, &contract, &token_id)?;

        let send = Cw721ReceiveMsg {
            sender: info.sender.to_string(),
            token_id: token_id.clone(),
            msg,
        };

        // Send message
        Ok(Response::new()
            .add_message(send.into_cosmos_msg(contract.clone())?)
            .add_attribute("action", "send_nft")
            .add_attribute("sender", info.sender.to_string())
            .add_attribute("recipient", contract)
            .add_attribute("token_id", token_id))
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
        _update_approvals::<TNftMetadataExtension>(
            deps, env, info, &spender, &token_id, true, expires,
        )?;

        Ok(Response::new()
            .add_attribute("action", "approve")
            .add_attribute("sender", info.sender.to_string())
            .add_attribute("spender", spender)
            .add_attribute("token_id", token_id))
    }

    fn revoke(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        _update_approvals::<TNftMetadataExtension>(
            deps, env, info, &spender, &token_id, false, None,
        )?;

        Ok(Response::new()
            .add_attribute("action", "revoke")
            .add_attribute("sender", info.sender.to_string())
            .add_attribute("spender", spender)
            .add_attribute("token_id", token_id))
    }

    fn approve_all(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        // reject expired data as invalid
        let expires = expires.unwrap_or_default();
        if expires.is_expired(&env.block) {
            return Err(Cw721ContractError::Expired {});
        }

        // set the operator for us
        let operator_addr = deps.api.addr_validate(&operator)?;
        let config = Cw721Config::<
            TNftMetadataExtension,
            TNftMetadataExtensionMsg,
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            TCustomResponseMsg,
        >::default();
        config
            .operators
            // stores info.sender as key (=granter, NFT owner) and operator as value (operator only(!) has control over NFTs of granter)
            // check is done in `check_can_send()`
            .save(deps.storage, (&info.sender, &operator_addr), &expires)?;

        Ok(Response::new()
            .add_attribute("action", "approve_all")
            .add_attribute("sender", info.sender.to_string())
            .add_attribute("operator", operator))
    }

    fn revoke_all(
        &self,
        deps: DepsMut,
        _env: &Env,
        info: &MessageInfo,
        operator: String,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        let operator_addr = deps.api.addr_validate(&operator)?;
        let config = Cw721Config::<
            TNftMetadataExtension,
            TNftMetadataExtensionMsg,
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            TCustomResponseMsg,
        >::default();
        config
            .operators
            .remove(deps.storage, (&info.sender, &operator_addr));

        Ok(Response::new()
            .add_attribute("action", "revoke_all")
            .add_attribute("sender", info.sender.to_string())
            .add_attribute("operator", operator))
    }

    fn burn_nft(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        token_id: String,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        let config = Cw721Config::<
            TNftMetadataExtension,
            TNftMetadataExtensionMsg,
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            TCustomResponseMsg,
        >::default();
        let token = config.nft_info.load(deps.storage, &token_id)?;
        check_can_send(deps.as_ref(), env, info, &token)?;

        config.nft_info.remove(deps.storage, &token_id)?;
        config.decrement_tokens(deps.storage)?;

        Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("sender", info.sender.to_string())
            .add_attribute("token_id", token_id))
    }

    // ------- opionated cw721 functions -------
    fn initialize_creator(
        &self,
        storage: &mut dyn Storage,
        api: &dyn Api,
        creator: Option<&str>,
    ) -> StdResult<Ownership<Addr>> {
        CREATOR.initialize_owner(storage, api, creator)
    }

    fn initialize_minter(
        &self,
        storage: &mut dyn Storage,
        api: &dyn Api,
        minter: Option<&str>,
    ) -> StdResult<Ownership<Addr>> {
        MINTER.initialize_owner(storage, api, minter)
    }

    fn update_collection_metadata(
        &self,
        deps: DepsMut,
        info: &MessageInfo,
        env: &Env,
        msg: CollectionMetadataMsg<TCollectionMetadataExtensionMsg>,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        let config = Cw721Config::<
            TNftMetadataExtension,
            TNftMetadataExtensionMsg,
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            TCustomResponseMsg,
        >::default();
        let current = config.collection_metadata.load(deps.storage)?;
        let collection_metadata = msg.create(deps.as_ref(), env, info, Some(&current))?;
        config
            .collection_metadata
            .save(deps.storage, &collection_metadata)?;

        Ok(Response::new()
            .add_attribute("action", "update_collection_metadata")
            .add_attribute("sender", info.sender.to_string()))
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
        MINTER.assert_owner(deps.storage, &info.sender)?;
        // create the token
        let token_msg = NftInfoMsg {
            owner: owner.clone(),
            approvals: vec![],
            token_uri: token_uri.clone(),
            extension,
        };
        let token = token_msg.create(deps.as_ref(), env, info, None)?;
        let config = Cw721Config::<
            TNftMetadataExtension,
            TNftMetadataExtensionMsg,
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            TCustomResponseMsg,
        >::default();
        config
            .nft_info
            .update(deps.storage, &token_id, |old| match old {
                Some(_) => Err(Cw721ContractError::Claimed {}),
                None => Ok(token),
            })?;

        config.increment_tokens(deps.storage)?;

        let mut res = Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.sender.to_string())
            .add_attribute("owner", owner)
            .add_attribute("token_id", token_id);
        if let Some(token_uri) = token_uri {
            res = res.add_attribute("token_uri", token_uri);
        }
        Ok(res)
    }

    fn update_minter_ownership(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        env: &Env,
        info: &MessageInfo,
        action: Action,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        let ownership = MINTER.update_ownership(api, storage, &env.block, &info.sender, action)?;
        Ok(Response::new()
            .add_attribute("update_minter_ownership", info.sender.to_string())
            .add_attributes(ownership.into_attributes()))
    }

    fn update_creator_ownership(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        env: &Env,
        info: &MessageInfo,
        action: Action,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        let ownership = CREATOR.update_ownership(api, storage, &env.block, &info.sender, action)?;
        Ok(Response::new()
            .add_attribute("update_creator_ownership", info.sender.to_string())
            .add_attributes(ownership.into_attributes()))
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
        CREATOR.assert_owner(deps.storage, &info.sender)?;
        let contract = Cw721Config::<
            TNftMetadataExtension,
            TNftMetadataExtensionMsg,
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            Empty,
        >::default();
        let current_nft_info = contract.nft_info.load(deps.storage, &token_id)?;
        let nft_info_msg = NftInfoMsg {
            owner: current_nft_info.owner.to_string(),
            approvals: current_nft_info.approvals.clone(),
            token_uri: token_uri.clone(),
            extension: msg,
        };
        let updated = nft_info_msg.create(deps.as_ref(), env, info, Some(&current_nft_info))?;
        contract.nft_info.save(deps.storage, &token_id, &updated)?;
        Ok(Response::new()
            .add_attribute("action", "update_metadata")
            .add_attribute("token_id", token_id))
    }

    fn set_withdraw_address(
        &self,
        deps: DepsMut,
        sender: &Addr,
        address: String,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        CREATOR.assert_owner(deps.storage, sender)?;
        deps.api.addr_validate(&address)?;
        let config = Cw721Config::<
            TNftMetadataExtension,
            TNftMetadataExtensionMsg,
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            TCustomResponseMsg,
        >::default();
        config.withdraw_address.save(deps.storage, &address)?;
        Ok(Response::new()
            .add_attribute("action", "set_withdraw_address")
            .add_attribute("address", address))
    }

    fn remove_withdraw_address(
        &self,
        storage: &mut dyn Storage,
        sender: &Addr,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        CREATOR.assert_owner(storage, sender)?;
        let config = Cw721Config::<
            TNftMetadataExtension,
            TNftMetadataExtensionMsg,
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            TCustomResponseMsg,
        >::default();
        let address = config.withdraw_address.may_load(storage)?;
        match address {
            Some(address) => {
                config.withdraw_address.remove(storage);
                Ok(Response::new()
                    .add_attribute("action", "remove_withdraw_address")
                    .add_attribute("address", address))
            }
            None => Err(Cw721ContractError::NoWithdrawAddress {}),
        }
    }

    fn withdraw_funds(
        &self,
        storage: &mut dyn Storage,
        amount: &Coin,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        let withdraw_address = Cw721Config::<
            TNftMetadataExtension,
            TNftMetadataExtensionMsg,
            TCollectionMetadataExtension,
            TCollectionMetadataExtensionMsg,
            TCustomResponseMsg,
        >::default()
        .withdraw_address
        .may_load(storage)?;
        match withdraw_address {
            Some(address) => {
                let msg = BankMsg::Send {
                    to_address: address,
                    amount: vec![amount.clone()],
                };
                Ok(Response::new()
                    .add_message(msg)
                    .add_attribute("action", "withdraw_funds")
                    .add_attribute("amount", amount.amount.to_string())
                    .add_attribute("denom", amount.denom.to_string()))
            }
            None => Err(Cw721ContractError::NoWithdrawAddress {}),
        }
    }
}

// ------- helper cw721 functions -------
fn _transfer_nft<TNftMetadataExtension>(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    recipient: &str,
    token_id: &str,
) -> Result<NftInfo<TNftMetadataExtension>, Cw721ContractError>
where
    TNftMetadataExtension: Cw721State,
{
    let config = Cw721Config::<TNftMetadataExtension, Empty, Empty, Empty, Empty>::default();
    let mut token = config.nft_info.load(deps.storage, token_id)?;
    // ensure we have permissions
    check_can_send(deps.as_ref(), env, info, &token)?;
    // set owner and remove existing approvals
    token.owner = deps.api.addr_validate(recipient)?;
    token.approvals = vec![];
    config.nft_info.save(deps.storage, token_id, &token)?;
    Ok(token)
}

#[allow(clippy::too_many_arguments)]
fn _update_approvals<TNftMetadataExtension>(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    spender: &str,
    token_id: &str,
    // if add == false, remove. if add == true, remove then set with this expiration
    add: bool,
    expires: Option<Expiration>,
) -> Result<NftInfo<TNftMetadataExtension>, Cw721ContractError>
where
    TNftMetadataExtension: Cw721State,
{
    let config = Cw721Config::<TNftMetadataExtension, Empty, Empty, Empty, Empty>::default();
    let mut token = config.nft_info.load(deps.storage, token_id)?;
    // ensure we have permissions
    check_can_approve(deps.as_ref(), env, info, &token)?;

    // update the approval list (remove any for the same spender before adding)
    let spender_addr = deps.api.addr_validate(spender)?;
    token.approvals.retain(|apr| apr.spender != spender_addr);

    // only difference between approve and revoke
    if add {
        // reject expired data as invalid
        let expires = expires.unwrap_or_default();
        if expires.is_expired(&env.block) {
            return Err(Cw721ContractError::Expired {});
        }
        let approval = Approval {
            spender: spender_addr,
            expires,
        };
        token.approvals.push(approval);
    }

    config.nft_info.save(deps.storage, token_id, &token)?;

    Ok(token)
}

/// returns true if the sender can execute approve or reject on the contract
pub fn check_can_approve<TNftMetadataExtension>(
    deps: Deps,
    env: &Env,
    info: &MessageInfo,
    token: &NftInfo<TNftMetadataExtension>,
) -> Result<(), Cw721ContractError>
where
    TNftMetadataExtension: Cw721State,
{
    // owner can approve
    if token.owner == info.sender {
        return Ok(());
    }
    // operator can approve
    let config = Cw721Config::<TNftMetadataExtension, Empty, Empty, Empty, Empty>::default();
    let op = config
        .operators
        .may_load(deps.storage, (&token.owner, &info.sender))?;
    match op {
        Some(ex) => {
            if ex.is_expired(&env.block) {
                Err(Cw721ContractError::Ownership(OwnershipError::NotOwner))
            } else {
                Ok(())
            }
        }
        None => Err(Cw721ContractError::Ownership(OwnershipError::NotOwner)),
    }
}

/// returns true iff the sender can transfer ownership of the token
pub fn check_can_send<TNftMetadataExtension>(
    deps: Deps,
    env: &Env,
    info: &MessageInfo,
    token: &NftInfo<TNftMetadataExtension>,
) -> Result<(), Cw721ContractError> {
    // owner can send
    if token.owner == info.sender {
        return Ok(());
    }

    // any non-expired token approval can send
    if token
        .approvals
        .iter()
        .any(|apr| apr.spender == info.sender && !apr.is_expired(&env.block))
    {
        return Ok(());
    }

    // operator can send
    let config = Cw721Config::<Empty, Empty, Empty, Empty, Empty>::default();
    let op = config
        .operators
        // has token owner approved/gave grant to sender for full control over owner's NFTs?
        .may_load(deps.storage, (&token.owner, &info.sender))?;

    match op {
        Some(ex) => {
            if ex.is_expired(&env.block) {
                Err(Cw721ContractError::Ownership(OwnershipError::NotOwner))
            } else {
                Ok(())
            }
        }
        None => Err(Cw721ContractError::Ownership(OwnershipError::NotOwner)),
    }
}

// ------- migrate -------
pub fn migrate_version(
    storage: &mut dyn Storage,
    contradct_name: &str,
    contract_version: &str,
    response: Response,
) -> StdResult<Response> {
    let response = response
        .add_attribute("from_version", cw2::get_contract_version(storage)?.version)
        .add_attribute("to_version", contract_version);

    // update contract version
    cw2::set_contract_version(storage, contradct_name, contract_version)?;
    Ok(response)
}

pub fn migrate_creator(
    storage: &mut dyn Storage,
    api: &dyn Api,
    _env: &Env,
    msg: &Cw721MigrateMsg,
    response: Response,
) -> StdResult<Response> {
    match msg {
        Cw721MigrateMsg::WithUpdate { creator, .. } => {
            if let Some(creator) = creator {
                CREATOR.initialize_owner(storage, api, Some(creator.as_str()))?;
                return Ok(response.add_attribute("creator", creator));
            }
        }
    }
    Ok(response)
}

pub fn migrate_minter(
    storage: &mut dyn Storage,
    api: &dyn Api,
    _env: &Env,
    msg: &Cw721MigrateMsg,
    response: Response,
) -> StdResult<Response> {
    match msg {
        Cw721MigrateMsg::WithUpdate { minter, .. } => {
            if let Some(minter) = minter {
                MINTER.initialize_owner(storage, api, Some(minter.as_str()))?;
                return Ok(response.add_attribute("creator", minter));
            }
        }
    }
    Ok(response)
}

/// Migrates only in case ownership is not present
/// !!! Important note here: !!!
/// - creator owns the contract and can update collection info
/// - minter can mint new tokens
///
/// Before v0.19.0 there were confusing naming conventions:
/// - v0.17.0: minter was replaced by cw_ownable, as a result minter is owner
/// - v0.16.0 and below: minter was stored in dedicated `minter` store (so NOT using cw_ownable at all)
pub fn migrate_legacy_minter_and_creator(
    storage: &mut dyn Storage,
    api: &dyn Api,
    _env: &Env,
    _msg: &Cw721MigrateMsg,
    response: Response,
) -> Result<Response, Cw721ContractError> {
    let minter = MINTER.item.may_load(storage)?;
    // no migration in case minter is already set
    if minter.is_some() {
        return Ok(response);
    }
    // in v0.17/18 cw_ownable::OWNERSHIP was used for minter, now it is used for creator
    let ownership_previously_used_as_minter = CREATOR.item.may_load(storage)?;
    let creator_and_minter = match ownership_previously_used_as_minter {
        // v0.18 migration
        Some(ownership) => {
            // owner is used for both: creator and minter
            // since it is already set for creator, we only need to migrate minter
            let owner = ownership.owner.map(|a| a.to_string());
            MINTER.initialize_owner(storage, api, owner.as_deref())?;
            owner
        }
        // v0.17 and older migration
        None => {
            let legacy_minter_store: Item<Addr> = Item::new("minter");
            let legacy_minter = legacy_minter_store.load(storage)?;
            MINTER.initialize_owner(storage, api, Some(legacy_minter.as_str()))?;
            CREATOR.initialize_owner(storage, api, Some(legacy_minter.as_str()))?;
            Some(legacy_minter.to_string())
        }
    };
    Ok(response.add_attribute("creator_and_minter", none_or(creator_and_minter.as_ref())))
}

/// Migrates only in case collection_metadata is not present
pub fn migrate_legacy_collection_metadata(
    storage: &mut dyn Storage,
    env: &Env,
    _msg: &Cw721MigrateMsg,
    response: Response,
) -> Result<Response, Cw721ContractError> {
    let contract = Cw721Config::<
        DefaultOptionNftMetadataExtension,
        DefaultOptionNftMetadataExtensionMsg,
        DefaultOptionCollectionMetadataExtension,
        DefaultOptionCollectionMetadataExtensionMsg,
        Empty,
    >::default();
    match contract.collection_metadata.may_load(storage)? {
        Some(_) => Ok(response),
        None => {
            // contract info is legacy collection info
            let legacy_collection_metadata_store: Item<cw721_016::ContractInfoResponse> =
                Item::new("nft_info");
            let legacy_collection_metadata = legacy_collection_metadata_store.load(storage)?;
            let collection_metadata = CollectionMetadata {
                name: legacy_collection_metadata.name.clone(),
                symbol: legacy_collection_metadata.symbol.clone(),
                extension: None,
                updated_at: env.block.time,
            };
            contract
                .collection_metadata
                .save(storage, &collection_metadata)?;
            Ok(response
                .add_attribute("migrated collection name", legacy_collection_metadata.name)
                .add_attribute(
                    "migrated collection symbol",
                    legacy_collection_metadata.symbol,
                ))
        }
    }
}
