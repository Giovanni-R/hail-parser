#![allow(dead_code)]
#![allow(clippy::type_complexity)]
use std::{collections::BTreeMap, path::PathBuf};

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Deserialize};

use serde_hail::types::{Call, Interval, Locus, NDArray};

fn parse_table<G, R>(file: &str) -> Result<()>
where
    G: DeserializeOwned,
    R: DeserializeOwned,
{
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../resources")
        .join(file);
    let _ = serde_hail::load::table::<G, R, &PathBuf>(&path)
        .context(format!("Failed to load table in path: {:?}", path))?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct EmptyGlobal;

#[test]
fn table_sample_vcf_rows() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Info {
        negative_train_site: Option<bool>,
        hwp: Option<f64>,
        ac: Option<Vec<Option<u32>>>,
        culprit: Option<String>,
        mq0: Option<u32>,
        read_pos_rank_sum: Option<f64>,
        an: Option<u32>,
        inbreeding_coeff: Option<f64>,
        af: Option<Vec<Option<f64>>>,
        gq_stddev: Option<f64>,
        fs: Option<f64>,
        dp: Option<u32>,
        gq_mean: Option<f64>,
        positive_train_site: Option<bool>,
        vqslod: Option<f64>,
        clipping_rank_sum: Option<f64>,
        base_q_rank_sum: Option<f64>,
        mleaf: Option<Vec<Option<f64>>>,
        mleac: Option<Vec<Option<u32>>>,
        mq: Option<f64>,
        qd: Option<f64>,
        end: Option<u32>,
        db: Option<bool>,
        haplotype_score: Option<f64>,
        mq_rank_sum: Option<f64>,
        ccc: Option<u32>,
        ncc: Option<u32>,
        ds: Option<bool>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        locus: Option<Locus>,
        alleles: Option<Vec<Option<String>>>,
        rsid: Option<String>,
        qual: Option<f64>,
        filters: Option<Vec<Option<String>>>,
        info: Option<Info>,
    }
    parse_table::<EmptyGlobal, Row>("sample.vcf.mt/rows")
}

#[test]
fn table_custom_references() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Row {
        idx: Option<u32>,
        locus_1: Option<Locus>,
        locus_2: Option<Locus>,
    }
    parse_table::<EmptyGlobal, Row>("custom_references.t")
}

#[test]
fn table_custom_references_2() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Row {
        idx: Option<u32>,
        locus_1: Option<Locus>,
        locus_2: Option<Locus>,
    }
    parse_table::<EmptyGlobal, Row>("custom_references_2.t")
}

#[test]
fn table_required_globals() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Row {
        idx: Option<u32>,
    }
    parse_table::<EmptyGlobal, Row>("required_globals.ht")
}

#[test]
fn table_small_pheno() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Row {
        s: Option<String>,
        phenotype: Option<f64>,
    }
    parse_table::<EmptyGlobal, Row>("small-pheno.t")
}

#[test]
fn table_three_key() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Row {
        x: u32,
        y: u32,
        z: u32,
    }
    parse_table::<EmptyGlobal, Row>("three_key.ht")
}

