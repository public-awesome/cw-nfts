use crate::msg::{ExecuteMsg, QueryMsg, TokenResponse};
use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, Empty, Querier, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw721MarketplaceContract(pub Addr);

impl Cw721MarketplaceContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call(&self, msg: ExecuteMsg, funds: Vec<Coin>) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg)?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds,
        }
        .into())
    }

    pub fn query<Q: Querier, T: DeserializeOwned>(
        &self,
        querier: &Q,
        req: QueryMsg,
    ) -> StdResult<T> {
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&req)?,
        }
        .into();
        QuerierWrapper::<Empty>::new(querier).query(&query)
    }

    /*** queries ***/
    pub fn token<Q: Querier, T: Into<String>>(
        &self,
        querier: &Q,
        token_id: T,
    ) -> StdResult<TokenResponse> {
        let req = QueryMsg::Token {
            id: token_id.into(),
        };
        self.query(querier, req)
    }
}
