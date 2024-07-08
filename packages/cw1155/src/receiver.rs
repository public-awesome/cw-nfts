use crate::msg::TokenAmount;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Binary, CosmosMsg, MessageInfo, StdResult, Uint128, WasmMsg};
use schemars::JsonSchema;

/// Cw1155ReceiveMsg should be de/serialized under `Receive()` variant in a ExecuteMsg
#[cw_serde]
pub struct Cw1155ReceiveMsg {
    /// The account that executed the send message
    pub operator: String,
    /// The account that the token transfered from
    pub from: Option<String>,
    pub token_id: String,
    pub amount: Uint128,
    pub msg: Binary,
}

impl Cw1155ReceiveMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::Receive(self);
        to_json_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<T: Into<String>, C>(
        self,
        info: &MessageInfo,
        contract_addr: T,
    ) -> StdResult<CosmosMsg<C>>
    where
        C: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: info.funds.to_vec(),
        };
        Ok(execute.into())
    }
}

/// Cw1155BatchReceiveMsg should be de/serialized under `BatchReceive()` variant in a ExecuteMsg
#[cw_serde]
pub struct Cw1155BatchReceiveMsg {
    pub operator: String,
    pub from: Option<String>,
    pub batch: Vec<TokenAmount>,
    pub msg: Binary,
}

impl Cw1155BatchReceiveMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::BatchReceive(self);
        to_json_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<T: Into<String>, C>(
        self,
        info: &MessageInfo,
        contract_addr: T,
    ) -> StdResult<CosmosMsg<C>>
    where
        C: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: info.funds.to_vec(),
        };
        Ok(execute.into())
    }
}

// This is just a helper to properly serialize the above message
#[cw_serde]
enum ReceiverExecuteMsg {
    Receive(Cw1155ReceiveMsg),
    BatchReceive(Cw1155BatchReceiveMsg),
}
