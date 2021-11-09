use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, Uint64};
use cw0::Expiration;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Cw1155ExecuteMsg {
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft {
        recipient: String,
        token_id: String,
        amount: Uint64,
    },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        amount: Uint64,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    IncreaseAllowance {
        spender: String,
        /// If owner is not specified its assumed that the sender is the owner
        owner: Option<String>,
        token_id: String,
        expires: Option<Expiration>,
        amount: Uint64,
    },
    /// Remove previously granted Approval
    DecreaseAllowance {
        spender: String,
        /// If owner is not specified its assumed that the sender is the owner
        owner: Option<String>,
        token_id: String,
        amount: Uint64,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },
}
