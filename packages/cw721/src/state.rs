use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Timestamp};

#[cw_serde]
pub struct CollectionInfo<TCollectionInfoExtension> {
    pub name: String,
    pub symbol: String,
    pub extension: TCollectionInfoExtension,
    pub updated_at: Timestamp,
}

#[cw_serde]
pub struct CollectionInfoExtension<TRoyaltyInfo> {
    pub description: Option<String>,
    pub image: Option<String>,
    pub external_link: Option<String>,
    pub explicit_content: Option<bool>,
    pub royalty_info: Option<TRoyaltyInfo>,
}

#[cw_serde]
pub struct RoyaltyInfo {
    pub payment_address: Addr,
    pub share: Decimal,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

pub type MetadataExtension = Option<Metadata>;
