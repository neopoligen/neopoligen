use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn title_with_nested_inline_spans() -> Page {
        let source = r#"-- title

Span <<strong|Nested <<strong|In>>>> Title

-- metadata
-- date: 2021-07-02 12:18:47
-- id: id441122
-- type: post
-- status: draft

"#;
        let config = Config::mock_basic_config();
        let page = Page::new(
            PathBuf::from("some-project-root/pages/bravo.neo"),
            source,
            config,
        );
        page
    }
}