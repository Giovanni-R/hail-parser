mod helpers;

pub mod compression;
pub mod metadata;

use std::path::Path;

use anyhow::Result;

use crate::{Component, Matrix, Table};

pub fn component<T: AsRef<Path>>(path: T) -> Result<Component> {
    let path: &Path = path.as_ref();

    let metadata = metadata::load_component_metadata(path)?;
    let data_path = path.join("parts");

    let data = helpers::load_component_data(
        &metadata.part_files,
        &metadata.encoded_type,
        &metadata.buffer_spec,
        &data_path,
    )?;

    Ok(Component { data, metadata })
}

pub fn table<T: AsRef<Path>>(path: T) -> Result<Table> {
    let path: &Path = path.as_ref();

    let metadata = metadata::load_table_metadata(path)?;

    let components = &metadata.components;

    let globals = component(path.join(&components.globals.rel_path))?;
    let rows = component(path.join(&components.rows.rel_path))?;

    Ok(Table {
        globals,
        rows,
        metadata,
    })
}

pub fn matrix<T: AsRef<Path>>(path: T) -> Result<Matrix> {
    let path: &Path = path.as_ref();

    let metadata = metadata::load_matrix_metadata(path)?;

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
