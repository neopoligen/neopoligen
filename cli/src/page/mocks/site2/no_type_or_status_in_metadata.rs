use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s2_no_type_or_status_in_metadata() -> Page {
        let config = Config::site2_config();
        let source_path = PathBuf::from(
            "leading_folder/Neopoligen/test-site2/content/no_type_or_status_in_metadata.neo",
        );
        let source = r#"-- metadata
-- date: 2022-12-01 14:31:29
-- id: id-no-type-or-status-in-metadata
"#
        .to_string();
        Page::new(source_path, source, &config).unwrap()
    }
}
