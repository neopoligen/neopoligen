#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod inits;
pub mod mocks;

use crate::neo_error::NeoError;
use crate::section_v42::SectionV42;
use crate::site_config::SiteConfigV42;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageV42 {
    pub raw_ast: Vec<SectionV42>,
    pub source: String,
    pub fs_path: PathBuf,
}

impl PageV42 {
    pub fn ast(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.raw_ast))
    }

    pub fn id(&self) -> Result<Value, Error> {
        Ok(Value::from("asdf"))
    }

    pub fn output_fs_rel_path(&self, base_path: &PathBuf) -> Result<PathBuf, NeoError> {
        Ok(PathBuf::from("index.html"))
    }

    pub fn template_list(&self) -> Vec<String> {
        vec!["page-templates/post/done.neoj".to_string()]
    }
}

impl Object for PageV42 {
    fn call_method(
        self: &Arc<PageV42>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "ast" => self.ast(),
            // "id" => self.id(),
            // "sections" => self.sections(),
            // "title" => self.title(),
            _ => Ok(Value::from("")),
        }
    }
}

#[cfg(test)]
mod test {

    use include_dir::{include_dir, Dir};

    use super::PageV42;
    // use super::*;
    // use pretty_assertions::assert_eq;
    //use rstest::rstest;

    // #[test]
    // pub fn solo_check_1() {
    //     let page = PageV42::valid_sample_mock("001-pagedata-only.neo");
    //     // dbg!(page.ast().unwrap());
    //     // dbg!(page);
    // }

    // #[test]
    // pub fn solo_check_2() {
    //     let page = PageV42::valid_sample_mock("002-basic-sample.neo");
    //     dbg!(page.ast().unwrap());
    //     // dbg!(page);
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
