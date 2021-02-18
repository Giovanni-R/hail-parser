use serde::Deserialize;

use crate::types::VType;

use super::shared;

#[derive(Deserialize, Clone, Debug)]
pub struct TableMetadata {
    pub file_version: u32,
    pub hail_version: String,
    pub references_rel_path: String,
    pub table_type: TableSchema,
    pub components: TableComponents,
}

#[derive(Clone, Debug)]
pub struct TableSchema {
    pub global_schema: VType,
    pub row_schema: VType,
    pub row_keys: Vec<(String, VType)>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TableComponents {
    pub globals: shared::ComponentReference,
    pub rows: shared::ComponentReference,
    pub partition_counts: shared::PartitionCounts,
}
