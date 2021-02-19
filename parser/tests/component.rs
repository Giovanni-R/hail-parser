use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

fn parse_component(file: &str) -> Result<()> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../resources")
        .join(file);
    let _ = parser::load::component(Path::new(&path))
        .context(format!("Failed to load component in path: {:?}", path))?;
    Ok(())
}

#[test]
fn component_sample_rows() -> Result<()> {
    parse_component("sample.vcf.mt/rows/rows")
}

#[test]
fn component_sample_columns() -> Result<()> {
    parse_component("sample.vcf.mt/cols/rows")
}

#[test]
fn component_sample_entries() -> Result<()> {
    parse_component("sample.vcf.mt/entries/rows")
}
