use crate::site_config::SiteConfigV41;
use std::path::PathBuf;

use super::PageV41;

impl PageV41 {
    pub fn mock1_basic_page() -> PageV41 {
        let site_config = SiteConfigV41::mock1_basic();
        let mut page = PageV41 {
            ast: None,
            raw_source: r#"-- title

Mock 1 Basic Page

-- pagedata
-- id: mock1_basic_page
-- status: published "#
                .to_string(),
            fs_path: PathBuf::from("not/a/real/path.neo"),
        };
        page.generate_ast(&site_config).unwrap();
        page
    }

    pub fn mock2_missing_id_and_status() -> PageV41 {
        let site_config = SiteConfigV41::mock1_basic();
        let mut page = PageV41 {
            ast: None,
            raw_source: r#"-- title

Mock 2 Missing pagedata id and status

-- pagedata"#
                .to_string(),
            fs_path: PathBuf::from("not/a/real/path.neo"),
        };
        page.generate_ast(&site_config).unwrap();
        page
    }

    pub fn mock3_custom_path() -> PageV41 {
        let site_config = SiteConfigV41::mock1_basic();
        let mut page = PageV41 {
            ast: None,
            raw_source: r#"-- title

Mock 2 Missing pagedata id and status

-- pagedata
-- id: mock3_custom_path
-- path: /index.html"#
                .to_string(),
            fs_path: PathBuf::from("not/a/real/path.neo"),
        };
        page.generate_ast(&site_config).unwrap();
        page
    }

    //
}
