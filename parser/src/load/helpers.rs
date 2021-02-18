use std::{convert::TryInto, path::Path};

use anyhow::Result;

use crate::{
    parse::{parse_rows, Encoding, StandardEncoder, UnsignedLEB128Encoder},
    types::{metadata::shared::BufferSpec, EType, HailValue, Metadata},
};

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
                true => parse_rows::<E>(&decompress_part_file(&file)?, row_schema)?,
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

pub fn decompress_part_file(raw: &[u8]) -> Result<Vec<u8>> {
    let final_size = get_total_original_size(raw)?;
    let mut decompressed = vec![0; final_size];

    let mut compressed_cursor = 0;
    let mut decompressed_cursor = 0;

    while compressed_cursor < raw.len() {
        let compressed_block_size: usize =
            u32::from_le_bytes(raw[compressed_cursor..(compressed_cursor + 4)].try_into()?)
                .try_into()?;
        let original_block_size: usize =
            u32::from_le_bytes(raw[(compressed_cursor + 4)..(compressed_cursor + 8)].try_into()?)
                .try_into()?;
        if original_block_size == 0 {
            break;
        }
        let block = &raw[(compressed_cursor + 8)..(compressed_cursor + 4 + compressed_block_size)];
        let destination_subslice =
            &mut decompressed[decompressed_cursor..(decompressed_cursor + original_block_size)];
        let _ = lzzzz::lz4::decompress_partial(block, destination_subslice, original_block_size)?;

        compressed_cursor += compressed_block_size + 4;
        decompressed_cursor += original_block_size;
    }

    Ok(decompressed)
}

pub fn get_total_original_size(raw: &[u8]) -> Result<usize> {
    let mut total = 0;
    let mut cursor = 0;

    while cursor < raw.len() {
        let block_size: usize =
            u32::from_le_bytes(raw[cursor..(cursor + 4)].try_into()?).try_into()?;
        let original_size: usize =
            u32::from_le_bytes(raw[(cursor + 4)..(cursor + 8)].try_into()?).try_into()?;
        total += original_size;
        cursor += block_size + 4
    }

    Ok(total)
}
