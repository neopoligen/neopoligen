use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn nav_items1() -> FileSet {
        let mut fs = FileSet::new();
        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            r#"Templates are not used for these tests"#.to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-items1-site/content/folder1/_index.neo"),
            r#"-- title

Folder1 Index

-- metadata
-- id: folder1-index 
"#
            .to_string(),
        );

        fs
    }
}
