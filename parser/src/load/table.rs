use std::path::Path;

use anyhow::Result;

use crate::types::{Metadata, Table};

use super::{component, helpers};

pub fn table<T: AsRef<Path>>(path: T) -> Result<Table> {
    let path: &Path = path.as_ref();

    let metadata: Metadata = helpers::load_metadata_in(path)?;

    match metadata {
        Metadata::TableSpec(metadata) => {
            let components = &metadata.components;

            let globals = component(path.join(&components.globals.rel_path))?;
            let rows = component(path.join(&components.rows.rel_path))?;

            Ok(Table {
                globals,
                rows,
                metadata,
            })
        }
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
