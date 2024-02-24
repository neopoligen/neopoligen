use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

// This one is to test what happens if the
// only thing available for the title is an
// id in the metadata

impl Page {
    pub fn no_title_or_standard_sections() -> Page {
        let source = r#"-- metadata
-- date: 2023-09-22 18:38:17
-- id: id996622
-- type: post
-- status: draft

"#;
        let config = Config::mock_basic_config();
        let page = Page::new(
            PathBuf::from("some-project-root/pages/things.neo"),
            source,
            config,
        );
        page
    }
}
