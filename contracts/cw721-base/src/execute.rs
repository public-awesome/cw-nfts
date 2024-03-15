use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::execute::*;
use cw721::traits::StateFactory;
use cw721::traits::{Cw721CustomMsg, Cw721State};

use crate::Cw721Contract;

impl<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
    Cw721Execute<
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
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
    TNftMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TNftMetadataExtension>,
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TCollectionMetadataExtension>,
    TCustomResponseMsg: CustomMsg,
{
}
