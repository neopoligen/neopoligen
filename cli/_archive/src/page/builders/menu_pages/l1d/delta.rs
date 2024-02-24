use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn menu_page_delta() -> Page {
        let source = r#"-- title

Menu Page: delta

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_delta 

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/l1d/delta.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
