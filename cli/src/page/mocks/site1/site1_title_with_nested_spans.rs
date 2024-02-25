use crate::config::Config;
use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn site1_title_with_nested_spans() -> Page {
        let config = Config::site1_config();
        let source_path = PathBuf::from(
            "leading_folder/Neopoligen/dev-test-site/content/site1_title_with_nested_spans.neo",
        );
        let source = r#"-- title

Nested <<strong|<<em|Span>>>> Test

-- metadata
-- date: 2024-02-24 19:11:09
-- id: site1_title_with_nested_spans
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
