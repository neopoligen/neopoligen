use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
	pub fn menu_page_juliett() -> Page {
		let source = r#"-- title

Menu Page: juliett

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_juliett 

"#;
		Page::new(
			PathBuf::from("some-project-root/pages/l1b/juliett.neo"),
			source,
			Config::mock_basic_config(),
		)
	}
}
