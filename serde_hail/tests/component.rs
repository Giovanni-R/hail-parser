#![allow(dead_code)]
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Deserialize};

use serde_hail::types::{Call, Locus};

fn parse_component<T>(file: &str) -> Result<()>
where
    T: DeserializeOwned,
{
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../resources")
        .join(file);
    let _ = serde_hail::load::component::<T, &PathBuf>(&path)
        .context(format!("Failed to load component in path: {:?}", path))?;
    Ok(())
}

#[test]
fn component_sample_rows() -> Result<()> {
    #[derive(Deserialize)]
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
    #[derive(Deserialize)]
    pub struct Component {
        locus: Option<Locus>,
        alleles: Option<Vec<Option<String>>>,
        rsid: Option<String>,
        qual: Option<f64>,
        filters: Option<Vec<Option<String>>>,
        info: Option<Info>,
    }
    parse_component::<Component>("sample.vcf.mt/rows/rows")
}

#[test]
fn component_sample_columns() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Component {
        s: Option<String>,
    }
    parse_component::<Component>("sample.vcf.mt/cols/rows")
}

#[test]
fn component_sample_entries() -> Result<()> {
    #[derive(Debug, Deserialize)]
    pub struct Entry {
        gt: Option<Call>,
        ad: Option<Vec<u32>>,
        dp: Option<u32>,
        gq: Option<u32>,
        pl: Option<Vec<u32>>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Component {
        entries: Option<Vec<Option<Entry>>>,
    }
    parse_component::<Component>("sample.vcf.mt/entries/rows")
}
