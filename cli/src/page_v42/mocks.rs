use crate::{ast_v42::parse_ast_v42, site_config::SiteConfigV42};
use include_dir::{include_dir, Dir};
use std::path::PathBuf;

use super::PageV42;

impl PageV42 {
    pub fn valid_sample(name: &str) -> PageV42 {
        const SAMPLE_DIR: Dir = include_dir!("src/page_v42/samples/valid");
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file(name)
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();
        let ast = parse_ast_v42(&source, &site_config).unwrap();
        PageV42 { ast, source }
    }
}
