use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::query::*;
use cw721::traits::{Cw721CustomMsg, Cw721Query, Cw721State, FromAttributesState};

use crate::Cw721Contract;

impl<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TCustomResponseMsg,
    > Cw721Query<TNftExtension, TCollectionExtension>
    for Cw721Contract<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg,
    TCollectionExtension: Cw721State + FromAttributesState,
    TCollectionExtensionMsg: Cw721CustomMsg,
    TCustomResponseMsg: CustomMsg,
{
}
