use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
	pub fn menu_page_oscar() -> Page {
		let source = r#"-- title

Menu Page: oscar

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_oscar 

"#;
		Page::new(
			PathBuf::from("some-project-root/pages/l1a/l2b/oscar.neo"),
			source,
			Config::mock_basic_config(),
		)
	}
}
