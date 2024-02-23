use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
	pub fn menu_page_papa() -> Page {
		let source = r#"-- title

Menu Page: papa

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_papa 

"#;
		Page::new(
			PathBuf::from("some-project-root/pages/l1a/l2b/papa.neo"),
			source,
			Config::mock_basic_config(),
		)
	}
}
