use std::path::Path;

use anyhow::Result;

#[test]
fn component_1() -> Result<()> {
    let _ = parser::load::component(Path::new("..\\resources\\sample.vcf.mt\\rows\\rows"))?;
    Ok(())
}

#[test]
fn component_2() -> Result<()> {
    let _ = parser::load::component(Path::new("..\\resources\\sample.vcf.mt\\cols\\rows"))?;
    Ok(())
}

#[test]
fn component_3() -> Result<()> {
    let _ = parser::load::component(Path::new("..\\resources\\sample.vcf.mt\\entries\\rows"))?;
    Ok(())
}
