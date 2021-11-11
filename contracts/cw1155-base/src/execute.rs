use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64};

use cw1155::{ContractInfoResponse, CustomMsg, Cw1155Execute, Cw1155ReceiveMsg, Expiration};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MintMsg};
use crate::state::{Approval, Cw1155Contract, OwnerInfo, TokenInfo};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw1155-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a, T, C> Cw1155Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response<C>> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let info = ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };
        self.contract_info.save(deps.storage, &info)?;
        let minter = deps.api.addr_validate(&msg.minter)?;
        self.minter.save(deps.storage, &minter)?;
        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<T>,
    ) -> Result<Response<C>, ContractError> {
        match msg {
            ExecuteMsg::Mint(msg) => self.mint(deps, env, info, msg),
            ExecuteMsg::IncreaseAllowance {
                spender,
                token_id,
                amount,
                expires,
            } => self.increase_allowance(deps, env, info, spender, token_id, amount, expires),
            ExecuteMsg::DecreaseAllowance {
                spender,
                token_id,
                amount,
                expires,
            } => self.decrease_allowance(deps, env, info, spender, token_id, amount, expires),
            ExecuteMsg::ApproveAll { operator, expires } => {
                self.approve_all(deps, env, info, operator, expires)
            }
            ExecuteMsg::RevokeAll { operator } => self.revoke_all(deps, env, info, operator),
            ExecuteMsg::Transfer {
                recipient,
                token_id,
                amount,
                owner,
            } => self.transfer(deps, env, info, recipient, token_id, amount, owner),
            ExecuteMsg::Send {
                contract,
                token_id,
                amount,
                owner,
                msg,
            } => self.send(deps, env, info, contract, token_id, amount, owner, msg),
        }
    }
}

// TODO pull this into some sort of trait extension??
impl<'a, T, C> Cw1155Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    pub fn mint(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: MintMsg<T>,
    ) -> Result<Response<C>, ContractError> {
        let minter = self.minter.load(deps.storage)?;

        if info.sender != minter {
            return Err(ContractError::Unauthorized {});
        }

        // create the token
        let owner = deps.api.addr_validate(&msg.owner)?;
        let msg_copy = msg.clone();

        // Update the token info
        self.tokens
            .update(deps.storage, &msg.token_id, |old| -> StdResult<_> {
                match old {
                    Some(mut old_token) => {
                        old_token.supply += msg.amount;
                        Ok(old_token)
                    }
                    None => Ok(TokenInfo::<T> {
                        token_uri: msg_copy.token_uri,
                        supply: msg_copy.amount,
                        extension: msg_copy.extension,
                    }),
                }
            })?;

        // Update the owner info
        self.token_owned_info.update(
            deps.storage,
            (&msg.token_id, &owner),
            |old| -> StdResult<_> {
                match old {
                    Some(mut old_owned) => {
                        old_owned.balance += msg.amount;
                        Ok(old_owned)
                    }
                    None => Ok(OwnerInfo {
                        approvals: vec![],
                        balance: msg.amount,
                    }),
                }
            },
        )?;

        self.increment_total_tokens(deps.storage, msg.amount)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.sender)
            .add_attribute("token_id", msg.token_id)
            .add_attribute("amount", msg.amount))
    }
}

