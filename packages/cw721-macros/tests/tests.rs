use cosmwasm_schema::cw_serde;
use cw721_macros::{cw721_execute, cw721_query};

#[cw721_execute]
#[cw_serde]
enum ExecuteMsg {
    Foo,
    Bar(u64),
    Baz { foo: u64 },
}

#[cw721_query]
#[cw_serde]
enum QueryMsg {
    Foo,
    Bar(u64),
    Baz { foo: u64 },
}

#[test]
fn deriving_execute_methods() {
    let msg = ExecuteMsg::Foo;

    // if this compiles we have won
    match msg {
        ExecuteMsg::TransferNft {
            recipient: _,
            token_id: _,
        }
        | ExecuteMsg::SendNft {
            contract: _,
            token_id: _,
            msg: _,
        }
        | ExecuteMsg::Approve {
            spender: _,
            token_id: _,
            expires: _,
        }
        | ExecuteMsg::ApproveAll {
            operator: _,
            expires: _,
        }
        | ExecuteMsg::Revoke {
            spender: _,
            token_id: _,
        }
        | ExecuteMsg::RevokeAll { operator: _ }
        | ExecuteMsg::Burn { token_id: _ }
        | ExecuteMsg::Foo
        | ExecuteMsg::Bar(_)
        | ExecuteMsg::Baz { .. } => "yay",
    };
}

#[test]
fn deriving_query_methods() {
    let msg = QueryMsg::Foo;

    // if this compiles we won
    match msg {
        QueryMsg::OwnerOf {
            token_id: _,
            include_expired: _,
        }
        | QueryMsg::Approval {
            token_id: _,
            spender: _,
            include_expired: _,
        }
        | QueryMsg::Approvals {
            token_id: _,
            include_expired: _,
        }
        | QueryMsg::AllOperators {
            owner: _,
            include_expired: _,
            start_after: _,
            limit: _,
        }
        | QueryMsg::NumTokens {}
        | QueryMsg::ContractInfo {}
        | QueryMsg::NftInfo { token_id: _ }
        | QueryMsg::AllNftInfo {
            token_id: _,
            include_expired: _,
        }
        | QueryMsg::Tokens {
            owner: _,
            start_after: _,
            limit: _,
        }
        | QueryMsg::AllTokens {
            start_after: _,
            limit: _,
        }
        | QueryMsg::Foo
        | QueryMsg::Bar(_)
        | QueryMsg::Baz { .. } => "yay",
    };
}
