use cosmwasm_schema::cw_serde;
use std::fmt::{Display, Formatter};

use cosmwasm_std::{Binary, Env, Uint128};
use cw_utils::Expiration;

#[cw_serde]
pub struct Cw1155InstantiateMsg {
    /// Name of the token contract
    pub name: String,
    /// Symbol of the token contract
    pub symbol: String,

    /// The minter is the only one who can create new tokens.
    /// This is designed for a base token platform that is controlled by an external program or
    /// contract.
    /// If None, sender is the minter.
    pub minter: Option<String>,
}

/// This is like Cw1155ExecuteMsg but we add a Mint command for a minter
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[cw_serde]
pub enum Cw1155ExecuteMsg<T, E> {
    // cw1155
    /// BatchSendFrom is a base message to move multiple types of tokens in batch,
    /// if `env.sender` is the owner or has sufficient pre-approval.
    SendBatch {
        /// check approval if from is Some, otherwise assume sender is owner
        from: Option<String>,
        /// if `to` is not contract, `msg` should be `None`
        to: String,
        batch: Vec<TokenAmount>,
        /// `None` means don't call the receiver interface
        msg: Option<Binary>,
    },
    /// Mint a batch of tokens, can only be called by the contract minter
    MintBatch {
        recipient: String,
        msgs: Vec<Cw1155MintMsg<T>>,
    },
    /// BatchBurn is a base message to burn multiple types of tokens in batch.
    BurnBatch {
        /// check approval if from is Some, otherwise assume sender is owner
        from: Option<String>,
        batch: Vec<TokenAmount>,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },

    // cw721
    /// SendFrom is a base message to move tokens,
    /// if `env.sender` is the owner or has sufficient pre-approval.
    Send {
        /// check approval if from is Some, otherwise assume sender is owner
        from: Option<String>,
        /// If `to` is not contract, `msg` should be `None`
        to: String,
        token_id: String,
        amount: Uint128,
        /// `None` means don't call the receiver interface
        msg: Option<Binary>,
    },
    /// Mint a new NFT, can only be called by the contract minter
    Mint {
        recipient: String,
        msg: Cw1155MintMsg<T>,
    },
    /// Burn is a base message to burn tokens.
    Burn {
        /// check approval if from is Some, otherwise assume sender is owner
        from: Option<String>,
        token_id: String,
        amount: Uint128,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        /// Optional amount to approve. If None, approve entire balance.
        amount: Option<Uint128>,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke {
        spender: String,
        token_id: String,
        /// Optional amount to revoke. If None, revoke entire amount.
        amount: Option<Uint128>,
    },

    /// Extension msg
    Extension { msg: E },
}

#[cw_serde]
pub struct Cw1155MintMsg<T> {
    pub token_id: String,
    /// The amount of the newly minted tokens
    pub amount: Uint128,

    /// Only first mint can set `token_uri` and `extension`
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// Any custom extension used by this contract
    pub extension: T,
}

#[cw_serde]
#[derive(Eq)]
pub struct TokenAmount {
    pub token_id: String,
    pub amount: Uint128,
}

impl Display for TokenAmount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.token_id, self.amount)
    }
}

#[cw_serde]
pub struct TokenApproval {
    pub amount: Uint128,
    pub expiration: Expiration,
}

impl TokenApproval {
    pub fn is_expired(&self, env: &Env) -> bool {
        self.expiration.is_expired(&env.block)
    }
}
