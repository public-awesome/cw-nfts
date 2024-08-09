use cw721::state::NftInfo;

#[deprecated(since = "0.19.0", note = "Please use `NftInfo`")]
pub type TokenInfo<TNftExtension> = NftInfo<TNftExtension>;