impl<'a, T, C> Cw1155Execute<T, C> for Cw1155Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    type Err = ContractError;

    fn transfer(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
        amount: Uint64,
        owner: Option<String>,
    ) -> Result<Response<C>, ContractError> {
        let owner = match owner {
            Some(owner) => deps.api.addr_validate(&owner)?,
            None => info.clone().sender,
        };

        let recipient = deps.api.addr_validate(&recipient)?;
        // Transfer token
        self._transfer_token_from(deps, &env, &info, &recipient, &token_id, &owner, amount)?;

        Ok(Response::new()
            .add_attribute("action", "transfer")
            .add_attribute("sender", info.sender)
            .add_attribute("recipient", recipient)
            .add_attribute("token_id", token_id))
    }

    fn send(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        amount: Uint64,
        owner: Option<String>,
        msg: Binary,
    ) -> Result<Response<C>, ContractError> {
        let owner = match owner {
            Some(owner) => deps.api.addr_validate(&owner)?,
            None => info.clone().sender,
        };
        let contract = deps.api.addr_validate(&contract)?;
        // Transfer token
        self._transfer_token_from(deps, &env, &info, &contract, &token_id, &owner, amount)?;

        let send = Cw1155ReceiveMsg {
            sender: info.sender.to_string(),
            token_id: token_id.clone(),
            amount,
            msg,
        };

        // Send message
        Ok(Response::new()
            .add_message(send.into_cosmos_msg(contract.clone())?)
            .add_attribute("action", "send")
            .add_attribute("sender", info.sender)
            .add_attribute("recipient", contract)
            .add_attribute("token_id", token_id))
    }

    fn increase_allowance(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        amount: Uint64,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        self._update_allowances(
            deps,
            &env,
            &info,
            &spender,
            &info.sender,
            &token_id,
            true,
            expires,
            amount,
        )?;

        Ok(Response::new()
            .add_attribute("action", "approve")
            .add_attribute("sender", info.sender)
            .add_attribute("spender", spender)
            .add_attribute("token_id", token_id))
    }

    fn decrease_allowance(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        amount: Uint64,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        self._update_allowances(
            deps,
            &env,
            &info,
            &spender,
            &info.sender,
            &token_id,
            false,
            expires,
            amount,
        )?;

        Ok(Response::new()
            .add_attribute("action", "revoke")
            .add_attribute("sender", info.sender)
            .add_attribute("spender", spender)
            .add_attribute("token_id", token_id))
    }

    fn approve_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        // reject expired data as invalid
        let expires = expires.unwrap_or_default();
        if expires.is_expired(&env.block) {
            return Err(ContractError::Expired {});
        }

        // set the operator for us
        let operator_addr = deps.api.addr_validate(&operator)?;
        self.operators
            .save(deps.storage, (&info.sender, &operator_addr), &expires)?;

        Ok(Response::new()
            .add_attribute("action", "approve_all")
            .add_attribute("sender", info.sender)
            .add_attribute("operator", operator))
    }

    fn revoke_all(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response<C>, ContractError> {
        let operator_addr = deps.api.addr_validate(&operator)?;
        self.operators
            .remove(deps.storage, (&info.sender, &operator_addr));

        Ok(Response::new()
            .add_attribute("action", "revoke_all")
            .add_attribute("sender", info.sender)
            .add_attribute("operator", operator))
    }
}

