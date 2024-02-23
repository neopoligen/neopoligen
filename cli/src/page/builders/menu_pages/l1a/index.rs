use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn menu_page_l1a_index() -> Page {
        let source = r#"-- title

Menu Page: alfa

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_l1a_index

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/l1a/index.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
