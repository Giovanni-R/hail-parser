#![allow(dead_code)]
#![allow(clippy::type_complexity)]
use std::{collections::BTreeMap, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use serde_hail::types::{Call, Interval, Locus, NDArray};

fn parse_matrix<G, C, R, E>(file: &str) -> Result<()>
where
    G: serde::de::DeserializeOwned,
    C: serde::de::DeserializeOwned,
    R: serde::de::DeserializeOwned,
    E: serde::de::DeserializeOwned,
{
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../resources")
        .join(file);
    let _ = serde_hail::load::matrix::<G, C, R, E, &PathBuf>(&path)
        .context(format!("Failed to load matrix in path: {:?}", path))?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct EmptyGlobal;

#[test]
fn matrix_table_sample_vcf() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Column {
        s: Option<String>,
    }
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
    #[derive(Debug, Deserialize)]
    pub struct Entry {
        gt: Option<Call>,
        ad: Option<Vec<u32>>,
        dp: Option<u32>,
        gq: Option<u32>,
        pl: Option<Vec<u32>>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryRow {
        entries: Option<Vec<Option<Entry>>>,
    }

    parse_matrix::<EmptyGlobal, Column, Row, EntryRow>("sample.vcf.mt")?;

    Ok(())
}

#[test]
fn matrix_table_required_globals() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Column {
        col_idx: Option<u32>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        row_idx: Option<u32>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Entry;
    #[derive(Debug, Deserialize)]
    pub struct EntryRow {
        entries: Option<Vec<Option<Entry>>>,
    }

    parse_matrix::<EmptyGlobal, Column, Row, EntryRow>("required_globals.mt")?;

    Ok(())
}

#[test]
fn matrix_table_custom_references() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Column {
        col_idx: Option<u32>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        row_idx: Option<u32>,
        locus_1: Option<Locus>,
        locus_2: Option<Locus>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Entry;
    #[derive(Debug, Deserialize)]
    pub struct EntryRow {
        entries: Option<Vec<Option<Entry>>>,
    }

    parse_matrix::<EmptyGlobal, Column, Row, EntryRow>("custom_references.mt")?;

    Ok(())
}

#[test]
fn matrix_table_ex_vcf() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Column {
        s: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Info {
        ns: Option<u32>,
        dp: Option<u32>,
        af: Option<Vec<Option<f64>>>,
        aa: Option<String>,
        test: Option<Vec<Option<String>>>,
        test_2: Option<Vec<Option<u32>>>,
        db: Option<bool>,
        h2: Option<bool>,
        homseq: Option<Vec<Option<String>>>,
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
    #[derive(Debug, Deserialize)]
    pub struct Entry {
        gt: Option<Call>,
        gq: Option<u32>,
        dp: Option<u32>,
        hq: Option<Vec<Option<u32>>>,
        cnl: Option<Vec<Option<u32>>>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryRow {
        entries: Option<Vec<Option<Entry>>>,
    }

    parse_matrix::<EmptyGlobal, Column, Row, EntryRow>("ex.vcf.mt")?;

    Ok(())
}

#[test]
fn matrix_table_hg00096_g_vcf() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Column {
        s: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Info {
        base_q_rank_sum: Option<f64>,
        clipping_rank_sum: Option<f64>,
        dp: Option<u32>,
        ds: Option<bool>,
        end: Option<u32>,
        excess_het: Option<f64>,
        haplotype_score: Option<f64>,
        inbreeding_coeff: Option<f64>,
        mleac: Option<Vec<Option<u32>>>,
        mleaf: Option<Vec<Option<f64>>>,
        mq: Option<f64>,
        mq_rank_sum: Option<f64>,
        raw_mq: Option<f64>,
        read_pos_rank_sum: Option<f64>,
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
    #[derive(Debug, Deserialize)]
    pub struct Entry {
        ad: Option<Vec<u32>>,
        dp: Option<u32>,
        gq: Option<u32>,
        gt: Option<Call>,
        min_dp: Option<u32>,
        pgt: Option<Call>,
        pid: Option<String>,
        pl: Option<Vec<u32>>,
        sb: Option<Vec<u32>>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryRow {
        entries: Option<Vec<Option<Entry>>>,
    }

    parse_matrix::<EmptyGlobal, Column, Row, EntryRow>("HG00096.g.vcf.gz.mt")?;

    Ok(())
}

#[test]
fn matrix_table_sample_indexed_0252() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Column {
        s: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Info {
        ns: Option<u32>,
        dp: Option<u32>,
        af: Option<Vec<Option<f64>>>,
        aa: Option<String>,
        test: Option<Vec<Option<String>>>,
        test_2: Option<Vec<Option<u32>>>,
        db: Option<bool>,
        h2: Option<bool>,
        homseq: Option<Vec<Option<String>>>,
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
    #[derive(Debug, Deserialize)]
    pub struct Entry {
        gt: Option<Call>,
        gq: Option<u32>,
        dp: Option<u32>,
        hq: Option<Vec<Option<u32>>>,
        cnl: Option<Vec<Option<u32>>>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryRow {
        entries: Option<Vec<Option<Entry>>>,
    }

    parse_matrix::<EmptyGlobal, Column, Row, EntryRow>("sample-indexed-0.2.52.mt")?;

    Ok(())
}

#[test]
fn matrix_table_compat_150() -> Result<()> {
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
    pub struct ColAstruct {
        a: Option<u32>,
        b: f64,
    }
    #[derive(Debug, Deserialize)]
    pub struct ColMstruct {
        x: u32,
        y: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Column {
        col_idx: u32,
        col_f_32: f32,
        col_i_64: i64,
        col_m: Option<f64>,
        col_astruct: ColAstruct,
        col_mstruct: Option<ColMstruct>,
        col_aset: Vec<String>,
        col_mset: Option<Vec<f64>>,
        col_d: BTreeMap<Vec<Option<String>>, f64>,
        col_md: Option<BTreeMap<u32, String>>,
        col_h_38: Locus,
        col_ml: Option<Locus>,
        col_i: Interval<Locus>,
        col_c: Call,
        col_mc: Option<Call>,
        col_t: (Call, String, Option<String>),
        col_mt: Option<(Locus, bool)>,
        col_nd: NDArray<u32, 2usize>,
    }
    #[derive(Debug, Deserialize)]
    pub struct RowAstruct {
        a: Option<u32>,
        b: f64,
    }
    #[derive(Debug, Deserialize)]
    pub struct RowMstruct {
        x: u32,
        y: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        row_idx: u32,
        row_f_32: f32,
        row_i_64: i64,
        row_m: Option<f64>,
        row_astruct: RowAstruct,
        row_mstruct: Option<RowMstruct>,
        row_aset: Vec<String>,
        row_mset: Option<Vec<f64>>,
        row_d: BTreeMap<Vec<Option<String>>, f64>,
        row_md: Option<BTreeMap<u32, String>>,
        row_h_38: Locus,
        row_ml: Option<Locus>,
        row_i: Interval<Locus>,
        row_c: Call,
        row_mc: Option<Call>,
        row_t: (Call, String, Option<String>),
        row_mt: Option<(Locus, bool)>,
        row_nd: NDArray<u32, 2usize>,
    }

    #[derive(Debug, Deserialize)]
    pub struct EntryAstruct {
        a: Option<u32>,
        b: f64,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryMstruct {
        x: u32,
        y: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Entry {
        entry_f_32: f32,
        entry_i_64: i64,
        entry_m: Option<f64>,
        entry_astruct: EntryAstruct,
        entry_mstruct: Option<EntryMstruct>,
        entry_aset: Vec<String>,
        entry_mset: Option<Vec<f64>>,
        entry_d: BTreeMap<Vec<Option<String>>, f64>,
        entry_md: Option<BTreeMap<u32, String>>,
        entry_h_38: Locus,
        entry_ml: Option<Locus>,
        entry_i: Interval<Locus>,
        entry_c: Call,
        entry_mc: Option<Call>,
        entry_t: (Call, String, Option<String>),
        entry_mt: Option<(Locus, bool)>,
        entry_nd: NDArray<u32, 2usize>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryRow {
        entries: Vec<Entry>,
    }

    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.5.0/matrix_table/0.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.5.0/matrix_table/1.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.5.0/matrix_table/2.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.5.0/matrix_table/3.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.5.0/matrix_table/4.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.5.0/matrix_table/5.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.5.0/matrix_table/6.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.5.0/matrix_table/7.hmt",
    )?;

    Ok(())
}

#[test]
fn matrix_table_compat_140() -> Result<()> {
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
    pub struct ColAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct ColMstruct {
        x: u32,
        y: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Column {
        col_idx: u32,
        col_f_32: f32,
        col_i_64: i64,
        col_m: Option<f64>,
        col_astruct: ColAstruct,
        col_mstruct: Option<ColMstruct>,
        col_aset: Vec<Option<String>>,
        col_mset: Option<Vec<f64>>,
        col_d: BTreeMap<Option<Vec<Option<String>>>, Option<f64>>,
        col_md: Option<BTreeMap<u32, String>>,
        col_h_38: Locus,
        col_ml: Option<Locus>,
        col_i: Interval<Option<Locus>>,
        col_c: Call,
        col_mc: Option<Call>,
        col_t: (Option<Call>, Option<String>, Option<String>),
        col_mt: Option<(Locus, bool)>,
    }

    #[derive(Debug, Deserialize)]
    pub struct RowAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct RowMstruct {
        x: u32,
        y: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        row_idx: u32,
        row_f_32: f32,
        row_i_64: i64,
        row_m: Option<f64>,
        row_astruct: RowAstruct,
        row_mstruct: Option<RowMstruct>,
        row_aset: Vec<Option<String>>,
        row_mset: Option<Vec<f64>>,
        row_d: BTreeMap<Option<Vec<Option<String>>>, Option<f64>>,
        row_md: Option<BTreeMap<u32, String>>,
        row_h_38: Locus,
        row_ml: Option<Locus>,
        row_i: Interval<Option<Locus>>,
        row_c: Call,
        row_mc: Option<Call>,
        row_t: (Option<Call>, Option<String>, Option<String>),
        row_mt: Option<(Locus, bool)>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryMstruct {
        x: u32,
        y: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Entry {
        entry_f_32: f32,
        entry_i_64: i64,
        entry_m: Option<f64>,
        entry_astruct: EntryAstruct,
        entry_mstruct: Option<EntryMstruct>,
        entry_aset: Vec<Option<String>>,
        entry_mset: Option<Vec<f64>>,
        entry_d: BTreeMap<Option<Vec<Option<String>>>, Option<f64>>,
        entry_md: Option<BTreeMap<u32, String>>,
        entry_h_38: Locus,
        entry_ml: Option<Locus>,
        entry_i: Interval<Option<Locus>>,
        entry_c: Call,
        entry_mc: Option<Call>,
        entry_t: (Option<Call>, Option<String>, Option<String>),
        entry_mt: Option<(Locus, bool)>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryRow {
        entries: Option<Vec<Entry>>,
    }

    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.4.0/matrix_table/0.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.4.0/matrix_table/1.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.4.0/matrix_table/2.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.4.0/matrix_table/3.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.4.0/matrix_table/4.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.4.0/matrix_table/5.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.4.0/matrix_table/6.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.4.0/matrix_table/7.hmt",
    )?;

    Ok(())
}

#[test]
fn matrix_table_compat_130() -> Result<()> {
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
    pub struct ColAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct ColMstruct {
        x: Option<u32>,
        y: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Column {
        col_idx: Option<u32>,
        col_f_32: Option<f32>,
        col_i_64: Option<i64>,
        col_m: Option<f64>,
        col_astruct: Option<ColAstruct>,
        col_mstruct: Option<ColMstruct>,
        col_aset: Option<Vec<Option<String>>>,
        col_mset: Option<Vec<Option<f64>>>,
        col_d: Option<BTreeMap<Option<Vec<Option<String>>>, Option<f64>>>,
        col_md: Option<BTreeMap<Option<u32>, Option<String>>>,
        col_h_38: Option<Locus>,
        col_ml: Option<Locus>,
        col_i: Option<Interval<Option<Locus>>>,
        col_c: Option<Call>,
        col_mc: Option<Call>,
        col_t: Option<(Option<Call>, Option<String>, Option<String>)>,
        col_mt: Option<(Option<Locus>, Option<bool>)>,
    }

    #[derive(Debug, Deserialize)]
    pub struct RowAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct RowMstruct {
        x: Option<u32>,
        y: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        row_idx: Option<u32>,
        row_f_32: Option<f32>,
        row_i_64: Option<i64>,
        row_m: Option<f64>,
        row_astruct: Option<RowAstruct>,
        row_mstruct: Option<RowMstruct>,
        row_aset: Option<Vec<Option<String>>>,
        row_mset: Option<Vec<Option<f64>>>,
        row_d: Option<BTreeMap<Option<Vec<Option<String>>>, Option<f64>>>,
        row_md: Option<BTreeMap<Option<u32>, Option<String>>>,
        row_h_38: Option<Locus>,
        row_ml: Option<Locus>,
        row_i: Option<Interval<Option<Locus>>>,
        row_c: Option<Call>,
        row_mc: Option<Call>,
        row_t: Option<(Option<Call>, Option<String>, Option<String>)>,
        row_mt: Option<(Option<Locus>, Option<bool>)>,
    }

    #[derive(Debug, Deserialize)]
    pub struct EntryAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryMstruct {
        x: Option<u32>,
        y: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Entry {
        entry_f_32: Option<f32>,
        entry_i_64: Option<i64>,
        entry_m: Option<f64>,
        entry_astruct: Option<EntryAstruct>,
        entry_mstruct: Option<EntryMstruct>,
        entry_aset: Option<Vec<Option<String>>>,
        entry_mset: Option<Vec<Option<f64>>>,
        entry_d: Option<BTreeMap<Option<Vec<Option<String>>>, Option<f64>>>,
        entry_md: Option<BTreeMap<Option<u32>, Option<String>>>,
        entry_h_38: Option<Locus>,
        entry_ml: Option<Locus>,
        entry_i: Option<Interval<Option<Locus>>>,
        entry_c: Option<Call>,
        entry_mc: Option<Call>,
        entry_t: Option<(Option<Call>, Option<String>, Option<String>)>,
        entry_mt: Option<(Option<Locus>, Option<bool>)>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryRow {
        entries: Option<Vec<Option<Entry>>>,
    }

    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.3.0/matrix_table/0.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.3.0/matrix_table/1.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.3.0/matrix_table/2.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.3.0/matrix_table/3.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.3.0/matrix_table/4.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.3.0/matrix_table/5.hmt",
    )?;

    Ok(())
}

#[test]
fn matrix_table_compat_120() -> Result<()> {
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
    pub struct ColAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct ColMstruct {
        x: Option<u32>,
        y: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Column {
        col_idx: Option<u32>,
        col_f_32: Option<f32>,
        col_i_64: Option<i64>,
        col_m: Option<f64>,
        col_astruct: Option<ColAstruct>,
        col_mstruct: Option<ColMstruct>,
        col_aset: Option<Vec<Option<String>>>,
        col_mset: Option<Vec<Option<f64>>>,
        col_d: Option<BTreeMap<Option<Vec<Option<String>>>, Option<f64>>>,
        col_md: Option<BTreeMap<Option<u32>, Option<String>>>,
        col_h_38: Option<Locus>,
        col_ml: Option<Locus>,
        col_i: Option<Interval<Option<Locus>>>,
        col_c: Option<Call>,
        col_mc: Option<Call>,
        col_t: Option<(Option<Call>, Option<String>, Option<String>)>,
        col_mt: Option<(Option<Locus>, Option<bool>)>,
    }
    #[derive(Debug, Deserialize)]
    pub struct RowAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct RowMstruct {
        x: Option<u32>,
        y: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Row {
        row_idx: Option<u32>,
        row_f_32: Option<f32>,
        row_i_64: Option<i64>,
        row_m: Option<f64>,
        row_astruct: Option<RowAstruct>,
        row_mstruct: Option<RowMstruct>,
        row_aset: Option<Vec<Option<String>>>,
        row_mset: Option<Vec<Option<f64>>>,
        row_d: Option<BTreeMap<Option<Vec<Option<String>>>, Option<f64>>>,
        row_md: Option<BTreeMap<Option<u32>, Option<String>>>,
        row_h_38: Option<Locus>,
        row_ml: Option<Locus>,
        row_i: Option<Interval<Option<Locus>>>,
        row_c: Option<Call>,
        row_mc: Option<Call>,
        row_t: Option<(Option<Call>, Option<String>, Option<String>)>,
        row_mt: Option<(Option<Locus>, Option<bool>)>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryAstruct {
        a: Option<u32>,
        b: Option<f64>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryMstruct {
        x: Option<u32>,
        y: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Entry {
        entry_f_32: Option<f32>,
        entry_i_64: Option<i64>,
        entry_m: Option<f64>,
        entry_astruct: Option<EntryAstruct>,
        entry_mstruct: Option<EntryMstruct>,
        entry_aset: Option<Vec<Option<String>>>,
        entry_mset: Option<Vec<Option<f64>>>,
        entry_d: Option<BTreeMap<Option<Vec<Option<String>>>, Option<f64>>>,
        entry_md: Option<BTreeMap<Option<u32>, Option<String>>>,
        entry_h_38: Option<Locus>,
        entry_ml: Option<Locus>,
        entry_i: Option<Interval<Option<Locus>>>,
        entry_c: Option<Call>,
        entry_mc: Option<Call>,
        entry_t: Option<(Option<Call>, Option<String>, Option<String>)>,
        entry_mt: Option<(Option<Locus>, Option<bool>)>,
    }
    #[derive(Debug, Deserialize)]
    pub struct EntryRow {
        entries: Option<Vec<Option<Entry>>>,
    }

    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.2.0/matrix_table/0.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.2.0/matrix_table/1.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.2.0/matrix_table/2.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.2.0/matrix_table/3.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.2.0/matrix_table/4.hmt",
    )?;
    parse_matrix::<Global, Column, Row, EntryRow>(
        "backward_compatability/1.2.0/matrix_table/5.hmt",
    )?;

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