#[test]
fn table_compat_150() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct GlobalAstruct {
        a: Option<u32>,
        b: f64,
    }
    #[derive(Debug, Deserialize)]
    pub struct GlobalMstruct {
        x: u32,
        y: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Global {
        global_f_32: f32,
        global_i_64: i64,
        global_m: Option<f64>,
        global_astruct: GlobalAstruct,
        global_mstruct: Option<GlobalMstruct>,
        global_aset: Vec<String>,
        global_mset: Option<Vec<f64>>,
        global_d: BTreeMap<Vec<Option<String>>, f64>,
        global_md: Option<BTreeMap<u32, String>>,
        global_h_38: Locus,
        global_ml: Option<Locus>,
        global_i: Interval<Locus>,
        global_c: Call,
        global_mc: Option<Call>,
        global_t: (Call, String, Option<String>),
        global_mt: Option<(Locus, bool)>,
        global_nd: NDArray<u32, 2usize>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Astruct {
        a: Option<u32>,
        b: f64,
    }
    #[derive(Debug, Deserialize)]
    pub struct Mstruct {
        x: u32,
        y: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        idx: u32,
        f_32: f32,
        i_64: i64,
        m: Option<f64>,
        astruct: Astruct,
        mstruct: Option<Mstruct>,
        aset: Vec<String>,
        mset: Option<Vec<f64>>,
        d: BTreeMap<Vec<Option<String>>, f64>,
        md: Option<BTreeMap<u32, String>>,
        h_38: Locus,
        ml: Option<Locus>,
        i: Interval<Locus>,
        c: Call,
        mc: Option<Call>,
        t: (Call, String, Option<String>),
        mt: Option<(Locus, bool)>,
        nd: NDArray<u32, 2usize>,
    }

    parse_table::<Global, Row>("backward_compatability/1.5.0/table/0.ht")?; // Doesn't use LEB128
    parse_table::<Global, Row>("backward_compatability/1.5.0/table/1.ht")?; // Negative LEB128 Int64
    parse_table::<Global, Row>("backward_compatability/1.5.0/table/2.ht")?; // Doesn't use LEB128
    parse_table::<Global, Row>("backward_compatability/1.5.0/table/3.ht")?; // Negative LEB128 Int64
    parse_table::<Global, Row>("backward_compatability/1.5.0/table/4.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.5.0/table/5.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.5.0/table/6.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.5.0/table/7.ht")?;
    Ok(())
}

#[test]
fn table_compat_140() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct GlobalAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct GlobalMstruct {
        x: u32,
        y: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Global {
        global_f_32: f32,
        global_i_64: i64,
        global_m: Option<f64>,
        global_astruct: GlobalAstruct,
        global_mstruct: Option<GlobalMstruct>,
        global_aset: Vec<Option<String>>,
        global_mset: Option<Vec<f64>>,
        global_d: BTreeMap<Option<Vec<Option<String>>>, Option<f64>>,
        global_md: Option<BTreeMap<u32, String>>,
        global_h_38: Locus,
        global_ml: Option<Locus>,
        global_i: Interval<Option<Locus>>,
        global_c: Call,
        global_mc: Option<Call>,
        global_t: (Option<Call>, Option<String>, Option<String>),
        global_mt: Option<(Locus, bool)>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Astruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Mstruct {
        x: u32,
        y: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        idx: u32,
        f_32: f32,
        i_64: i64,
        m: Option<f64>,
        astruct: Astruct,
        mstruct: Option<Mstruct>,
        aset: Vec<Option<String>>,
        mset: Option<Vec<f64>>,
        d: BTreeMap<Option<Vec<Option<String>>>, Option<f64>>,
        md: Option<BTreeMap<u32, String>>,
        h_38: Locus,
        ml: Option<Locus>,
        i: Interval<Option<Locus>>,
        c: Call,
        mc: Option<Call>,
        t: (Option<Call>, Option<String>, Option<String>),
        mt: Option<(Locus, bool)>,
    }

    parse_table::<Global, Row>("backward_compatability/1.4.0/table/0.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.4.0/table/1.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.4.0/table/2.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.4.0/table/3.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.4.0/table/4.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.4.0/table/5.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.4.0/table/6.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.4.0/table/7.ht")?;

    Ok(())
}

#[test]
fn table_compat_130() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct GlobalAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct GlobalMstruct {
        x: Option<u32>,
        y: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Global {
        global_f_32: Option<f32>,
        global_i_64: Option<i64>,
        global_m: Option<f64>,
        global_astruct: Option<GlobalAstruct>,
        global_mstruct: Option<GlobalMstruct>,
        global_aset: Option<Vec<Option<String>>>,
        global_mset: Option<Vec<Option<f64>>>,
        global_d: Option<BTreeMap<Option<Vec<Option<String>>>, Option<f64>>>,
        global_md: Option<BTreeMap<Option<u32>, Option<String>>>,
        global_h_38: Option<Locus>,
        global_ml: Option<Locus>,
        global_i: Option<Interval<Option<Locus>>>,
        global_c: Option<Call>,
        global_mc: Option<Call>,
        global_t: Option<(Option<Call>, Option<String>, Option<String>)>,
        global_mt: Option<(Option<Locus>, Option<bool>)>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Astruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Mstruct {
        x: Option<u32>,
        y: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        idx: Option<u32>,
        f_32: Option<f32>,
        i_64: Option<i64>,
        m: Option<f64>,
        astruct: Option<Astruct>,
        mstruct: Option<Mstruct>,
        aset: Option<Vec<Option<String>>>,
        mset: Option<Vec<Option<f64>>>,
        d: Option<BTreeMap<Option<Vec<Option<String>>>, Option<f64>>>,
        md: Option<BTreeMap<Option<u32>, Option<String>>>,
        h_38: Option<Locus>,
        ml: Option<Locus>,
        i: Option<Interval<Option<Locus>>>,
        c: Option<Call>,
        mc: Option<Call>,
        t: Option<(Option<Call>, Option<String>, Option<String>)>,
        mt: Option<(Option<Locus>, Option<bool>)>,
    }
    parse_table::<Global, Row>("backward_compatability/1.3.0/table/0.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.3.0/table/1.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.3.0/table/2.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.3.0/table/3.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.3.0/table/4.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.3.0/table/5.ht")?;
    Ok(())
}

#[test]
fn table_compat_120() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct GlobalAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct GlobalMstruct {
        x: Option<u32>,
        y: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Global {
        global_f_32: Option<f32>,
        global_i_64: Option<i64>,
        global_m: Option<f64>,
        global_astruct: Option<GlobalAstruct>,
        global_mstruct: Option<GlobalMstruct>,
        global_aset: Option<Vec<Option<String>>>,
        global_mset: Option<Vec<Option<f64>>>,
        global_d: Option<BTreeMap<Option<Vec<Option<String>>>, Option<f64>>>,
        global_md: Option<BTreeMap<Option<u32>, Option<String>>>,
        global_h_38: Option<Locus>,
        global_ml: Option<Locus>,
        global_i: Option<Interval<Option<Locus>>>,
        global_c: Option<Call>,
        global_mc: Option<Call>,
        global_t: Option<(Option<Call>, Option<String>, Option<String>)>,
        global_mt: Option<(Option<Locus>, Option<bool>)>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Astruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Mstruct {
        x: Option<u32>,
        y: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        idx: Option<u32>,
        f_32: Option<f32>,
        i_64: Option<i64>,
        m: Option<f64>,
        astruct: Option<Astruct>,
        mstruct: Option<Mstruct>,
        aset: Option<Vec<Option<String>>>,
        mset: Option<Vec<Option<f64>>>,
        d: Option<BTreeMap<Option<Vec<Option<String>>>, Option<f64>>>,
        md: Option<BTreeMap<Option<u32>, Option<String>>>,
        h_38: Option<Locus>,
        ml: Option<Locus>,
        i: Option<Interval<Option<Locus>>>,
        c: Option<Call>,
        mc: Option<Call>,
        t: Option<(Option<Call>, Option<String>, Option<String>)>,
        mt: Option<(Option<Locus>, Option<bool>)>,
    }
    parse_table::<Global, Row>("backward_compatability/1.2.0/table/0.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.2.0/table/1.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.2.0/table/2.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.2.0/table/3.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.2.0/table/4.ht")?;
    parse_table::<Global, Row>("backward_compatability/1.2.0/table/5.ht")?;
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
