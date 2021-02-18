use serde::Deserialize;

use crate::types::{EType, VType};

use super::shared;

#[derive(Deserialize, Clone, Debug)]
pub struct RVDMetadataV2 {
    #[serde(rename = "_key")]
    pub key: Vec<String>,
    #[serde(rename = "_codecSpec")]
    pub codec_spec: ComponentCodecSpecV2,
    // pub index_spec: Option<Something>,
    #[serde(rename = "_partFiles")]
    pub part_files: Vec<String>,
    // pub j_range_bounds: Vec<Something>,
    // pub attrs: Something,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "name")]
pub enum ComponentCodecSpecV2 {
    TypedCodecSpec(TypedCodecSpec),
}

#[derive(Deserialize, Clone, Debug)]
pub struct TypedCodecSpec {
    #[serde(rename = "_eType")]
    pub encoded_type: EType,
    #[serde(rename = "_vType")]
    pub virtual_type: VType,
    #[serde(rename = "_bufferSpec")]
    pub buffer_spec: shared::BufferSpec,
}
