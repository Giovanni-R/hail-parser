use crate::types::{EType, VType};

#[derive(Clone, Debug)]
pub struct ComponentMetadata {
    pub key: Vec<String>,
    pub virtual_type: VType,
    pub encoded_type: EType,
    pub codec_spec: CodecSpec,
    pub buffer_spec: super::shared::BufferSpec,
    pub part_files: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum CodecSpec {
    TypedCodecSpec,
    PackCodecSpec,
}
