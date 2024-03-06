use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn set2() -> FileSet {
        let mut fs = FileSet::new();

        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            r#"This is a stub page"#.to_string(),
        );

        fs.templates.insert(
            "pages/custom-template-type/published.jinja".to_string(),
            r#"This is a stub page"#.to_string(),
        );

        fs.templates.insert(
            "pages/post/custom-template-status.jinja".to_string(),
            r#"This is a stub page"#.to_string(),
        );

        fs.templates.insert(
            "pages/custom-template-type/custom-template-status.jinja".to_string(),
            r#"This is a stub page"#.to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set2-test-site/content/_index.neo"),
            r#"-- title

Dev Test Site 2 Home Page

-- metadata
-- date: 2000-01-01 00:00:00
-- id: site2-home-page
-- path: /
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/title-from-section-attribute.neo",
            ),
            r#"-- bookmark
-- title: This Is A Title From A Bookmark Attribute
-- url: https://www.example.com/

-- metadata
-- date: 2000-01-02 00:00:00
-- id: title-from-section-attribute
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/title-from-block-content.neo",
            ),
            r#"-- p

Title from block content example

-- metadata
-- date: 2000-01-03 00:00:00
-- id: title-from-block-content
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/title-from-metadata-id.neo",
            ),
            r#"-- metadata
-- date: 2000-01-04 00:00:00
-- id: no-title-just-id
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/metadata-override-title.neo",
            ),
            r#"-- title

This Title Should Be Overridden By Metadata

-- metadata
-- title: Metadata Override Title
-- date: 2000-01-05 00:00:00
-- id: metadata-override-title
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/title-with-inline-span.neo",
            ),
            r#"-- title

Inline <<em|Spans>> Should <<strong|Not>> Show UP

-- metadata
-- date: 2000-01-06 00:00:00
-- id: title-with-inline-span
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/title-with-nested-inline-spans.neo",
            ),
            r#"-- title

Nested <<em|<<strong|Inline Spans>>>> Should Not Show UP

-- metadata
-- date: 2000-01-07 00:00:00
-- id: title-with-nested-inline-spans
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/page-type-in-metadata.neo",
            ),
            r#"-- metadata
-- date: 2000-01-08 00:00:00
-- id: page-type-in-metadata
-- type: example
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/page-type-not-in-metadata.neo",
            ),
            r#"-- metadata
-- date: 2000-01-09 00:00:00
-- id: page-type-not-in-metadata
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/page-status-in-metadata.neo",
            ),
            r#"-- metadata
-- date: 2000-01-10 00:00:00
-- id: page-status-in-metadata
-- status: example_status
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/page-status-not-in-metadata.neo",
            ),
            r#"-- metadata
-- date: 2000-01-11 00:00:00
-- id: page-status-not-in-metadata
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/page-without-override-path.neo",
            ),
            r#"-- metadata
-- date: 2000-01-12 00:00:00
-- id: page-without-override-path
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/url-escape-title-check.neo",
            ),
            r#"-- title

URL Escape / Title Check

-- metadata
-- id: url-escape-title-check
-- date: 2000-01-13 00:00:00

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set2-test-site/content/default-template.neo"),
            r#"-- title

This has no type or status metadata so will show the default template

-- metadata
-- id: default-template
-- date: 2000-01-14 00:00:00

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set2-test-site/content/custom-template-type.neo"),
            r#"-- title

This is a custom template type that exists

-- metadata
-- id: custom-template-type
-- date: 2000-01-15 00:00:00
-- type: custom-template-type

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/custom-template-status.neo",
            ),
            r#"-- title

This is a custom template status that exists

-- metadata
-- id: custom-template-status
-- date: 2000-01-16 00:00:00
-- status: custom-template-status

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/type-for-non-existent-template.neo",
            ),
            r#"-- title

Confirm if a custom type doesn't have a 
template it rolls back to the default

-- metadata
-- id: type-for-non-existent-template
-- date: 2000-01-17 00:00:00
-- type: type-for-non-existent-template

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set2-test-site/content/status-for-non-existent-template.neo"),
            r#"-- title

Confirm if a custom stauts doesn't have a 
template it rolls back to the default

-- metadata
-- id: status-for-non-existent-template
-- date: 2000-01-18 00:00:00
-- status: status-for-non-existent-template

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set2-test-site/content/basic-main-body-test.neo"),
            r#"-- title

Basic main_body Test

-- p

Test main_body output

-- metadata
-- id: basic-main-body-test
-- date: 2000-01-19 00:00:00

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/basic-place-section-test.neo",
            ),
            r#"-- title

Basic Place Section Test

-- metadata
-- id: basic-place-section-test
-- date: 2000-01-20 00:00:00

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set2-test-site/content/source-path-check.neo"),
            r#"-- title

This is a source path check

-- metadata
-- id: source-path-check
-- date: 2000-01-21 00:00:00

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/path/parts/example/subfile.neo",
            ),
            r#"-- title

This checks path parts

-- metadata
-- id: page-parts-example
-- date: 2000-01-22 00:00:00

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/page/folders/example/file.neo",
            ),
            r#"-- title

Check Page Folders

-- metadata
-- id: page-folders-example
-- date: 2000-01-23 00:00:00

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set2-test-site/content/link-or-title-start.neo"),
            r#"-- title

Link Or Title Start

-- metadata
-- id: link-or-title-start
-- date: 2000-01-24 00:00:00

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set2-test-site/content/link-or-title-target.neo"),
            r#"-- title

Link Or Title Target

-- metadata
-- id: link-or-title-target
-- date: 2000-01-25 00:00:00

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/menu/folder/open/switch.neo",
            ),
            r#"-- title

Test For Open Menu Folder

-- metadata
-- date: 2000-01-26 00:00:00
-- id: menu-folder-open-switch

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/set2-test-site/content/menu/folder/closed/switch.neo",
            ),
            r#"-- title

Test For Closed Menu Folder

-- metadata
-- date: 2000-01-26 00:00:00
-- id: menu-folder-closed-switch

"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/set2-test-site/content/top-level-file.neo"),
            r#"-- title

Top Level File

-- metadata
-- date: 2000-01-27 00:00:00
-- id: top-level-file

"#
            .to_string(),
        );

        fs
    }
}
