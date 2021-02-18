#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{convert::TryInto, path::Path};

use anyhow::Result;

fn main() -> Result<()> {
    let m = parser::load::matrix(Path::new("resources\\sample.vcf.mt"))?;

    println!("{:#?}", m.entries.metadata.encoded_type);
    Ok(())
}
