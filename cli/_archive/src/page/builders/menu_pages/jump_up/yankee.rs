use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn menu_page_yankee() -> Page {
        let source = r#"-- title

Menu Page: yankee

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_yankee 

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/jump_up/yankee.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
