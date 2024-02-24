use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn menu_page_xray() -> Page {
        let source = r#"-- title

Menu Page: xray

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_xray 

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/jump_up/xray.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
