use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn title_via_standard_section() -> Page {
        let source = r#"-- p

Title from first few words of a page
without any title in it at all

-- metadata
-- date: 2023-09-22 18:38:17
-- id: id662211
-- type: post
-- status: draft

"#;
        let config = Config::mock_basic_config();
        let page = Page::new(
            PathBuf::from("some-project-root/pages/example.neo"),
            source,
            config,
        );
        page
    }
}
