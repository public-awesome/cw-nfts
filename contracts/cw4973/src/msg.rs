use crate::state::PermitSignature;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Called by issuer to give a new ABT to receiver's address
    Give {
        to: String,
        uri: String,
        signature: PermitSignature,
    },

    /// Called by receiver to take a new ABT from issuer's address
    Take {
        from: String,
        uri: String,
        signature: PermitSignature,
    },

    /// Called by owner of an ABT to burn it
    Unequip { token_id: String },
}
