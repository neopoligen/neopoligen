use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn menu_page_charlie() -> Page {
        let source = r#"-- title

Menu Page: charlie

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_charlie 

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/l1c/charlie.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
