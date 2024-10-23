use crate::site_config::SiteConfigV42;
use std::path::PathBuf;

use super::PageV42;

impl PageV42 {
    pub fn mock1_basic_page() -> PageV42 {
        let site_config = SiteConfigV42::mock1_basic();
        let mut page = PageV42 {
            ast: None,
            raw_source: r#"-- title

Mock 1 Basic Page

-- pagedata
-- id: mock1_basic_page
-- status: scratch 
"#
            .to_string(),
            fs_path: PathBuf::from("not/a/real/path.neo"),
        };
        page.generate_ast(&site_config).unwrap();
        page
    }

    pub fn mock2_missing_id_and_status() -> PageV42 {
        let site_config = SiteConfigV42::mock1_basic();
        let mut page = PageV42 {
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

    pub fn mock3_custom_path() -> PageV42 {
        let site_config = SiteConfigV42::mock1_basic();
        let mut page = PageV42 {
            ast: None,
            raw_source: r#"-- title

Mock 3

-- pagedata
-- id: mock3_test
-- template: home-page
-- status: draft
-- path: /index.html"#
                .to_string(),
            fs_path: PathBuf::from("not/a/real/path.neo"),
        };
        page.generate_ast(&site_config).unwrap();
        page
    }

    //
}
