use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
	pub fn menu_page_uniform() -> Page {
		let source = r#"-- title

Menu Page: uniform

-- metadata
-- date: 2021-07-02 12:18:47
-- id: menu_uniform 

"#;
		Page::new(
			PathBuf::from("some-project-root/pages/l1a/l21/l3a/uniform.neo"),
			source,
			Config::mock_basic_config(),
		)
	}
}
