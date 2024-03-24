use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::execute::*;
use cw721::traits::{Cw721CustomMsg, Cw721Execute, Cw721State};
use cw721::traits::{FromAttributesState, StateFactory, ToAttributesState};

use crate::Cw721Contract;

impl<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    >
    Cw721Execute<
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TCustomResponseMsg,
    >
    for Cw721Contract<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    >
where
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg + StateFactory<TNftExtension>,
    TCollectionExtension: Cw721State + ToAttributesState + FromAttributesState,
    TCollectionExtensionMsg: Cw721CustomMsg + StateFactory<TCollectionExtension>,
    TCustomResponseMsg: CustomMsg,
{
}
