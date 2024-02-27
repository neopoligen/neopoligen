use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn set2() -> FileSet {
        let mut fs = FileSet::new();
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/dev-test-site-2/content/_index.neo"),
            r#"-- title

Dev Test Site 2 Home Page

-- metadata
-- date: 2024-02-26 20:46:34
-- id: id-site2-home-page
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
