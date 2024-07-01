use cosmwasm_std::Empty;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::execute::*;
use cw721::traits::Cw721Execute;

use crate::state::Cw2981Contract;
use crate::{
    DefaultOptionMetadataExtensionWithRoyalty, DefaultOptionMetadataExtensionWithRoyaltyMsg,
};

impl
    Cw721Execute<
        DefaultOptionMetadataExtensionWithRoyalty,
        DefaultOptionMetadataExtensionWithRoyaltyMsg,
        Option<Empty>,
        Option<Empty>,
        Empty,
        Empty,
    > for Cw2981Contract<'static>
{
}
