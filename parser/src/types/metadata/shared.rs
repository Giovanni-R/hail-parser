use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "name")]
pub enum BufferSpec {
    LEB128BufferSpec {
        child: Box<BufferSpec>,
    },
    BlockingBufferSpec {
        #[serde(rename = "blockSize")]
        block_size: u32,
        child: Box<BufferSpec>,
    },
    LZ4BlockBufferSpec {
        #[serde(rename = "blockSize")]
        block_size: u32,
        child: Box<BufferSpec>,
    },
    LZ4HCBlockBufferSpec {
        #[serde(rename = "blockSize")]
        block_size: u32,
        child: Box<BufferSpec>,
    },
    LZ4FastBlockBufferSpec {
        #[serde(rename = "blockSize")]
        block_size: u32,
        child: Box<BufferSpec>,
    },
    StreamBlockBufferSpec,
    StreamBufferSpec,
}

impl BufferSpec {
    pub fn uses_compression(&self) -> bool {
        match self {
            BufferSpec::LEB128BufferSpec { child } => child.uses_compression(),
            BufferSpec::BlockingBufferSpec { child, .. } => child.uses_compression(),
            BufferSpec::LZ4BlockBufferSpec { .. } => true,
            BufferSpec::LZ4HCBlockBufferSpec { .. } => true,
            BufferSpec::LZ4FastBlockBufferSpec { .. } => true,
            BufferSpec::StreamBlockBufferSpec => false,
            BufferSpec::StreamBufferSpec => false,
        }
    }

    pub fn uses_leb128(&self) -> bool {
        match self {
            BufferSpec::LEB128BufferSpec { .. } => true,
            BufferSpec::BlockingBufferSpec { child, .. } => child.uses_leb128(),
            BufferSpec::LZ4BlockBufferSpec { child, .. } => child.uses_leb128(),
            BufferSpec::LZ4HCBlockBufferSpec { child, .. } => child.uses_leb128(),
            BufferSpec::LZ4FastBlockBufferSpec { child, .. } => child.uses_leb128(),
            BufferSpec::StreamBlockBufferSpec => false,
            BufferSpec::StreamBufferSpec => false,
        }
    }

    pub fn appends_length(&self) -> bool {
        match self {
            BufferSpec::LEB128BufferSpec { child, .. } => child.appends_length(),
            BufferSpec::BlockingBufferSpec { child, .. } => child.appends_length(),
            BufferSpec::LZ4BlockBufferSpec { child, .. } => child.appends_length(),
            BufferSpec::LZ4HCBlockBufferSpec { child, .. } => child.appends_length(),
            BufferSpec::LZ4FastBlockBufferSpec { child, .. } => child.appends_length(),
            BufferSpec::StreamBlockBufferSpec => true,
            BufferSpec::StreamBufferSpec => false,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComponentReference {
    pub name: String, // RVDComponentSpec
    pub rel_path: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PartitionCounts {
    pub name: String, // PartitionCountsComponentSpec
    pub counts: Vec<u32>,
}
