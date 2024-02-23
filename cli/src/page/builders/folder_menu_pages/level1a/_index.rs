use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

// The position in the directory tree doesn't
// have an effect with this load style. The
// PathBuf::from() must be defined properly

impl Page {
    pub fn level1a_index() -> Page {
        let source = r#"-- title

level1a index

-- metadata
-- date: 2021-07-02 12:18:47
-- id: level1a-index

"#;
        Page::new(
            PathBuf::from("some-project-root/pages/level1a/_index.neo"),
            source,
            Config::mock_basic_config(),
        )
    }
}
