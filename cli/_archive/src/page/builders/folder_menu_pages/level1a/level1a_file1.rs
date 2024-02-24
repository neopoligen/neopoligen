use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

// The position in the directory tree doesn't
// have an effect with this load style. The
// PathBuf::from() must be defined properly

impl Page {
    pub fn level1a_file1() -> Page {
        let source = r#"-- title

level1a file1

-- metadata
-- date: 2021-07-02 12:18:47
-- id: level1a-file1

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/level1a/file1.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
