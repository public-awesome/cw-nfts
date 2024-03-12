use std::{env::current_dir, fs::create_dir_all};

use cosmwasm_schema::{export_schema_with_title, remove_schemas, schema_for};
use cosmwasm_std::Empty;
use cw721::state::{DefaultOptionCollectionMetadataExtension, DefaultOptionNftMetadataExtension};

use cw721_expiration::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    // entry points - generate always with title for avoiding name suffixes like "..._empty_for_..." due to generics
    export_schema_with_title(
        &schema_for!(InstantiateMsg<DefaultOptionCollectionMetadataExtension>),
        &out_dir,
        "InstantiateMsg",
    );
    export_schema_with_title(
        &schema_for!(
            ExecuteMsg::<
                DefaultOptionNftMetadataExtension,
                Empty,
                DefaultOptionCollectionMetadataExtension,
            >
        ),
        &out_dir,
        "ExecuteMsg",
    );
    export_schema_with_title(
        &schema_for!(QueryMsg<DefaultOptionNftMetadataExtension, DefaultOptionCollectionMetadataExtension>),
        &out_dir,
        "QueryMsg",
    );
    export_schema_with_title(&schema_for!(MigrateMsg), &out_dir, "MigrateMsg");
}
