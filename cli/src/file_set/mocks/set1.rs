use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn set1() -> FileSet {
        let mut fs = FileSet::new();
        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            "Templates are not used for these tests".to_string(),
        );

        fs.images.push(PathBuf::from(
            "leading-dir/Neopoligen/set1-test-site/images/root-level-image.png",
        ));

        fs.images.push(PathBuf::from(
            "leading-dir/Neopoligen/set1-test-site/images/sub-folder/sub-folder-image.png",
        ));

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set1-test-site/content/home-page.neo"),
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
                "leading-dir/Neopoligen/set1-test-site/content/title-from-title-section.neo",
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
            PathBuf::from("leading-dir/Neopoligen/set1-test-site/content/title-from-metadata.neo"),
            r#"-- title

This Title Is Overwritten By Metadata

-- metadata
-- id: ttss0030
-- title: Title From Metadata 
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/title-from-any-section.neo",
            ),
            r#"-- p
-- title: Title From Any Section

-- metadata
-- id: ttss0040
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/title-from-first-few-words.neo",
            ),
            r#"-- p

Title from the first few words of a section

-- metadata
-- id: ttss0050
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/title-from-id-as-fallback.neo",
            ),
            r#"-- metadata
-- id: ttss0060
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/tag-from-folder/tag-check-default-type-and-status.neo",
            ),
            r#"-- tags
-- tag-from-tags-section

-- metadata
-- id: ttss0070
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/tag-from-folder/tag-from-custom-type-and-status.neo",
            ),
            r#"-- metadata
-- id: ttss0080
-- type: tag-from-type
-- status: tag-from-status
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/custom-type-and-status.neo",
            ),
            r#"-- metadata
-- id: ttss0090
-- type: custom-page-type
-- status: custom-page-status 
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/defatul-type-status-and-template-not-in-folder.neo",
            ),
            r#"-- metadata
-- id: ttss0100
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/type-from-first-folder/test.neo",
            ),
            r#"-- metadata
-- id: ttss0110
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set1-test-site/content/css-for-head.neo"),
            r#"-- css

body {
    color: goldenrod;
}

-- metadata
-- id: ttss0110
"#
            .to_string(),
        );

        fs
    }
}
