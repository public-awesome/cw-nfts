use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::query::*;
use cw721::traits::{Contains, Cw721CustomMsg, Cw721Query, Cw721State, FromAttributesState};

use crate::Cw721Contract;

impl<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    > Cw721Query<TNftExtension, TCollectionExtension, TExtensionQueryMsg>
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
    TNftExtension: Cw721State + Contains,
    TNftExtensionMsg: Cw721CustomMsg,
    TCollectionExtension: Cw721State + FromAttributesState,
    TCollectionExtensionMsg: Cw721CustomMsg,
    TExtensionQueryMsg: Cw721CustomMsg,
    TCustomResponseMsg: CustomMsg,
{
}
