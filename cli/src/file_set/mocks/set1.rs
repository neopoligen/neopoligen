use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn set1() -> FileSet {
        let mut fs = FileSet::new();
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-tree-1-site/content/_index.neo"),
            r#"-- title

Home Page

-- metadata
-- id: nav-tree-1-home-page 
-- path: /
"#
            .to_string(),
        );

        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            "Templates are not used for these tests".to_string(),
        );

        fs
    }
}
