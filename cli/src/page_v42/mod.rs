#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::section_v42::SectionV42;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageV42 {
    pub ast: Option<Vec<SectionV42>>,
    pub raw_source: String,
    pub fs_path: PathBuf,
}

#[cfg(test)]
mod test {
    // use super::*;
    // use pretty_assertions::assert_eq;
    //use rstest::rstest;
    use std::collections::BTreeMap;

    // #[test]
    // fn solo_validate_samples() {
    //     valid_samples().iter().for_each(|(_key, source)| {
    //         dbg!(source);
    //     });
    // }

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
