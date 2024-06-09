use crate::source_page::SourcePage;

impl SourcePage {
    pub fn mock1_20240101_alfa1234_minimal() -> SourcePage {
        let source = "-- title\n\nAlfa Alfa Alfa\n\n-- metadata\n-- id: 20240101_alfa1234\n-- created: 2024-01-01T00:00:00-04:00";
        SourcePage::new_mock_from_str(source)
    }

    pub fn mock2_20240102_bravo123_home_page_path() -> SourcePage {
        let source = "-- title\n\nBravo Bravo Bravo\n\n-- metadata\n-- id: 20240102_bravo123\n-- created: 2024-01-02T00:00:00-04:00\n-- path: /";
        SourcePage::new_mock_from_str(source)
    }

    pub fn mock3_20240103_charlie1_title_in_div_section_and_template() -> SourcePage {
        let source = "-- div\n-- title: Charlie Title From Section\n-- template: attr-template\n\n\n\n-- metadata\n-- id: 20240103_bravo123\n-- created: 2024-01-03T00:00:00-04:00\n-- path: /";
        SourcePage::new_mock_from_str(source)
    }

    pub fn mock4_20240104_delta123_type_and_status() -> SourcePage {
        let source = "-- metadata\n-- id: 20240103_bravo123\n-- created: 2024-01-03T00:00:00-04:00\n-- type: custom-type\n-- status: custom-status\n\n";
        SourcePage::new_mock_from_str(source)
    }
}
