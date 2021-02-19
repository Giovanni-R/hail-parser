use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

fn parse_table(file: &str) -> Result<()> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../resources")
        .join(file);
    let _ = parser::load::table(Path::new(&path))
        .context(format!("Failed to load table in path: {:?}", path))?;
    Ok(())
}

#[test]
fn table_sample_vcf_rows() -> Result<()> {
    parse_table("sample.vcf.mt/rows")
}

#[test]
fn table_custom_references() -> Result<()> {
    parse_table("custom_references.t")
}

#[test]
fn table_custom_references_2() -> Result<()> {
    parse_table("custom_references_2.t")
}

#[test]
fn table_required_globals() -> Result<()> {
    parse_table("required_globals.ht")
}

#[test]
fn table_small_pheno() -> Result<()> {
    parse_table("small-pheno.t")
}

#[test]
fn table_three_key() -> Result<()> {
    parse_table("three_key.ht")
}

#[test]
fn table_compat_150() -> Result<()> {
    parse_table("backward_compatability/1.5.0/table/0.ht")?; // Doesn't use LEB128
    parse_table("backward_compatability/1.5.0/table/1.ht")?; // Negative LEB128 Int64
    parse_table("backward_compatability/1.5.0/table/2.ht")?; // Doesn't use LEB128
    parse_table("backward_compatability/1.5.0/table/3.ht")?; // Negative LEB128 Int64
    parse_table("backward_compatability/1.5.0/table/4.ht")?;
    parse_table("backward_compatability/1.5.0/table/5.ht")?;
    parse_table("backward_compatability/1.5.0/table/6.ht")?;
    parse_table("backward_compatability/1.5.0/table/7.ht")?;
    Ok(())
}

#[test]
fn table_compat_140() -> Result<()> {
    parse_table("backward_compatability/1.4.0/table/0.ht")?;
    parse_table("backward_compatability/1.4.0/table/1.ht")?;
    parse_table("backward_compatability/1.4.0/table/2.ht")?;
    parse_table("backward_compatability/1.4.0/table/3.ht")?;
    parse_table("backward_compatability/1.4.0/table/4.ht")?;
    parse_table("backward_compatability/1.4.0/table/5.ht")?;
    parse_table("backward_compatability/1.4.0/table/6.ht")?;
    parse_table("backward_compatability/1.4.0/table/7.ht")?;
    Ok(())
}

#[test]
fn table_compat_130() -> Result<()> {
    parse_table("backward_compatability/1.3.0/table/0.ht")?;
    parse_table("backward_compatability/1.3.0/table/1.ht")?;
    parse_table("backward_compatability/1.3.0/table/2.ht")?;
    parse_table("backward_compatability/1.3.0/table/3.ht")?;
    parse_table("backward_compatability/1.3.0/table/4.ht")?;
    parse_table("backward_compatability/1.3.0/table/5.ht")?;
    Ok(())
}

#[test]
fn table_compat_120() -> Result<()> {
    parse_table("backward_compatability/1.2.0/table/0.ht")?;
    parse_table("backward_compatability/1.2.0/table/1.ht")?;
    parse_table("backward_compatability/1.2.0/table/2.ht")?;
    parse_table("backward_compatability/1.2.0/table/3.ht")?;
    parse_table("backward_compatability/1.2.0/table/4.ht")?;
    parse_table("backward_compatability/1.2.0/table/5.ht")?;
    Ok(())
}

// #[test]
// fn table_compat_110() -> Result<()> {
//     parse_table("backward_compatability/1.1.0/table/0.ht")?;
//     parse_table("backward_compatability/1.1.0/table/1.ht")?;
//     parse_table("backward_compatability/1.1.0/table/2.ht")?;
//     parse_table("backward_compatability/1.1.0/table/3.ht")?;
//     parse_table("backward_compatability/1.1.0/table/4.ht")?;
//     parse_table("backward_compatability/1.1.0/table/5.ht")?;
//     Ok(())
// }
