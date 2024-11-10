use super::PageV42b;
use crate::helpers::get_include_str_file;
use crate::site_config::SiteConfigV42;
use include_dir::{include_dir, Dir};
use std::path::PathBuf;

const DATA_SAMPLES: Dir = include_dir!("tests/page_v42_data_tests/valid/input");
const SECTION_SAMPLES: Dir = include_dir!("tests/page_v42_section_tests/valid/input");

impl PageV42b {
    pub fn mock_from_data_tests(file: &str) -> PageV42b {
        let source = get_include_str_file(DATA_SAMPLES, file);
        let site_config = SiteConfigV42::mock1_basic();
        let source_path =
            PathBuf::from(format!("{}/{}", &site_config.project_dir().display(), file));
        let mut page =
            PageV42b::new_from_str_v42(&source_path, &source, &site_config.project_dir());
        page.set_language_from_site_config(&site_config);
        page.generate_ast(&site_config);
        page.set_id();
        page.set_rel_output_path();
        page.set_titles();
        page
    }

    pub fn mock_from_section_tests(file: &str) -> PageV42b {
        let source = get_include_str_file(SECTION_SAMPLES, &format!("{}.neo", file));
        let site_config = SiteConfigV42::mock1_basic();
        let source_path =
            PathBuf::from(format!("{}/{}", &site_config.project_dir().display(), file));
        let mut page =
            PageV42b::new_from_str_v42(&source_path, &source, &site_config.project_dir());
        page.set_language_from_site_config(&site_config);
        page.generate_ast(&site_config);
        page.set_id();
        page.set_rel_output_path();
        page
    }
}
