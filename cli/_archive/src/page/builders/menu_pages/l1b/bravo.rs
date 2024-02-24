use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn menu_page_bravo() -> Page {
        let source = r#"-- title

Menu Page: bravo

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_bravo 

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/l1b/bravo.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
