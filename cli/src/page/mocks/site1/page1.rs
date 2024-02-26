use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s1_page_alfa() -> Page {
        let config = Config::site1_config();
        let source_path =
            PathBuf::from("leading_folder/Neopoligen/test-site1/content/page-alfa.neo");
        let source = r#"-- title

This Is Page/File Alfa

-- metadata
-- date: 2021-09-23 18:45:57
-- id: page-alfa 
-- path: /
"#
        .to_string();
        Page::new(source_path, source, &config).unwrap()
    }
}
