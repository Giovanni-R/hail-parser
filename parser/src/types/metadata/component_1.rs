use serde::Deserialize;

use crate::types::VType;

use super::shared;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RvdMetadataV1 {
    #[serde(alias = "orvdType")]
    pub rvd_type: RvdTypeSchema,
    pub codec_spec: ComponentCodecSpec,
    pub index_spec: Option<IndexSpec>,
    pub part_files: Vec<String>,
    pub j_range_bounds: Vec<JRangeBound>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnpartitionedRvdMetadataV1 {
    pub row_type: VType,
    pub codec_spec: ComponentCodecSpec,
    pub part_files: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct RvdTypeSchema {
    pub row_schema: VType,
    pub row_keys: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "name")]
pub enum ComponentCodecSpec {
    PackCodecSpec { child: shared::BufferSpec },
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexSpec {
    pub rel_path: String,
    pub key_type: String,
    pub annotation_type: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JRangeBound {
    // start: HailValue,
    // end: HailValue,
    include_start: bool,
    include_end: bool,
}
