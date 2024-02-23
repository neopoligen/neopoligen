use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
	pub fn menu_page_india() -> Page {
		let source = r#"-- title

Menu Page: india

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_india 

"#;
		Page::new(
			PathBuf::from("some-project-root/pages/l1b/india.neo"),
			source,
			Config::mock_basic_config(),
		)
	}
}
