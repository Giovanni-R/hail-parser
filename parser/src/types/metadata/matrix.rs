use serde::Deserialize;

use crate::types::VType;

use super::shared;

#[derive(Deserialize, Clone, Debug)]
pub struct MatrixMetadata {
    pub file_version: u32,
    pub hail_version: String,
    pub references_rel_path: String,
    pub matrix_type: MatrixSchema,
    pub components: MatrixComponents,
}

#[derive(Clone, Debug)]
pub struct MatrixSchema {
    pub global_schema: VType,
    pub col_keys: Vec<(String, VType)>,
    pub col_schema: VType,
    pub row_keys: Vec<(String, VType)>,
    pub row_schema: VType,
    pub entry_schema: VType,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MatrixComponents {
    pub entries: shared::ComponentReference,
    pub globals: shared::ComponentReference,
    pub rows: shared::ComponentReference,
    pub cols: shared::ComponentReference,
    pub partition_counts: shared::PartitionCounts,
}
