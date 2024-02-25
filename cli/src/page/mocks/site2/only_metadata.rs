use crate::config::Config;
use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s2_only_metadata() -> Page {
        let config = Config::site1_config();
        let source_path =
            PathBuf::from("leading_folder/Neopoligen/test-site2/content/only_metadata.neo");
        let source = r#"-- metadata
-- date: 2022-12-01 14:31:29
-- id: id_only_metadata 
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
