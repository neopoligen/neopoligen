use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn nav_prev_next1() -> FileSet {
        let mut fs = FileSet::new();
        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            r#"Templates are not used for these tests"#.to_string(),
        );

        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/nav-prev_next1-site/content/level-1a/_title.neo"),
            r#"-- title

Level 1a Title

-- metadata
-- id: level-1a-title
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-prev_next1-site/content/level-1a/content-alfa.neo",
            ),
            r#"-- title

Content Alfa

-- metadata
-- id: content-alfa 
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-prev_next1-site/content/level-1a/content-bravo.neo",
            ),
            r#"-- title

Content Bravo 

-- metadata
-- id: content-bravo
"#
            .to_string(),
        );

        fs.pages.insert(
            PathBuf::from(
                "leading-dir/Neopoligen/nav-prev_next1-site/content/level-1a/content-charlie.neo",
            ),
            r#"-- title

Content Charlie 

-- metadata
-- id: content-charlie
"#
            .to_string(),
        );

        fs
    }
}
