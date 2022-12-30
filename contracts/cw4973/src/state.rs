use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct MsgSignDataValue {
    pub data: String,
    pub signer: String,
}

// MsgSignData based on ADR 036 of cosmos sdk
#[cw_serde]
pub struct MsgSignData {
    pub r#type: String,
    // value is a struct with signer and data
    pub value: MsgSignDataValue,
}

#[cw_serde]
pub struct Fee {
    pub amount: Vec<u8>,
    pub gas: String,
}

// TODO: the order of these fields is VERY IMPORTANT, DO NOT CHANGE IT
// ADR 36 SignDoc based on cosmos sdk
#[cw_serde]
pub struct ADR36SignDoc {
    pub account_number: String,
    pub chain_id: String,
    pub fee: Fee,
    pub memo: String,
    pub msgs: Vec<MsgSignData>,
    pub sequence: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PermitSignature {
    pub hrp: String,
    pub pub_key: String,
    pub signature: String,
}
