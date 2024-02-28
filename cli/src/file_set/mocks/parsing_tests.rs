use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn parsing_tests() -> FileSet {
        let mut fs = FileSet::new();
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/parsing-tests/content/code-section-test.neo"),
            r#"-- title

Allow Flag Attributes In link Spans

-- h2

Overview


-- todo
-- title: Details

[] Make this not break the attributes in
the output

-- code

<<link|Lorem ipsum|https://www.example.com/|class: green|example_flag>>




-- tags


-- metadata
-- date: 2024-02-03 17:04:07
-- updated: 2024-02-03 17:04:07
-- id: code-section-test
-- status: scratch
                        
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
