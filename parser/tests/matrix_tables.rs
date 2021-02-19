use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

fn parse_matrix(file: &str) -> Result<()> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../resources")
        .join(file);
    let _ = parser::load::matrix(Path::new(&path))
        .context(format!("Failed to load matrix in path: {:?}", path))?;
    Ok(())
}

#[test]
fn matrix_table_sample_vcf() -> Result<()> {
    parse_matrix("sample.vcf.mt")
}

#[test]
fn matrix_table_required_globals() -> Result<()> {
    parse_matrix("required_globals.mt")
}

#[test]
fn matrix_table_custom_references() -> Result<()> {
    parse_matrix("custom_references.mt")
}

#[test]
fn matrix_table_ex_vcf() -> Result<()> {
    parse_matrix("ex.vcf.mt")
}

#[test]
fn matrix_table_hg00096_g_vcf() -> Result<()> {
    parse_matrix("HG00096.g.vcf.gz.mt")
}

#[test]
fn matrix_table_sample_indexed_0252() -> Result<()> {
    parse_matrix("sample-indexed-0.2.52.mt")
}

#[test]
fn matrix_table_compat_150() -> Result<()> {
    parse_matrix("backward_compatability/1.5.0/matrix_table/0.hmt")?;
    parse_matrix("backward_compatability/1.5.0/matrix_table/1.hmt")?;
    parse_matrix("backward_compatability/1.5.0/matrix_table/2.hmt")?;
    parse_matrix("backward_compatability/1.5.0/matrix_table/3.hmt")?;
    parse_matrix("backward_compatability/1.5.0/matrix_table/4.hmt")?;
    parse_matrix("backward_compatability/1.5.0/matrix_table/5.hmt")?;
    parse_matrix("backward_compatability/1.5.0/matrix_table/6.hmt")?;
    parse_matrix("backward_compatability/1.5.0/matrix_table/7.hmt")?;
    Ok(())
}

#[test]
fn matrix_table_compat_140() -> Result<()> {
    parse_matrix("backward_compatability/1.4.0/matrix_table/0.hmt")?;
    parse_matrix("backward_compatability/1.4.0/matrix_table/1.hmt")?;
    parse_matrix("backward_compatability/1.4.0/matrix_table/2.hmt")?;
    parse_matrix("backward_compatability/1.4.0/matrix_table/3.hmt")?;
    parse_matrix("backward_compatability/1.4.0/matrix_table/4.hmt")?;
    parse_matrix("backward_compatability/1.4.0/matrix_table/5.hmt")?;
    parse_matrix("backward_compatability/1.4.0/matrix_table/6.hmt")?;
    parse_matrix("backward_compatability/1.4.0/matrix_table/7.hmt")?;
    Ok(())
}

#[test]
fn matrix_table_compat_130() -> Result<()> {
    parse_matrix("backward_compatability/1.3.0/matrix_table/0.hmt")?;
    parse_matrix("backward_compatability/1.3.0/matrix_table/1.hmt")?;
    parse_matrix("backward_compatability/1.3.0/matrix_table/2.hmt")?;
    parse_matrix("backward_compatability/1.3.0/matrix_table/3.hmt")?;
    parse_matrix("backward_compatability/1.3.0/matrix_table/4.hmt")?;
    parse_matrix("backward_compatability/1.3.0/matrix_table/5.hmt")?;
    Ok(())
}

#[test]
fn matrix_table_compat_120() -> Result<()> {
    parse_matrix("backward_compatability/1.2.0/matrix_table/0.hmt")?;
    parse_matrix("backward_compatability/1.2.0/matrix_table/1.hmt")?;
    parse_matrix("backward_compatability/1.2.0/matrix_table/2.hmt")?;
    parse_matrix("backward_compatability/1.2.0/matrix_table/3.hmt")?;
    parse_matrix("backward_compatability/1.2.0/matrix_table/4.hmt")?;
    parse_matrix("backward_compatability/1.2.0/matrix_table/5.hmt")?;
    Ok(())
}

// #[test]
// fn matrix_table_compat_110() -> Result<()> {
//     parse_matrix("backward_compatability/1.1.0/matrix_table/0.hmt")?;
//     parse_matrix("backward_compatability/1.1.0/matrix_table/1.hmt")?;
//     parse_matrix("backward_compatability/1.1.0/matrix_table/2.hmt")?;
//     parse_matrix("backward_compatability/1.1.0/matrix_table/3.hmt")?;
//     parse_matrix("backward_compatability/1.1.0/matrix_table/4.hmt")?;
//     parse_matrix("backward_compatability/1.1.0/matrix_table/5.hmt")?;
//     Ok(())
// }
