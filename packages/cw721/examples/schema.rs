use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};

use cosmwasm_std::Empty;
use cw721::{
    msg::{
        AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, Cw721ExecuteMsg,
        Cw721InstantiateMsg, Cw721MigrateMsg, Cw721QueryMsg, MinterResponse, NftInfoResponse,
        NumTokensResponse, OperatorResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
    },
    receiver::Cw721ReceiveMsg,
    state::{
        CollectionMetadata, DefaultOptionCollectionMetadataExtension,
        DefaultOptionNftMetadataExtension,
    },
};
fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    // entry points - generate always with title for avoiding name suffixes like "..._empty_for_..." due to generics
    export_schema_with_title(
        &schema_for!(Cw721InstantiateMsg<DefaultOptionCollectionMetadataExtension>),
        &out_dir,
        "Cw721InstantiateMsg",
    );
    export_schema_with_title(
        &schema_for!(
            Cw721ExecuteMsg::<
                DefaultOptionNftMetadataExtension,
                Empty,
                DefaultOptionCollectionMetadataExtension,
            >
        ),
        &out_dir,
        "Cw721ExecuteMsg",
    );
    export_schema_with_title(
        &schema_for!(Cw721QueryMsg<Empty, DefaultOptionCollectionMetadataExtension>),
        &out_dir,
        "Cw721QueryMsg",
    );
    export_schema_with_title(&schema_for!(Cw721MigrateMsg), &out_dir, "Cw721MigrateMsg");

    // messages
    export_schema_with_title(&schema_for!(Cw721ReceiveMsg), &out_dir, "Cw721ReceiveMsg");

    // responses
    export_schema_with_title(
        &schema_for!(NftInfoResponse<DefaultOptionNftMetadataExtension>),
        &out_dir,
        "NftInfoResponse",
    );
    export_schema_with_title(
        &schema_for!(AllNftInfoResponse<DefaultOptionNftMetadataExtension>),
        &out_dir,
        "AllNftInfoResponse",
    );
    export_schema(&schema_for!(ApprovalResponse), &out_dir);
    export_schema(&schema_for!(ApprovalsResponse), &out_dir);
    export_schema(&schema_for!(OperatorResponse), &out_dir);
    export_schema(&schema_for!(OperatorsResponse), &out_dir);
    export_schema_with_title(
        &schema_for!(CollectionMetadata<DefaultOptionCollectionMetadataExtension>),
        &out_dir,
        "CollectionMetadata",
    );
    export_schema(&schema_for!(OwnerOfResponse), &out_dir);
    export_schema(&schema_for!(MinterResponse), &out_dir);
    export_schema(&schema_for!(NumTokensResponse), &out_dir);
    export_schema(&schema_for!(TokensResponse), &out_dir);
}
