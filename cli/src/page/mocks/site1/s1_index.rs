use crate::config::Config;
use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s1_index() -> Page {
        let config = Config::site1_config();
        let source_path =
            PathBuf::from("leading_folder/Neopoligen/dev-test-site/content/_index.neo");
        let source = r#"-- title

Site 1 Home Page

The initial test page

-- metadata
-- date: 2024-02-24 19:11:09
-- id: s1_index
-- path: /
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
