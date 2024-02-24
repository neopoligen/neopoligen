use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn title_in_metadata() -> Page {
        let source = r#"-- title

This should not be the title. It should
be overwritten by the metadata title

-- metadata
-- title: This should be the title. It's from the metadata
-- date: 2021-07-02 12:18:47
-- id: id441122
-- type: post
-- status: draft

"#;
        let config = Config::mock_basic_config();
        let page = Page::new(
            PathBuf::from("some-project-root/pages/again.neo"),
            source,
            config,
        );
        page
    }
}
