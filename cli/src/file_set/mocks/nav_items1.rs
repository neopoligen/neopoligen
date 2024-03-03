use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn nav_items1() -> FileSet {
        let mut fs = FileSet::new();
        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            r#"This is a stub page"#.to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-items1-site/content/top-level-page.neo"),
            r#"-- title

Top Level Page

-- metadata
-- id: top-level-page
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-items1-site/content/level-1a/_title.neo"),
            r#"-- title

Level 1a Title

-- metadata
-- id: level-1a-title
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items1-site/content/level-1a/content-alfa.neo",
            ),
            r#"-- title

Level 1a Content Alfa

-- metadata
-- id: level-1a-content-alfa
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items1-site/content/level-1a/sub-level-2a/_title.neo",
            ),
            r#"-- title

Level 1a SubLevel 2a Title

-- metadata
-- id: level-1a-sub-level-2a-title
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items1-site/content/level-1a/sub-level-2a/content-bravo.neo",
            ),
            r#"-- title

Level 1a SubLevel 2a Content Bravo

-- metadata
-- id: level-1a-sub-level-2a-content-bravo
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-items1-site/content/level-1b/_index.neo"),
            r#"-- title

Level 1b Index

-- metadata
-- id: level-1b-index
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-items1-site/content/level-1b/content-charlie.neo",
            ),
            r#"-- title

Level 1b Content Charlie

-- metadata
-- id: level-1b-content-charlie
"#
            .to_string(),
        );

        fs
    }
}
