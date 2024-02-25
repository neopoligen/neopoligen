use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn site1_index() -> Page {
        let source_path =
            PathBuf::from("leading_folder/Neopoligen/dev-test-site/content/_index.neo");
        let source = r#"-- title

Site 1 Home Page

The initial test page

-- metadata
-- date: 2024-02-24 19:11:09
-- id: site1_index 
-- path: /
"#
        .to_string();

        let ast = "".to_string();
        Page {
            ast,
            source,
            source_path,
        }
    }
}
