use std::path::Path;

use anyhow::Result;

use parser::{
    parse::{Encoding, StandardEncoder, UnsignedLEB128Encoder},
    types::metadata::shared::BufferSpec,
};

use crate::parse_rows;

pub fn load_component_data_with_serde<T>(
    part_files: &[String],
    buffer_spec: &BufferSpec,
    path: &Path,
) -> Result<Vec<Vec<T>>>
where
    T: serde::de::DeserializeOwned,
{
    let data = match buffer_spec.uses_leb128() {
        true => _load_data_with_serde::<T, UnsignedLEB128Encoder>(
            part_files,
            buffer_spec.uses_compression(),
            buffer_spec.appends_length(),
            path,
        ),
        false => _load_data_with_serde::<T, StandardEncoder>(
            part_files,
            buffer_spec.uses_compression(),
            buffer_spec.appends_length(),
            path,
        ),
    }?;

    Ok(data)
}

fn _load_data_with_serde<T, E>(
    part_files: &[String],
    is_compressed: bool,
    has_appended_length: bool,
    path: &Path,
) -> Result<Vec<Vec<T>>>
where
    E: Encoding,
    T: serde::de::DeserializeOwned,
{
    let mut data = Vec::new();

    for part_name in part_files {
        let part_data = {
            let part_path = path.join(part_name);
            let file = std::fs::read(&part_path)?;
            if is_compressed {
                parse_rows::<T, E>(&parser::load::compression::decompress_part_file(&file)?)?
            } else {
                let mut file: &[u8] = &file;
                if has_appended_length {
                    file = &file[4..];
                };
                parse_rows::<T, E>(file)?
            }
        };

        data.push(part_data);
    }

    Ok(data)
}
