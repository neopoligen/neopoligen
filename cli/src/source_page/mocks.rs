use crate::source_page::SourcePage;

impl SourcePage {
    pub fn mock1_20240101_alfa1234_minimal() -> SourcePage {
        let source = "-- title\n\nHello World\n\n-- metadata\n-- id: 20240101_alfa1234\n-- created: 2024-01-01T00:00:00-04:00";
        SourcePage::new_mock_from_str(source)
    }
    pub fn mock1_20240102_bravo123_home_page_path() -> SourcePage {
        let source = "-- title\n\nHello World\n\n-- metadata\n-- id: 20240102_bravo123\n-- created: 2024-01-02T00:00:00-04:00\n-- path: /";
        SourcePage::new_mock_from_str(source)
    }
}
