use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct CollectionInfo {
    pub name: String,
    pub symbol: String,
}
