use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};

use cosmwasm_std::Empty;
use cw721::{
    msg::{
        AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, Cw721ExecuteMsg,
        Cw721InstantiateMsg, Cw721QueryMsg, NftInfoResponse, NumTokensResponse, OperatorResponse,
        OperatorsResponse, OwnerOfResponse, TokensResponse,
    },
    receiver::Cw721ReceiveMsg,
    state::{CollectionInfo, DefaultOptionCollectionInfoExtension, DefaultOptionMetadataExtension},
};
fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(
        &schema_for!(Cw721InstantiateMsg<DefaultOptionCollectionInfoExtension>),
        &out_dir,
        "InstantiateMsg",
    );
    export_schema_with_title(
        &schema_for!(
            Cw721ExecuteMsg::<
                DefaultOptionMetadataExtension,
                Empty,
                DefaultOptionCollectionInfoExtension,
            >
        ),
        &out_dir,
        "ExecuteMsg",
    );
    export_schema_with_title(
        &schema_for!(Cw721QueryMsg<Empty, DefaultOptionCollectionInfoExtension>),
        &out_dir,
        "QueryMsg",
    );
    export_schema(&schema_for!(Cw721ReceiveMsg), &out_dir);
    export_schema_with_title(
        &schema_for!(NftInfoResponse<DefaultOptionMetadataExtension>),
        &out_dir,
        "NftInfoResponse",
    );
    export_schema_with_title(
        &schema_for!(AllNftInfoResponse<DefaultOptionMetadataExtension>),
        &out_dir,
        "AllNftInfoResponse",
    );
    export_schema(&schema_for!(ApprovalResponse), &out_dir);
    export_schema(&schema_for!(ApprovalsResponse), &out_dir);
    export_schema(&schema_for!(OperatorResponse), &out_dir);
    export_schema(&schema_for!(OperatorsResponse), &out_dir);
    export_schema_with_title(
        &schema_for!(CollectionInfo<DefaultOptionCollectionInfoExtension>),
        &out_dir,
        "CollectionInfo",
    );
    export_schema(&schema_for!(OwnerOfResponse), &out_dir);
    export_schema(&schema_for!(NumTokensResponse), &out_dir);
    export_schema(&schema_for!(TokensResponse), &out_dir);
}
