use crate::{page_v39::PageV39, site_config::SiteConfig};

impl PageV39 {
    pub fn mock_1_20240101_basic_page() -> PageV39 {
        let mut p = PageV39 {
            ast: None,
            config: SiteConfig::mock1(),
            errors: vec![],
            fs_modified: None,
            output_content: None,
            source_content: Some("-- title\n\nAlfa Bravo\n\n-- metadata\n-- id: 20240101alfa1234\n-- created: 2024-01-01T10:10:10-04:00\n\n".to_string()),
            source_path: None,
        };
        let _ = p.generate_ast();
        p
    }

    pub fn mock_2_20240102_with_type_and_status() -> PageV39 {
        let mut p = PageV39 {
            ast: None,
            config: SiteConfig::mock1(),
            errors: vec![],
            fs_modified: None,
            output_content: None,
            source_content: Some("-- title\n\nCharlie Delta\n\n-- metadata\n-- id: 20240102bravo123\n-- created: 2024-01-02T10:10:10-04:00\n-- type: example\n-- status: draft\n-- path: /custom-path".to_string()),
            source_path: None,
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
            source_path: None,
        };
        let _ = p.generate_ast();
        p
    }
}
