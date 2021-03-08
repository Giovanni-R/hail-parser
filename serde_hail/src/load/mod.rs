use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::de::DeserializeOwned;

use crate::types::{SerdeComponent, SerdeMatrix, SerdeTable};

mod helpers;

pub fn component<R, P>(path: P) -> Result<SerdeComponent<R>>
where
    R: DeserializeOwned,
    P: AsRef<Path>,
{
    let path: &Path = path.as_ref();

    let metadata = parser::load::metadata::load_component_metadata(path)?;
    let data_path = path.join("parts");

    let data = helpers::load_component_data_with_serde::<R>(
        &metadata.part_files,
        &metadata.buffer_spec,
        &data_path,
    )?;

    Ok(SerdeComponent { data, metadata })
}

pub fn table<G, R, P>(path: P) -> Result<SerdeTable<G, R>>
where
    G: serde::de::DeserializeOwned,
    R: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let path: &Path = path.as_ref();

    let metadata = parser::load::metadata::load_table_metadata(path)?;

    let components = &metadata.components;

    let globals_path = path.join(&components.globals.rel_path);
    let rows_path = path.join(&components.rows.rel_path);

    let globals = component::<G, PathBuf>(globals_path)?;
    let rows = component::<R, PathBuf>(rows_path)?;

    Ok(SerdeTable {
        globals,
        rows,
        metadata,
    })
}

pub fn matrix<G, C, R, E, P>(path: P) -> Result<SerdeMatrix<G, C, R, E>>
where
    G: serde::de::DeserializeOwned,
    C: serde::de::DeserializeOwned,
    R: serde::de::DeserializeOwned,
    E: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let path: &Path = path.as_ref();

    let metadata = parser::load::metadata::load_matrix_metadata(path)?;

    let components = &metadata.components;

    let globals_path = path.join(&components.globals.rel_path);
    let cols_path = path.join(&components.cols.rel_path);
    let rows_path = path.join(&components.rows.rel_path);
    let entries_path = path.join(&components.entries.rel_path);

    let globals = component::<G, PathBuf>(globals_path)?;
    let cols = component::<C, PathBuf>(cols_path)?;
    let rows = component::<R, PathBuf>(rows_path)?;
    let entries = component::<E, PathBuf>(entries_path)?;

    Ok(SerdeMatrix {
        globals,
        cols,
        rows,
        entries,
        metadata,
    })
}
