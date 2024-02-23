use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
	pub fn menu_page_victor() -> Page {
		let source = r#"-- title

Menu Page: victor

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_victor 

"#;
		Page::new(
			PathBuf::from("some-project-root/pages/l1a/l21/l3a/victor.neo"),
			source,
			Config::mock_basic_config(),
		)
	}
}
