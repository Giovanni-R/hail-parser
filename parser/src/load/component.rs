use std::path::Path;

use anyhow::Result;

use crate::types::{metadata::ComponentMetadata, Component, Metadata};

use super::helpers;

pub fn component<T: AsRef<Path>>(path: T) -> Result<Component> {
    let path: &Path = path.as_ref();

    let metadata: ComponentMetadata = match helpers::load_metadata_in(path)? {
        Metadata::OrderedRVDSpec(metadata) => metadata.into(),
        Metadata::IndexedRVDSpec(metadata) => metadata.into(),
        Metadata::UnpartitionedRVDSpec(metadata) => metadata.into(),
        Metadata::OrderedRVDSpec2(metadata) => metadata.into(),
        Metadata::IndexedRVDSpec2(metadata) => metadata.into(),
        Metadata::TableSpec(ref table_metadata) => {
            return Err(anyhow::anyhow!(format!(
                "Expected a component, found a table {:?}",
                table_metadata
            )))
        }
        Metadata::MatrixTableSpec(ref matrix_metadata) => {
            return Err(anyhow::anyhow!(format!(
                "Expected a component, found a matrix {:?}",
                matrix_metadata
            )))
        }
    };
    let data_path = path.join("parts");

    let data = helpers::load_component_data(
        &metadata.part_files,
        &metadata.encoded_type,
        &metadata.buffer_spec,
        &data_path,
    )?;

    Ok(Component { data, metadata })
}
