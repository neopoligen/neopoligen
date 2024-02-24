use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn menu_page_echo() -> Page {
        let source = r#"-- title

Menu Page: echo

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_echo 
-- child-folder: ["l1a", "l2a"]

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/l1a/echo.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
