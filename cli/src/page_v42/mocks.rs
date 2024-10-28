use super::PageV42;
use crate::helpers::get_include_str_file;
use crate::{ast_v42::parse_ast_v42, site_config::SiteConfigV42};
use include_dir::{include_dir, Dir};
use std::path::PathBuf;

impl PageV42 {
    pub fn valid_sample_mock(name: &str) -> PageV42 {
        const SAMPLES: Dir = include_dir!("src/page_v42/samples/valid");
        let site_config = SiteConfigV42::mock1_basic();
        let source = get_include_str_file(SAMPLES, name);
        let raw_ast = parse_ast_v42(&source, &site_config).unwrap();
        PageV42 {
            id: Some("TODO_MAKE_MOCK_ID_HERE".to_string()),
            error: None,
            html_output_path: None,
            raw_ast,
            source,
            fs_path: PathBuf::from("mock/path/file.neo"),
        }
    }
}
