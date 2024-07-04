use cosmwasm_std::Empty;
// expose to all others using contract, so others dont need to import cw721
pub use cw721_base::execute::*;
use cw721_base::traits::Cw721Execute;

use crate::state::Cw2981Contract;
use crate::{
    DefaultOptionMetadataExtensionWithRoyalty, DefaultOptionMetadataExtensionWithRoyaltyMsg,
};

impl
    Cw721Execute<
        DefaultOptionMetadataExtensionWithRoyalty,
        DefaultOptionMetadataExtensionWithRoyaltyMsg,
        Empty,
        Empty,
        Empty,
        Empty,
    > for Cw2981Contract<'static>
{
}
