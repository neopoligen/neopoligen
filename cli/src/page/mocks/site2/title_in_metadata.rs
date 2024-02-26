use crate::config::Config;
use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s2_title_in_metadata() -> Page {
        let config = Config::site2_config();
        let source_path =
            PathBuf::from("leading_folder/Neopoligen/test-site2/content/title_in_metadata.neo");
        let source = r#"-- title

This title should be overridded by metadata

-- metadata
-- date: 2024-02-24 19:11:09
-- id: id_title_in_metadata
-- title: This is the override title from metadata
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
