use crate::config::Config;
use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s2_title_from_content() -> Page {
        let config = Config::site1_config();
        let source_path =
            PathBuf::from("leading_folder/Neopoligen/dev-test-site/content/title_from_content.neo");
        let source = r#"-- bookmark
-- title: This Is A Title From A Bookmark Attribute

-- metadata
-- date: 2023-02-07 01:47:31
-- id: id_title_from_content
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
