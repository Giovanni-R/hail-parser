use std::path::Path;

use anyhow::Result;

#[test]
fn table_sample_vcf_rows() -> Result<()> {
    let _ = parser::load::table(Path::new("..\\resources\\sample.vcf.mt\\rows"))?;
    Ok(())
}

#[test]
fn table_custom_references() -> Result<()> {
    let _ = parser::load::table(Path::new("..\\resources\\custom_references.t"))?;
    Ok(())
}

#[test]
fn table_custom_references_2() -> Result<()> {
    let _ = parser::load::table(Path::new("..\\resources\\custom_references_2.t"))?;
    Ok(())
}

#[test]
fn table_required_globals() -> Result<()> {
    let _ = parser::load::table(Path::new("..\\resources\\required_globals.ht"))?;
    Ok(())
}

#[test]
fn table_small_pheno() -> Result<()> {
    let _ = parser::load::table(Path::new("..\\resources\\small-pheno.t"))?;
    Ok(())
}

#[test]
fn table_three_key() -> Result<()> {
    let _ = parser::load::table(Path::new("..\\resources\\three_key.ht"))?;
    Ok(())
}

#[test]
fn table_compat_150_0() -> Result<()> {
    let _ = parser::load::table(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\table\\0.ht",
    ))?; // Doesn't use LEB128
    Ok(())
}

// #[test]
// fn table_compat_150_1() -> Result<()> {
//     let _ = parser::load::table(Path::new(
//         "..\\resources\\backward_compatability\\1.5.0\\table\\1.ht",
//     ))?; // Extra FF 01
//     Ok(())
// }

#[test]
fn table_compat_150_2() -> Result<()> {
    let _ = parser::load::table(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\table\\2.ht",
    ))?; // Doesn't use LEB128
    Ok(())
}

// #[test]
// fn table_compat_150_3() -> Result<()> {
//     let _ = parser::load::table(Path::new(
//         "..\\resources\\backward_compatability\\1.5.0\\table\\3.ht",
//     ))?; // Extra FF 01
//     Ok(())
// }

#[test]
fn table_compat_150_4() -> Result<()> {
    let _ = parser::load::table(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\table\\4.ht",
    ))?;
    Ok(())
}

// #[test]
// fn table_compat_150_5() -> Result<()> {
//     let _ = parser::load::table(Path::new(
//         "..\\resources\\backward_compatability\\1.5.0\\table\\5.ht",
//     ))?;
//     Ok(())
// }

#[test]
fn table_compat_150_6() -> Result<()> {
    let _ = parser::load::table(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\table\\6.ht",
    ))?;
    Ok(())
}

// #[test]
// fn table_compat_150_7() -> Result<()> {
//     let _ = parser::load::table(Path::new(
//         "..\\resources\\backward_compatability\\1.5.0\\table\\7.ht",
//     ))?;
//     Ok(())
// }
