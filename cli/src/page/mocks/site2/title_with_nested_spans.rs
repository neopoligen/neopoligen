use crate::config::Config;
use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s2_title_with_nested_spans() -> Page {
        let config = Config::site1_config();
        let source_path = PathBuf::from(
            "leading_folder/Neopoligen/dev-test-site/content/title_with_nested_spans.neo",
        );
        let source = r#"-- title

Nested <<strong|<<em|Span>>>> Test

-- metadata
-- date: 2021-02-14 08:22:04
-- id: id_title_with_nested_spans
"#
        .to_string();
        let ast = parse(&source, &config);
        Page {
            ast,
            source,
            source_path,
        }
    }
}
