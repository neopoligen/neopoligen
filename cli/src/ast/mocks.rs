use crate::ast::ast;
use crate::section::Section;
use crate::site_config::SiteConfig;

pub fn ast_mock1() -> Vec<Section> {
    let config = SiteConfig::mock1();
    let content = r#"-- title

This is a title

-- p

This is a paragraph

-- metadata
-- date: 2024-03-04 10:11:12
-- id: ast-mock1"#;
    ast(&content, &config.sections).unwrap()
}
