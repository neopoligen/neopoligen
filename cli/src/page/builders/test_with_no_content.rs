use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn test_with_no_content() -> Page {
        Page {
            site: None,
            ast: vec![],
            config: Config::mock_basic_config(),
            source_path: PathBuf::from("some-project-root/pages/alfa/alfa.neo"),
            source: "".to_string(),
        }
    }
}
