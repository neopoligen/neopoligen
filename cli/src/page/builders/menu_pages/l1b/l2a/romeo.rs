use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
	pub fn menu_page_romeo() -> Page {
		let source = r#"-- title

Menu Page: romeo

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_romeo 

"#;
		Page::new(
			PathBuf::from("some-project-root/pages/l1b/l2a/romeo.neo"),
			source,
			Config::mock_basic_config(),
		)
	}
}