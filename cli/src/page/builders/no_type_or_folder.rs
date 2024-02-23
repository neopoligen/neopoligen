use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn no_type_or_folder() -> Page {
        let source = r#"-- title

Span <<strong|Nested <<strong|In>>>> Title

-- metadata
-- date: 2022-09-12 02:23:11
-- id: id003344
-- status: draft

"#;
        let config = Config::mock_basic_config();
        let page = Page::new(
            PathBuf::from("some-project-root/pages/index.neo"),
            source,
            config,
        );
        page
    }
}
