use std::path::Path;

use anyhow::Result;

use crate::types::{Matrix, Metadata};

use super::{component, helpers};

pub fn matrix<T: AsRef<Path>>(path: T) -> Result<Matrix> {
    let path: &Path = path.as_ref();

    let metadata: Metadata = helpers::load_metadata_in(path)?;

    match metadata {
        Metadata::MatrixTableSpec(metadata) => {
            let components = &metadata.components;

            let globals = component(path.join(&components.globals.rel_path))?;
            let cols = component(path.join(&components.cols.rel_path))?;
            let rows = component(path.join(&components.rows.rel_path))?;
            let entries = component(path.join(&components.entries.rel_path))?;

            Ok(Matrix {
                globals,
                cols,
                rows,
                entries,
                metadata,
            })
        }
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
