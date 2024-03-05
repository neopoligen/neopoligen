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
-- id: ttss0010
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
-- id: ttss0020
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

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-tree-1-site/content/title-from-any-section.neo",
            ),
            r#"-- p
-- title: Title From Any Section

-- metadata
-- id: aabb0040
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-tree-1-site/content/title-from-first-few-words.neo",
            ),
            r#"-- p

Title from the first few words of a section

-- metadata
-- id: aabb0050
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-tree-1-site/content/title-from-id-as-fallback.neo",
            ),
            r#"-- metadata
-- id: aabb0060
"#
            .to_string(),
        );

        fs
    }
}
