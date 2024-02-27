use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn set1() -> FileSet {
        let mut fs = FileSet::new();
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/dev-test-site-1/content/_index.neo"),
            r#"-- title

Dev Test Site 1 Home Page

-- metadata
-- date: 2024-02-26 18:48:19
-- id: id-site1-home-page 
-- path: /
"#
            .to_string(),
        );

        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            r#"This is a stub page"#.to_string(),
        );

        fs
    }
}
