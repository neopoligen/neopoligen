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
-- date: 2024-01-02 00:00:00
-- id: site2-home-page
-- path: /
"#
            .to_string(),
        );


        
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/dev-test-site-2/content/title-from-content.neo"),
            r#"-- bookmark
-- title: This Is A Title From A Bookmark Attribute
-- url: https://www.example.com/

-- metadata
-- date: 2024-01-01 00:00:00
-- id: title-from-content
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
