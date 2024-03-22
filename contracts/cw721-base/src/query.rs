use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::query::*;
use cw721::traits::{Cw721CustomMsg, Cw721State, FromAttributes};

use crate::Cw721Contract;

impl<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    > Cw721Query<TNftMetadataExtension, TCollectionMetadataExtension>
    for Cw721Contract<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtension: Cw721State + FromAttributes,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
    TCustomResponseMsg: CustomMsg,
{
}
