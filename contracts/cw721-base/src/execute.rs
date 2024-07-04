use cosmwasm_std::{CustomMsg, Empty};

// expose so other libs dont need to import cw721
pub use cw721::execute::*;

use cw721::traits::{Cw721CustomMsg, Cw721Execute, Cw721State};
use cw721::traits::{FromAttributesState, StateFactory, ToAttributesState};
use cw721::{
    DefaultOptionalCollectionExtension, DefaultOptionalCollectionExtensionMsg,
    DefaultOptionalNftExtension, DefaultOptionalNftExtensionMsg, EmptyOptionalCollectionExtension,
    EmptyOptionalCollectionExtensionMsg, EmptyOptionalNftExtension, EmptyOptionalNftExtensionMsg,
};

use crate::{Cw721Contract, DefaultOptionalCw721Contract, EmptyOptionalCw721Contract};

impl<'a>
    Cw721Execute<
        DefaultOptionalNftExtension,
        DefaultOptionalNftExtensionMsg,
        DefaultOptionalCollectionExtension,
        DefaultOptionalCollectionExtensionMsg,
        Empty,
        Empty,
    > for DefaultOptionalCw721Contract<'a>
{
}

impl<'a>
    Cw721Execute<
        EmptyOptionalNftExtension,
        EmptyOptionalNftExtensionMsg,
        EmptyOptionalCollectionExtension,
        EmptyOptionalCollectionExtensionMsg,
        Empty,
        Empty,
    > for EmptyOptionalCw721Contract<'a>
{
}

impl<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    >
    Cw721Execute<
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
        TCustomResponseMsg,
    >
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
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg + StateFactory<TNftExtension>,
    TCollectionExtension: Cw721State + ToAttributesState + FromAttributesState,
    TCollectionExtensionMsg: Cw721CustomMsg + StateFactory<TCollectionExtension>,
    TCustomResponseMsg: CustomMsg,
{
}
