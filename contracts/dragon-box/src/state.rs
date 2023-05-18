use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionInfo {
    pub name: String,
    pub symbol: String,
    pub minter: String,
}

pub const COLLECTION_INFO: Item<CollectionInfo> = Item::new("collection_info");
