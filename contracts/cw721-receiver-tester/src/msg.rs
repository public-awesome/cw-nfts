use cosmwasm_schema::{cw_serde, QueryResponses};
use cw721::receiver::Cw721ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
}

#[cw_serde]
pub enum InnerMsg {
    Succeed,
    Fail,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, to_json_binary};

    use super::*;

    #[test]
    fn inner_msg_json() {
        let json = to_json_binary(&InnerMsg::Succeed).unwrap();
        assert_eq!(json, br#""succeed""#);
        let msg: InnerMsg = from_json(&json).unwrap();
        assert_eq!(msg, InnerMsg::Succeed);
    }
}
