use crate::config::Config;
use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s1_title_from_text() -> Page {
        let config = Config::site1_config();
        let source_path =
            PathBuf::from("leading_folder/Neopoligen/dev-test-site/content/s1_title_from_text.neo");
        let source = r#"-- p

This is a title from the first few words 
of text of a paragraph section of a page
that doesn't have a title section or
metadata

-- metadata
-- date: 2024-02-24 19:11:09
-- id: s1_title_from_text
"#
        .to_string();
        let ast = parse(&source, &config);
        Page {
            ast,
            source,
            source_path,
        }
    }
}
