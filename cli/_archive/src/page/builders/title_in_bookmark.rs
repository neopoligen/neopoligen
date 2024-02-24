use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn title_in_bookmark() -> Page {
        let source = r#"-- bookmark
-- title: This is the title from a bookmark

-- metadata
-- date: 2023-09-22 18:38:17
-- id: id662211
-- type: post
-- status: draft

"#;
        let config = Config::mock_basic_config();
        let page = Page::new(
            PathBuf::from("some-project-root/pages/here-file.neo"),
            source,
            config,
        );
        page
    }
}
