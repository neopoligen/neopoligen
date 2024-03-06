use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn nav_items2() -> FileSet {
        let mut fs = FileSet::new();
        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            r#"This is a stub page"#.to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-items2-test-site/content/top-level-page.neo"),
            r#"-- title

Top Level Page

-- metadata
-- id: aabb0010
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items2-test-site/content/level-1a/_title.neo",
            ),
            r#"-- title

Level 1a Title

-- metadata
-- id: aabb0020
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items2-test-site/content/level-1a/content-alfa.neo",
            ),
            r#"-- title

Level 1a Content Alfa

-- metadata
-- id: aabb0030
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items2-test-site/content/level-1a/sub-level-2a/_title.neo",
            ),
            r#"-- title

Level 1a SubLevel 2a Title

-- metadata
-- id: aabb0040
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items2-test-site/content/level-1a/sub-level-2a/content-bravo.neo",
            ),
            r#"-- title

Level 1a SubLevel 2a Content Bravo

-- metadata
-- id: aabb0050 
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items2-test-site/content/level-1b/_index.neo",
            ),
            r#"-- title

Level 1b Index

-- metadata
-- id: aabb0060
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items2-test-site/content/level-1b/content-charlie.neo",
            ),
            r#"-- title

Level 1b Content Charlie

-- metadata
-- id: aabb0070
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items2-test-site/content/level-1b/sub-level-2b/_index.neo",
            ),
            r#"-- title

Level 1b SubLevel 2b Index 

-- metadata
-- id: aabb0080
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items2-test-site/content/level-1b/sub-level-2b/under-index-level-2-content.neo",
            ),
            r#"-- title

Content Under Section Index Level

-- metadata
-- id: aabb0090
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items2-test-site/content/default-sort-test/_index.neo",
            ),
            r#"-- title

Default Sort Test

-- metadata
-- id: aabb0100
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-items2-test-site/content/default-sort-test/delta-file.neo"),
            r#"-- title

Default Sort Delta File

-- metadata
-- id: aabb0110
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-items2-test-site/content/default-sort-test/alfa-file.neo"),
            r#"-- title

Default Sort Alfa File

-- metadata
-- id: aabb0120
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-items2-test-site/content/default-sort-test/charlie-title-folder/_title.neo"),
            r#"-- title

Default Sort Charlie Title Folder

-- metadata
-- id: aabb0130
"#
            .to_string(),
        );

        fs
    }
}
