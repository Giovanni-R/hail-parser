use std::path::Path;

use anyhow::Result;

#[test]
fn matrix_table_sample_vcf() -> Result<()> {
    let _ = parser::load::matrix(Path::new("..\\resources\\sample.vcf.mt"))?;
    Ok(())
}

#[test]
fn matrix_table_required_globals() -> Result<()> {
    let _ = parser::load::matrix(Path::new("..\\resources\\required_globals.mt"))?;
    Ok(())
}

#[test]
fn matrix_table_custom_references() -> Result<()> {
    let _ = parser::load::matrix(Path::new("..\\resources\\custom_references.mt"))?;
    Ok(())
}

#[test]
fn matrix_table_ex_vcf() -> Result<()> {
    let _ = parser::load::matrix(Path::new("..\\resources\\ex.vcf.mt"))?;
    Ok(())
}

#[test]
fn matrix_table_hg00096_g_vcf() -> Result<()> {
    let _ = parser::load::matrix(Path::new("..\\resources\\HG00096.g.vcf.gz.mt"))?;
    Ok(())
}

#[test]
fn matrix_table_sample_indexed_0252() -> Result<()> {
    let _ = parser::load::matrix(Path::new("..\\resources\\sample-indexed-0.2.52.mt"))?;
    Ok(())
}

#[test]
fn matrix_table_compat_150_0() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\matrix_table\\0.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_150_1() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\matrix_table\\1.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_150_2() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\matrix_table\\2.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_150_3() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\matrix_table\\3.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_150_4() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\matrix_table\\4.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_150_5() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\matrix_table\\5.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_150_6() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\matrix_table\\6.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_150_7() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.5.0\\matrix_table\\7.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_140_0() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.4.0\\matrix_table\\0.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_140_1() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.4.0\\matrix_table\\1.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_140_2() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.4.0\\matrix_table\\2.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_140_3() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.4.0\\matrix_table\\3.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_140_4() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.4.0\\matrix_table\\4.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_140_5() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.4.0\\matrix_table\\5.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_140_6() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.4.0\\matrix_table\\6.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_140_7() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.4.0\\matrix_table\\7.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_130_0() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.3.0\\matrix_table\\0.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_130_1() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.3.0\\matrix_table\\1.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_130_2() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.3.0\\matrix_table\\2.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_130_3() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.3.0\\matrix_table\\3.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_130_4() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.3.0\\matrix_table\\4.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_130_5() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.3.0\\matrix_table\\5.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_120_0() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.2.0\\matrix_table\\0.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_120_1() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.2.0\\matrix_table\\1.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_120_2() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.2.0\\matrix_table\\2.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_120_3() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.2.0\\matrix_table\\3.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_120_4() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.2.0\\matrix_table\\4.hmt",
    ))?;
    Ok(())
}

#[test]
fn matrix_table_compat_120_5() -> Result<()> {
    let _ = parser::load::matrix(Path::new(
        "..\\resources\\backward_compatability\\1.2.0\\matrix_table\\5.hmt",
    ))?;
    Ok(())
}

// #[test]
// fn matrix_table_compat_110_0() -> Result<()> {
//     let _ = parser::load::matrix(Path::new(
//         "..\\resources\\backward_compatability\\1.1.0\\matrix_table\\0.hmt",
//     ))?;
//     Ok(())
// }

// #[test]
// fn matrix_table_compat_110_1() -> Result<()> {
//     let _ = parser::load::matrix(Path::new(
//         "..\\resources\\backward_compatability\\1.1.0\\matrix_table\\1.hmt",
//     ))?;
//     Ok(())
// }

// #[test]
// fn matrix_table_compat_110_2() -> Result<()> {
//     let _ = parser::load::matrix(Path::new(
//         "..\\resources\\backward_compatability\\1.1.0\\matrix_table\\2.hmt",
//     ))?;
//     Ok(())
// }

// #[test]
// fn matrix_table_compat_110_3() -> Result<()> {
//     let _ = parser::load::matrix(Path::new(
//         "..\\resources\\backward_compatability\\1.1.0\\matrix_table\\3.hmt",
//     ))?;
//     Ok(())
// }

// #[test]
// fn matrix_table_compat_110_4() -> Result<()> {
//     let _ = parser::load::matrix(Path::new(
//         "..\\resources\\backward_compatability\\1.1.0\\matrix_table\\4.hmt",
//     ))?;
//     Ok(())
// }

// #[test]
// fn matrix_table_compat_110_5() -> Result<()> {
//     let _ = parser::load::matrix(Path::new(
//         "..\\resources\\backward_compatability\\1.1.0\\matrix_table\\5.hmt",
//     ))?;
//     Ok(())
// }
