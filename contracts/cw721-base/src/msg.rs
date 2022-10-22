use cosmwasm_schema::cw_serde;
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg};

#[cw_serde]
pub struct InstantiateMsg {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,

    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: String,
}

/// This is like Cw721ExecuteMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[cw_serde]
#[serde(untagged)]
pub enum ExecuteMsg<T> {
    Parent(Cw721ExecuteMsg),

    /// Mint a new NFT, can only be called by the contract minter
    Mint {
        token_id: String,
        /// The owner of the newly minter NFT
        owner: String,
        /// Universal resource identifier for this NFT
        /// Should point to a JSON file that conforms to the ERC721
        /// Metadata JSON Schema
        token_uri: Option<String>,
        /// Any custom extension used by this contract
        extension: T,
    },
}

#[cw_serde]
#[serde(untagged)]
pub enum QueryMsg {
    Parent(Cw721QueryMsg),

    // Return the minter
    Minter {},
}

/// Shows who can mint these tokens
#[cw_serde]
pub struct MinterResponse {
    pub minter: String,
}
