use crate::Cw1155RoyaltiesConfig;
use cosmwasm_std::{Decimal, Deps, StdResult, Uint128};
use cw2981_royalties::msg::RoyaltiesInfoResponse;

/// NOTE: default behaviour here is to round down
/// EIP2981 specifies that the rounding behaviour is at the discretion of the implementer
/// NOTE: This implementation is copied from the cw2981-royalties contract, only difference is the TokenInfo struct (no owner field in cw1155)
pub fn query_royalties_info(
    deps: Deps,
    token_id: String,
    sale_price: Uint128,
) -> StdResult<RoyaltiesInfoResponse> {
    let config = Cw1155RoyaltiesConfig::default();
    let token_info = config.tokens.load(deps.storage, &token_id)?;

    let royalty_percentage = match token_info.extension {
        Some(ref ext) => match ext.royalty_percentage {
            Some(percentage) => Decimal::percent(percentage),
            None => Decimal::percent(0),
        },
        None => Decimal::percent(0),
    };
    let royalty_from_sale_price = sale_price * royalty_percentage;

    let royalty_address = match token_info.extension {
        Some(ext) => match ext.royalty_payment_address {
            Some(addr) => addr,
            None => String::from(""),
        },
        None => String::from(""),
    };

    Ok(RoyaltiesInfoResponse {
        address: royalty_address,
        royalty_amount: royalty_from_sale_price,
    })
}
