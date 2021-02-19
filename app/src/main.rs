use std::path::PathBuf;

use anyhow::Result;

fn main() -> Result<()> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../resources/sample.vcf.mt");

    let m = parser::load::matrix(path)?;

    println!("{:#?}", m.entries.metadata.encoded_type);
    Ok(())
}
