use crate::site_config::SiteConfig;
use std::path::PathBuf;

use super::PageV41;

impl PageV41 {
    pub fn mock1_basic_page() -> PageV41 {
        let site_config = SiteConfig::mock1_basic();
        let mut page = PageV41 {
            ast: None,
            raw_source: r#"-- title

Mock 1 Basic Page

-- pagedata
-- id: mock1_basic_page
-- status: published"#
                .to_string(),
            fs_path: PathBuf::from("not/a/real/path.neo"),
        };
        let _ = page.generate_ast(&site_config);
        page
    }
}
