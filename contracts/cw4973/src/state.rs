use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct MsgSignDataValue {
    pub signer: String,
    pub data: Vec<u8>,
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
    pub gas: String,
    pub amount: Vec<u8>,
}

// ADR 36 SignDoc based on cosmos sdk
#[cw_serde]
pub struct ADR36SignDoc {
    pub chain_id: String,
    pub account_number: String,
    pub sequence: String,
    pub fee: Fee,
    pub msgs: Vec<MsgSignData>,
    pub memo: String,
}

#[cw_serde]
pub struct PermitSignature {
    pub hrp: String,
    pub pub_key: Binary,
    pub signature: Binary,
}