use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn set1() -> FileSet {
        let mut fs = FileSet::new();
        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            "Templates are not used for these tests".to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-tree-1-site/content/home-page.neo"),
            r#"-- title

Home Page

-- metadata
-- id: aabb0010
-- path: /
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-tree-1-site/content/title-from-title-section.neo",
            ),
            r#"-- title

Title From Title Section

-- metadata
-- id: aabb0020
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-tree-1-site/content/title-from-metadata.neo"),
            r#"-- title

This Title Is Overwritten By Metadata

-- metadata
-- id: aabb0030
-- title: Title From Metadata 
"#
            .to_string(),
        );

        fs
    }
}
