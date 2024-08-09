use std::marker::PhantomData;

use crate::traits::{Cw721Calls, Cw721CustomMsg, Cw721State};
use crate::{
    DefaultOptionalCollectionExtension, DefaultOptionalCollectionExtensionMsg,
    DefaultOptionalNftExtension, DefaultOptionalNftExtensionMsg,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Empty};

/// Returns "empty" if the string is empty, otherwise the string itself
pub fn value_or_empty(value: &str) -> String {
    if value.is_empty() {
        "empty".to_string()
    } else {
        value.to_string()
    }
}

#[deprecated(
    since = "0.19.0",
    note = "Please use `DefaultCw721Helper`, `EmptyCw721Helper`, or `Cw721Helper` instead"
)]
pub type Cw721Contract = DefaultCw721Helper;

/// Opionated version of generic `Cw721Helper` with default extensions.
#[cw_serde]
pub struct DefaultCw721Helper(
    pub Addr,
    pub PhantomData<DefaultOptionalNftExtension>,
    pub PhantomData<DefaultOptionalNftExtensionMsg>,
    pub PhantomData<DefaultOptionalCollectionExtension>,
    pub PhantomData<DefaultOptionalCollectionExtensionMsg>,
    pub PhantomData<Empty>,
    pub PhantomData<Empty>,
);

impl DefaultCw721Helper {
    pub fn new(addr: Addr) -> Self {
        DefaultCw721Helper(
            addr,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

impl
    Cw721Calls<
        DefaultOptionalNftExtension,
        DefaultOptionalNftExtensionMsg,
        DefaultOptionalCollectionExtension,
        DefaultOptionalCollectionExtensionMsg,
        Empty,
        Empty,
    > for DefaultCw721Helper
{
    fn addr(&self) -> Addr {
        self.0.clone()
    }
}

/// Opionated version of generic `Cw721Helper` with empty extensions.
#[cw_serde]
pub struct EmptyCw721Helper(
    pub Addr,
    pub PhantomData<Empty>,
    pub PhantomData<Empty>,
    pub PhantomData<Empty>,
    pub PhantomData<Empty>,
    pub PhantomData<Empty>,
    pub PhantomData<Empty>,
);

impl EmptyCw721Helper {
    pub fn new(addr: Addr) -> Self {
        EmptyCw721Helper(
            addr,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

impl Cw721Calls<Empty, Empty, Empty, Empty, Empty, Empty> for EmptyCw721Helper {
    fn addr(&self) -> Addr {
        self.0.clone()
    }
}

/// `Cw721Helper` with generic extionsions. See `DefaultCw721Helper` and `EmptyCw721Helper` for
/// specific use cases.
#[cw_serde]
pub struct Cw721Helper<
    TNftExtension,
    TNftExtensionMsg,
    TCollectionExtension,
    TCollectionExtensionMsg,
    TExtensionMsg,
    TExtensionQueryMsg,
>(
    pub Addr,
    pub PhantomData<TNftExtension>,
    pub PhantomData<TNftExtensionMsg>,
    pub PhantomData<TCollectionExtension>,
    pub PhantomData<TCollectionExtensionMsg>,
    pub PhantomData<TExtensionMsg>,
    pub PhantomData<TExtensionQueryMsg>,
);

impl<
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
        TExtensionQueryMsg,
    >
    Cw721Calls<
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
        TExtensionQueryMsg,
    >
    for Cw721Helper<
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
        TExtensionQueryMsg,
    >
where
    TNftExtensionMsg: Cw721CustomMsg,
    TNftExtension: Cw721State,
    TCollectionExtension: Cw721State,
    TCollectionExtensionMsg: Cw721CustomMsg,
    TExtensionMsg: Cw721CustomMsg,
    TExtensionQueryMsg: Cw721CustomMsg,
{
    fn addr(&self) -> Addr {
        self.0.clone()
    }
}
