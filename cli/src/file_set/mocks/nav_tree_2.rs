use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn nav_tree_2() -> FileSet {
        let mut fs = FileSet::new();
        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            r#"Templates are not used for these tests"#.to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-tree-2-site/content/current-file-target.neo"),
            r#"-- title

Current File Target

-- metadata
-- id:  current-file-target
"#
            .to_string(),
        );

        fs
    }
}
