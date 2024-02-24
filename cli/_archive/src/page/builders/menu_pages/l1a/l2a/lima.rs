use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn menu_page_lima() -> Page {
        let source = r#"-- title

Menu Page: lima

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_lima 

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/l1a/l2a/lima.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
