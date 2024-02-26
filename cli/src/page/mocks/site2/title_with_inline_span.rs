use crate::config::Config;
use crate::page::parse::parse;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn s2_title_with_inline_span() -> Page {
        let config = Config::site2_config();
        let source_path = PathBuf::from(
            "leading_folder/Neopoligen/test-site2/content/title_with_inline_span.neo",
        );
        let source = r#"-- title

Title <<strong|With Inline>> Span

-- metadata
-- date: 2022-07-08 11:31:29
-- id: title_with_inline_span
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