// helpers
impl<'a, T, C> Cw1155Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    pub fn _transfer_token_from(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        recipient: &Addr,
        token_id: &str,
        owner: &Addr,
        amount: Uint64,
    ) -> Result<(), ContractError> {
        // ensure we have permissions
        self.check_can_send(deps.as_ref(), env, info, token_id, owner, amount)?;
        // set owner and remove existing approvals

        // Update sender state
        let old_owner_info = self
            .token_owned_info
            .load(deps.storage, (token_id, owner))?;
        if old_owner_info.balance == amount {
            // Delete old owner
            self.token_owned_info
                .remove(deps.storage, (token_id, owner));
        } else {
            // Reduce old owners balance
            self.token_owned_info.update(
                deps.storage,
                (token_id, owner),
                |map_val| match map_val {
                    Some(mut owner_info) => {
                        owner_info.balance =
                            owner_info.balance.checked_sub(amount).or_else(|_| {
                                Err(ContractError::InvalidState {
                                    msg: "Owner balance is less than amount".into(),
                                })
                            })?;

                        if info.sender != *owner {
                            for val in owner_info.approvals.iter_mut() {
                                if val.spender == info.sender {
                                    val.allowance =
                                        val.allowance.checked_sub(amount).or_else(|_| {
                                            Err(ContractError::InvalidState {
                                                msg: "Spender allowance is less than amount".into(),
                                            })
                                        })?;
                                }
                            }
                        }
                        Ok(owner_info)
                    }
                    None => Err(ContractError::InvalidState {
                        msg: "Owner info does not exist for existing owner".into(),
                    }),
                },
            )?;
        }
        let storage = deps.storage;
        // Update the recipient state
        let owned_info = self.token_owned_info.load(storage, (token_id, recipient));
        match owned_info {
            Ok(mut owned_info) => {
                owned_info.balance = owned_info.balance.checked_add(amount).or_else(|_| {
                    Err(ContractError::InvalidState {
                        msg: "Recipient balance is greater than max".into(),
                    })
                })?;
                self.token_owned_info
                    .save(storage, (token_id, recipient), &owned_info)?;
            }
            Err(_) => {
                self.token_owned_info.save(
                    storage,
                    (token_id, recipient),
                    &OwnerInfo {
                        balance: amount,
                        approvals: vec![],
                    },
                )?;
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn _update_allowances(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        spender: &str,
        owner: &Addr,
        token_id: &str,
        // if add == false, remove. if add == true, remove then set with this expiration
        add: bool,
        expires: Option<Expiration>,
        amount: Uint64,
    ) -> Result<(), ContractError> {
        self.check_can_approve(deps.as_ref(), env, info, owner)?;

        // reject expired data as invalid
        if expires.unwrap_or_default().is_expired(&env.block) {
            return Err(ContractError::Expired {});
        }

        //update the approval list (remove any for the same spender before adding)
        let spender_addr = deps.api.addr_validate(spender)?;
        let mut owner_info = self
            .token_owned_info
            .load(deps.storage, (token_id, owner))?;
        let (mut spender_approval, mut other_approvals): (Vec<Approval>, Vec<Approval>) =
            owner_info
                .approvals
                .into_iter()
                .partition(|apr| apr.spender == spender_addr);

        if add {
            if spender_approval.len() > 0 {
                spender_approval[0].allowance += amount;
                if expires.is_some() {
                    spender_approval[0].expires = expires.unwrap();
                }
                other_approvals.push(spender_approval[0].clone());
            } else {
                let approval = Approval {
                    spender: spender_addr,
                    expires: expires.unwrap_or_default(),
                    allowance: amount,
                };
                other_approvals.push(approval);
            }
        } else {
            // Subtract the amount if the spender allowance exists and is more than
            // the amount to be removed. In other cases clear the allowance.
            if spender_approval.len() > 0 && spender_approval[0].allowance > amount {
                spender_approval[0].allowance =
                    spender_approval[0].allowance.checked_sub(amount).unwrap();
                if expires.is_some() {
                    spender_approval[0].expires = expires.unwrap();
                }
                other_approvals.push(spender_approval[0].clone());
            }
        }
        owner_info.approvals = other_approvals;
        self.token_owned_info
            .save(deps.storage, (token_id, owner), &owner_info)?;

        Ok(())
    }

    /// returns true iff the sender can execute approve or reject on the contract
    pub fn check_can_approve(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        owner: &Addr,
    ) -> Result<(), ContractError> {
        // owner can approve
        if *owner == info.sender {
            return Ok(());
        }
        // operator can approve
        let op = self
            .operators
            .may_load(deps.storage, (owner, &info.sender))?;
        match op {
            Some(ex) => {
                if ex.is_expired(&env.block) {
                    Err(ContractError::Unauthorized {})
                } else {
                    Ok(())
                }
            }
            None => Err(ContractError::Unauthorized {}),
        }
    }

    /// returns true iff the sender can transfer ownership of the token
    fn check_can_send(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        token_id: &str,
        owner: &Addr,
        amount: Uint64,
    ) -> Result<(), ContractError> {
        // owner can send
        let token_owned_info = match self.token_owned_info.load(deps.storage, (token_id, owner)) {
            Ok(info) => info,
            Err(_) => return Err(ContractError::Unauthorized {}),
        };
        if *owner == info.sender {
            if token_owned_info.balance >= amount {
                return Ok(());
            } else {
                return Err(ContractError::InsufficientBalance {
                    token_id: (*token_id).to_string(),
                    owner: owner.clone(),
                });
            }
        }

        // any non-expired token approval can send
        let token_owned_info = match self.token_owned_info.load(deps.storage, (token_id, owner)) {
            Ok(info) => info,
            Err(_) => return Err(ContractError::Unauthorized {}),
        };
        if token_owned_info.approvals.iter().any(|apr| {
            apr.spender == info.sender && !apr.is_expired(&env.block) && apr.allowance >= amount
        }) {
            return Ok(());
        }

        // operator can send
        let op = self
            .operators
            .may_load(deps.storage, (owner, &info.sender))?;
        match op {
            Some(ex) => {
                if ex.is_expired(&env.block) {
                    Err(ContractError::Unauthorized {})
                } else {
                    Ok(())
                }
            }
            None => Err(ContractError::Unauthorized {}),
        }
    }
}
