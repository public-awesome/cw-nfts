use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DataEnum, DeriveInput, Variant};

/// Adds the necessary execute messages defined by the CW-721 specs to an enum. For example,
///
/// ```rust
/// use cw721_macros::cw721_execute;
///
/// #[cw721_execute]
/// enum ExecuteMsg {
///     Foo,
///     Bar,
/// }
/// ```
///
/// Will transform the enum to:
///
/// ```rust
/// enum ExecuteMsg {
///     TransferNft {
///         recipient: String,
///         token_id: String,
///     },
///     SendNft {
///         contract: String,
///         token_id: String,
///         msg: cosmwasm_std::Binary,
///     },
///     // and other execute methods required by CW-721 specs
///     // ...
///     Foo,
///     Bar,
/// }
/// ```
///
/// Note that other derive macro invocations must occur after this procedural macro as they may
/// depend on the new fields. For example, the following will fail because `#[cw_serde]` derivation
/// occurs before the addition of the fields:
///
/// ```compile_fail
/// use cosmwasm_schema::cw_serde;
/// use cw721_macros::cw721_execute;
///
/// #[cw_serde]
/// #[cw721_execute]
/// enum ExecuteMsg {}
/// ```
///
/// The implementation of this macro this heavily inspired by
/// [DAO DAO](https://github.com/DA0-DA0/dao-contracts/tree/main/packages/cwd-macros).
/// ```
#[proc_macro_attribute]
pub fn cw721_execute(metadata: TokenStream, input: TokenStream) -> TokenStream {
    // make sure no arguments were passed in
    let args = parse_macro_input!(metadata as AttributeArgs);
    if let Some(first_arg) = args.first() {
        return syn::Error::new_spanned(first_arg, "macro `cw721_execute` takes no arguments")
            .to_compile_error()
            .into();
    }

    let mut ast: DeriveInput = parse_macro_input!(input);
    match &mut ast.data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let transfer_nft: Variant = syn::parse2(quote! {
                /// Transfer is a base message to move a token to another account without
                /// triggering actions.
                TransferNft {
                    recipient: String,
                    token_id: String,
                }
            })
            .unwrap();

            let send_nft: Variant = syn::parse2(quote! {
                /// Send is a base message to transfer a token to a contract and trigger an action
                /// on the receiving contract.
                SendNft {
                    contract: String,
                    token_id: String,
                    msg: ::cosmwasm_std::Binary,
                }
            })
            .unwrap();

            let approve: Variant = syn::parse2(quote! {
                /// Allows operator to transfer / send the token from the owner's account.
                /// If expiration is set, then this allowance has a time/height limit.
                Approve {
                    spender: String,
                    token_id: String,
                    expires: Option<::cw_utils::Expiration>,
                }
            })
            .unwrap();

            let approve_all: Variant = syn::parse2(quote! {
                /// Allows operator to transfer / send any token from the owner's account.
                /// If expiration is set, then this allowance has a time/height limit.
                ApproveAll {
                    operator: String,
                    expires: Option<::cw_utils::Expiration>,
                }
            })
            .unwrap();

            let revoke: Variant = syn::parse2(quote! {
                /// Remove previously granted Approval
                Revoke {
                    spender: String,
                    token_id: String,
                }
            })
            .unwrap();

            let revoke_all: Variant = syn::parse2(quote! {
                /// Remove previously granted ApproveAll permission
                RevokeAll {
                    operator: String,
                }
            })
            .unwrap();

            let burn: Variant = syn::parse2(quote! {
                /// Burn an NFT the sender has access to
                Burn {
                    token_id: String,
                }
            })
            .unwrap();

            variants.push(transfer_nft);
            variants.push(send_nft);
            variants.push(approve);
            variants.push(approve_all);
            variants.push(revoke);
            variants.push(revoke_all);
            variants.push(burn);
        }
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                "macro `cw721_execute` only applies to enums",
            )
            .to_compile_error()
            .into()
        }
    }

    quote! { #ast }.into()
}

