use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn set2() -> FileSet {
        let mut fs = FileSet::new();



        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            r#"This is a stub page"#.to_string(),
        );


        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/dev-test-site-2/content/_index.neo"),
            r#"-- title

Dev Test Site 2 Home Page

-- metadata
-- date: 2024-01-01 00:00:00
-- id: site2-home-page
-- path: /
"#
            .to_string(),
        );


        
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/dev-test-site-2/content/title-from-section-attribute.neo"),
            r#"-- bookmark
-- title: This Is A Title From A Bookmark Attribute
-- url: https://www.example.com/

-- metadata
-- date: 2024-01-02 00:00:00
-- id: title-from-section-attribute
"#
            .to_string(),
        );


        
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/dev-test-site-2/content/title-from-block-content.neo"),
            r#"-- p

Title from block content example

-- metadata
-- date: 2024-01-03 00:00:00
-- id: title-from-block-content
"#
            .to_string(),
        );

        

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/dev-test-site-2/content/title-from-metadata-id.neo"),
            r#"-- metadata
-- date: 2024-01-03 00:00:00
-- id: no-title-just-id
"#
            .to_string(),
        );


        fs
    }
}
