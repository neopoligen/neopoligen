use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn set1() -> FileSet {
        let mut fs = FileSet::new();
        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            "Templates are not used for these tests".to_string(),
        );

        fs.templates.insert(
            "custom/template/path.jinja".to_string(),
            "This is to check template paths to see if they exist".to_string(),
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
                "leading-dir/Neopoligen/set1-test-site/content/title-from-first-few-words-with-elipses.neo",
            ),
            r#"-- p

Title from the first few words of a section 
lorem ipsum verde AND THESE WORDS ARE NOT 
IN THE TITLE

-- metadata
-- id: ttss0050
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/title-from-first-few-words-that-does-not-get-truncated.neo",
            ),
            r#"-- p

First words short title

-- metadata
-- id: ttss0051
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/title-first-few-words-dont-add-extra-dot.neo",
            ),
            r#"-- p

First few words. There should only be three dots here not four. AND THESE WORDS SHOULD NOT SHOW UP

-- metadata
-- id: ttss0052
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
            PathBuf::from("leading-dir/Neopoligen/set1-test-site/content/stylesheets-test.neo"),
            r#"-- css

body { color: goldenrod; }

-- metadata
-- id: ttss0120
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set1-test-site/content/scripts-test.neo"),
            r#"-- script

console.log("ping")

-- metadata
-- id: ttss0130
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set1-test-site/content/head-test.neo"),
            r#"-- head

<!-- content for head -->

-- metadata
-- id: ttss0140
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/template-section-test.neo",
            ),
            r#"-- template

This is a template section test

-- metadata
-- id: ttss0150
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set1-test-site/content/script-with-attrs.neo"),
            r#"-- script
-- type: module

console.log("module")

-- metadata
-- id: ttss0160
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set1-test-site/content/type-to-template.neo"),
            r#"-- title

Testing type to template

-- metadata
-- id: ttss0170
-- type: example
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set1-test-site/content/base-template-default-post.neo",
            ),
            r#"-- title

Base template defaults to post

-- metadata
-- id: ttss0180
"#
            .to_string(),
        );

        /////////////////////////////////////////////////////////////////////////
        /// Final return of the file set
        fs
    }
}
