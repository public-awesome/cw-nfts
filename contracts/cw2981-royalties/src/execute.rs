use cosmwasm_std::Empty;
use cw721::traits::Cw721Execute;

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
