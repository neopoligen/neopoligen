use crate::config::Config;
// use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s1_index() -> Page {
        let config = Config::site1_config();
        let source_path = PathBuf::from("leading_folder/Neopoligen/test-site1/content/_index.neo");
        let source = r#"-- title

Integration Test Site

This is the integration test site

-- metadata
-- date: 2024-01-02 03:04:05
-- id: id_index
-- path: /
"#
        .to_string();
        Page::new(source_path, source, &config).unwrap()
    }
}