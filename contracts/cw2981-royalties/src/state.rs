use std::marker::PhantomData;

use cosmwasm_std::Empty;
use cw721::{state::Cw721Config, state::NftInfo, traits::Contains};

use crate::{
    DefaultOptionMetadataExtensionWithRoyalty, DefaultOptionMetadataExtensionWithRoyaltyMsg,
    MetadataWithRoyalty,
};

#[deprecated(since = "0.19.0", note = "Please use `NftInfo`")]
pub type TokenInfo<TNftExtension> = NftInfo<TNftExtension>;

pub struct Cw2981Contract<'a> {
    pub config: Cw721Config<'a, DefaultOptionMetadataExtensionWithRoyalty>,
    pub(crate) _collection_extension: PhantomData<Empty>,
    pub(crate) _nft_extension_msg: PhantomData<DefaultOptionMetadataExtensionWithRoyaltyMsg>,
    pub(crate) _collection_extension_msg: PhantomData<Empty>,
    pub(crate) _extension_msg: PhantomData<Empty>,
    pub(crate) _extension_query_msg: PhantomData<Empty>,
    pub(crate) _custom_response_msg: PhantomData<Empty>,
}

impl Default for Cw2981Contract<'static> {
    fn default() -> Self {
        Self {
            config: Cw721Config::default(),
            _collection_extension: PhantomData,
            _nft_extension_msg: PhantomData,
            _collection_extension_msg: PhantomData,
            _extension_msg: PhantomData,
            _extension_query_msg: PhantomData,
            _custom_response_msg: PhantomData,
        }
    }
}

impl Contains for MetadataWithRoyalty {
    fn contains(&self, other: &MetadataWithRoyalty) -> bool {
        fn is_equal(a: &Option<String>, b: &Option<String>) -> bool {
            match (a, b) {
                (Some(a), Some(b)) => a.contains(b),
                (Some(_), None) => true,
                (None, None) => true,
                _ => false,
            }
        }
        if !is_equal(&self.image, &other.image) {
            return false;
        }
        if !is_equal(&self.image_data, &other.image_data) {
            return false;
        }
        if !is_equal(&self.external_url, &other.external_url) {
            return false;
        }
        if !is_equal(&self.description, &other.description) {
            return false;
        }
        if !is_equal(&self.name, &other.name) {
            return false;
        }
        if !is_equal(&self.background_color, &other.background_color) {
            return false;
        }
        if !is_equal(&self.animation_url, &other.animation_url) {
            return false;
        }
        if !is_equal(&self.youtube_url, &other.youtube_url) {
            return false;
        }
        if let (Some(a), Some(b)) = (&self.attributes, &other.attributes) {
            for (i, b) in b.iter().enumerate() {
                if !a[i].eq(b) {
                    return false;
                }
            }
        }
        true
    }
}
