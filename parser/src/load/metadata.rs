use std::path::Path;

use anyhow::Result;

use crate::types::{
    metadata::{ComponentMetadata, MatrixMetadata, TableMetadata},
    Metadata,
};

use crate::load;

pub fn load_component_metadata<T: AsRef<Path>>(path: T) -> Result<ComponentMetadata> {
    match load::helpers::load_metadata_in(path)? {
        Metadata::OrderedRVDSpec(metadata) => Ok(metadata.into()),
        Metadata::IndexedRVDSpec(metadata) => Ok(metadata.into()),
        Metadata::UnpartitionedRVDSpec(metadata) => Ok(metadata.into()),
        Metadata::OrderedRVDSpec2(metadata) => Ok(metadata.into()),
        Metadata::IndexedRVDSpec2(metadata) => Ok(metadata.into()),
        Metadata::TableSpec(ref table_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a component, found a table {:?}",
            table_metadata
        ))),
        Metadata::MatrixTableSpec(ref matrix_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a component, found a matrix {:?}",
            matrix_metadata
        ))),
    }
}

pub fn load_table_metadata<T: AsRef<Path>>(path: T) -> Result<TableMetadata> {
    match load::helpers::load_metadata_in(path)? {
        Metadata::TableSpec(metadata) => Ok(metadata),
        Metadata::OrderedRVDSpec(ref comp_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a table, found an ordered component {:?}",
            comp_metadata
        ))),
        Metadata::IndexedRVDSpec(ref comp_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a table, found an indexed component {:?}",
            comp_metadata
        ))),
        Metadata::UnpartitionedRVDSpec(ref comp_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a table, found an unpartitioned component {:?}",
            comp_metadata
        ))),
        Metadata::OrderedRVDSpec2(ref comp_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a table, found an ordered component (version 2) {:?}",
            comp_metadata
        ))),
        Metadata::IndexedRVDSpec2(ref comp_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a table, found an indexed component (version 2) {:?}",
            comp_metadata
        ))),
        Metadata::MatrixTableSpec(ref matrix_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a table, found a matrix {:?}",
            matrix_metadata
        ))),
    }
}

pub fn load_matrix_metadata<T: AsRef<Path>>(path: T) -> Result<MatrixMetadata> {
    match load::helpers::load_metadata_in(path)? {
        Metadata::MatrixTableSpec(metadata) => Ok(metadata),
        Metadata::OrderedRVDSpec(ref comp_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a matrix, found an ordered component {:?}",
            comp_metadata
        ))),
        Metadata::IndexedRVDSpec(ref comp_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a matrix, found an indexed component {:?}",
            comp_metadata
        ))),
        Metadata::UnpartitionedRVDSpec(ref comp_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a matrix, found an unpartitioned component {:?}",
            comp_metadata
        ))),
        Metadata::OrderedRVDSpec2(ref comp_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a matrix, found an ordered component (version 2) {:?}",
            comp_metadata
        ))),
        Metadata::IndexedRVDSpec2(ref comp_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a matrix, found an indexed component (version 2) {:?}",
            comp_metadata
        ))),
        Metadata::TableSpec(ref table_metadata) => Err(anyhow::anyhow!(format!(
            "Expected a matrix, found a table {:?}",
            table_metadata
        ))),
    }
}
