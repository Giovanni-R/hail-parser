use std::path::Path;

use anyhow::Result;

use crate::{
    parse::{parse_rows, Encoding, StandardEncoder, UnsignedLEB128Encoder},
    types::{metadata::shared::BufferSpec, EType, HailValue, Metadata},
};

use super::compression;

pub fn load_metadata_in<T: AsRef<Path>>(path: T) -> Result<Metadata> {
    let metadata_path = path.as_ref().join("metadata.json.gz");
    let metadata_decoder = flate2::read::GzDecoder::new(std::fs::File::open(metadata_path)?);
    let metadata: Metadata = serde_json::from_reader(metadata_decoder)?;
    Ok(metadata)
}

pub fn load_component_data<T: AsRef<Path>>(
    part_files: &[String],
    row_schema: &EType,
    buffer_spec: &BufferSpec,
    path: T,
) -> Result<Vec<Vec<HailValue>>> {
    let path: &Path = path.as_ref();

    let data = match buffer_spec.uses_leb128() {
        true => _load_data::<UnsignedLEB128Encoder>(
            part_files,
            row_schema,
            buffer_spec.uses_compression(),
            buffer_spec.appends_length(),
            path,
        ),
        false => _load_data::<StandardEncoder>(
            part_files,
            row_schema,
            buffer_spec.uses_compression(),
            buffer_spec.appends_length(),
            path,
        ),
    }?;

    Ok(data)
}

fn _load_data<E: Encoding>(
    part_files: &[String],
    row_schema: &EType,
    is_compressed: bool,
    has_appended_length: bool,
    path: &Path,
) -> Result<Vec<Vec<HailValue>>> {
    let mut data = Vec::new();

    for part_name in part_files {
        let part_data = {
            let part_path = path.join(part_name);
            let file = std::fs::read(&part_path)?;
            match is_compressed {
                true => parse_rows::<E>(&compression::decompress_part_file(&file)?, row_schema)?,
                false => match has_appended_length {
                    true => parse_rows::<E>(&file[4..], row_schema)?,
                    false => parse_rows::<E>(&file, row_schema)?,
                },
            }
        };

        data.push(part_data);
    }

    Ok(data)
}
