use cosmwasm_std::{CustomMsg, Empty};

// expose so other libs dont need to import cw721
pub use cw721::query::*;

use cw721::{
    traits::{Contains, Cw721CustomMsg, Cw721Query, Cw721State, FromAttributesState},
    DefaultOptionalCollectionExtension, DefaultOptionalNftExtension,
};

use crate::{Cw721Contract, DefaultCw721Contract, EmptyCw721Contract};

impl<'a> Cw721Query<DefaultOptionalNftExtension, DefaultOptionalCollectionExtension, Empty>
    for DefaultCw721Contract<'a>
{
}

impl<'a> Cw721Query<Empty, Empty, Empty> for EmptyCw721Contract<'a> {}

impl<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    > Cw721Query<TNftExtension, TCollectionExtension, TExtensionQueryMsg>
    for Cw721Contract<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
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
