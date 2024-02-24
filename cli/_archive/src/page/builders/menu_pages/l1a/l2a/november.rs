use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn menu_page_november() -> Page {
        let source = r#"-- title

Menu Page: november

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_november 

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/l1a/l2a/november.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
