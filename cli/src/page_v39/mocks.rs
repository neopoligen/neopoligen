use crate::{page_v39::PageV39, site_config::SiteConfig};

impl PageV39 {
    pub fn mock_1_20240101_basic_page() -> PageV39 {
        let mut p = PageV39 {
            ast: None,
            config: SiteConfig::mock1(),
            errors: vec![],
            fs_modified: None,
            output_content: None,
            source_content: Some("-- title\n\nAlfa Bravo\n\n-- metadata\n-- id: 20240101alfa\n-- created: 2024-05-31T12:25:52-04:00".to_string()),
        };
        let _ = p.generate_ast();
        p
    }

    pub fn mock_invalid_ast_1() -> PageV39 {
        let mut p = PageV39 {
            ast: None,
            config: SiteConfig::mock1(),
            errors: vec![],
            fs_modified: None,
            output_content: None,
            source_content: Some("-- title\n\nAlfa Bravo\n\n-- metadata\n--\n".to_string()),
        };
        let _ = p.generate_ast();
        p
    }
}
