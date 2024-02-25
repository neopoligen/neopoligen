use crate::config::Config;
use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s2_index() -> Page {
        let config = Config::site2_config();
        let source_path =
            PathBuf::from("leading_folder/Neopoligen/dev-test-site/content/_index.neo");
        let source = r#"-- title

Site 2 Home Page

The initial test page

-- metadata
-- date: 2020-01-14 13:13:36
-- id: id_index
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
