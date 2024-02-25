use crate::config::Config;
use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s1_title_with_inline_span() -> Page {
        let config = Config::site1_config();
        let source_path = PathBuf::from(
            "leading_folder/Neopoligen/dev-test-site/content/s1_title_with_inline_span.neo",
        );
        let source = r#"-- title

Title <<strong|With Inline>> Span

-- metadata
-- date: 2024-02-24 19:11:09
-- id: s1_title_with_inline_span
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
