use crate::{page_v39::PageV39, site_config::SiteConfig};

impl PageV39 {
    pub fn mock_1_20240101_basic_page() -> PageV39 {
        let config = SiteConfig::mock1();
        let content = "-- title\n\nAlfa Bravo\n\n-- metadata\n-- id: 20240101alfa1234\n-- created: 2024-01-01T10:10:10-04:00\n\n".to_string();
        PageV39::new_from_string(config, content).expect("made page")
    }

    pub fn mock_2_20240102_with_type_and_status() -> PageV39 {
        let config = SiteConfig::mock1();
        let content = "-- title\n\nCharlie Delta\n\n-- metadata\n-- id: 20240102bravo123\n-- created: 2024-01-02T10:10:10-04:00\n-- type: example\n-- status: draft\n-- path: /custom-path".to_string();
        PageV39::new_from_string(config, content).expect("made page")
    }

    pub fn mock_3_integration_alfa() -> PageV39 {
        let content = r#"-- start-theme-test

this is some test

-- title

Alfa Bravo

-- code/

ping

-- /code




-- metadata
-- created: 2024-05-30T12:12:11-04:00
-- updated: 2024-05-30T12:12:11-04:00
-- id: 2hc65pgj
            "#
        .to_string();
        let config = SiteConfig::mock1();
        PageV39::new_from_string(config, content).expect("made page")
    }

    pub fn mock_invalid_ast_1() -> PageV39 {
        let config = SiteConfig::mock1();
        let content = "-- title\n\nAlfa Bravo\n\n-- metadata\n--\n".to_string();
        PageV39::new_from_string(config, content).expect("made page")
    }
}
