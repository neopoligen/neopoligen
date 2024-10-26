#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod mocks;

use crate::neo_error::NeoError;
use crate::section_v42::SectionV42;
use crate::site_config::SiteConfigV42;
use minijinja::Error;
use minijinja::Value;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageV42 {
    pub raw_ast: Vec<SectionV42>,
    pub source: String,
}

impl PageV42 {
    pub fn ast(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.raw_ast))
    }
}

#[cfg(test)]
mod test {

    use include_dir::{include_dir, Dir};

    use super::PageV42;
    // use super::*;
    // use pretty_assertions::assert_eq;
    //use rstest::rstest;

    #[test]
    pub fn solo_check_1() {
        let page = PageV42::valid_sample_mock("001-pagedata-only.neo");
        // dbg!(page.ast().unwrap());
        // dbg!(page);
    }

    #[test]
    pub fn solo_check_2() {
        let page = PageV42::valid_sample_mock("002-basic-sample.neo");
        dbg!(page.ast().unwrap());
        // dbg!(page);
    }

    // fn valid_samples() -> BTreeMap<String, String> {
    //     let mut samples = BTreeMap::new();
    //     samples.insert(
    //         "000-pagedata-only".to_string(),
    //         include_str!("samples/valid/000-pagedata-only.neo").to_string(),
    //     );
    //     samples.insert(
    //         "001-basic-sample".to_string(),
    //         include_str!("samples/valid/001-basic-sample.neo").to_string(),
    //     );
    //     samples
    // }
}