/// Adds the necessary query messages defined by the CW-721 specs to an enum. For example,
///
/// ```rust
/// use cw721_macros::cw721_query;
///
/// #[cw721_query]
/// enum QueryMsg {
///     Foo,
///     Bar,
/// }
/// ```
///
/// Will transform the enum to:
///
/// ```rust
/// enum QueryMsg {
///     OwnerOf {
///         token_id: String,
///         include_expired: Option<bool>,
///     },
///     Approval {
///         token_id: String,
///         spender: String,
///         include_expired: Option<bool>,
///     },
///     // and other query methods required by CW-721 specs
///     // ...
///     Foo,
///     Bar,
/// }
/// ```
///
/// Note that other derive macro invocations must occur after this procedural macro as they may
/// depend on the new fields. For example, the following will fail because `#[cw_serde]` derivation
/// occurs before the addition of the fields:
///
/// ```compile_fail
/// use cosmwasm_schema::cw_serde;
/// use cw721_macros::cw721_query;
///
/// #[cw_serde]
/// #[cw721_query]
/// enum QueryMsg {}
/// ```
///
/// The implementation of this macro this heavily inspired by
/// [DAO DAO](https://github.com/DA0-DA0/dao-contracts/tree/main/packages/cwd-macros).
/// ```
#[proc_macro_attribute]
pub fn cw721_query(metadata: TokenStream, input: TokenStream) -> TokenStream {
    // make sure no arguments were passed in
    let args = parse_macro_input!(metadata as AttributeArgs);
    if let Some(first_arg) = args.first() {
        return syn::Error::new_spanned(first_arg, "macro `cw721_query` takes no arguments")
            .to_compile_error()
            .into();
    }

    let mut ast: DeriveInput = parse_macro_input!(input);
    match &mut ast.data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let owner_of: Variant = syn::parse2(quote! {
                /// Return the owner of the given token, error if token does not exist
                /// Return type: OwnerOfResponse
                OwnerOf {
                    token_id: String,
                    include_expired: Option<bool>,
                }
            })
            .unwrap();

            let approval: Variant = syn::parse2(quote! {
                /// Return operator that can access all of the owner's tokens.
                /// Return type: `ApprovalResponse`
                Approval {
                    token_id: String,
                    spender: String,
                    include_expired: Option<bool>,
                }
            })
            .unwrap();

            let approvals: Variant = syn::parse2(quote! {
                /// Return approvals that a token has
                /// Return type: `ApprovalsResponse`
                Approvals {
                    token_id: String,
                    include_expired: Option<bool>,
                }
            })
            .unwrap();

            let all_operators: Variant = syn::parse2(quote! {
                /// List all operators that can access all of the owner's tokens
                /// Return type: `OperatorsResponse`
                AllOperators {
                    owner: String,
                    include_expired: Option<bool>,
                    start_after: Option<String>,
                    limit: Option<u32>,
                }
            })
            .unwrap();

            let num_tokens: Variant = syn::parse2(quote! {
                /// Total number of tokens issued
                NumTokens {}
            })
            .unwrap();

            let contract_info: Variant = syn::parse2(quote! {
                /// With MetaData Extension.
                /// Returns top-level metadata about the contract: `ContractInfoResponse`
                ContractInfo {}
            })
            .unwrap();

            let nft_info: Variant = syn::parse2(quote! {
                /// With MetaData Extension.
                /// Returns metadata about one particular token, based on *ERC721 Metadata JSON
                /// Schema* but directly from the contract: `NftInfoResponse`
                NftInfo {
                    token_id: String,
                }
            })
            .unwrap();

            let all_nft_info: Variant = syn::parse2(quote! {
                /// With MetaData Extension.
                /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an
                /// optimization for clients: `AllNftInfo`
                AllNftInfo {
                    token_id: String,
                    include_expired: Option<bool>,
                }
            })
            .unwrap();

            let tokens: Variant = syn::parse2(quote! {
                /// With Enumerable extension.
                /// Returns all tokens owned by the given address, [] if unset.
                /// Return type: TokensResponse.
                Tokens {
                    owner: String,
                    start_after: Option<String>,
                    limit: Option<u32>,
                }
            })
            .unwrap();

            let all_tokens: Variant = syn::parse2(quote! {
                /// With Enumerable extension.
                /// Requires pagination. Lists all token_ids controlled by the contract.
                /// Return type: TokensResponse.
                AllTokens {
                    start_after: Option<String>,
                    limit: Option<u32>,
                }
            })
            .unwrap();

            variants.push(owner_of);
            variants.push(approval);
            variants.push(approvals);
            variants.push(all_operators);
            variants.push(num_tokens);
            variants.push(contract_info);
            variants.push(nft_info);
            variants.push(all_nft_info);
            variants.push(tokens);
            variants.push(all_tokens);
        }
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                "macro `cw721_query` only applies to enums",
            )
            .to_compile_error()
            .into()
        }
    }

    quote! { #ast }.into()
}
