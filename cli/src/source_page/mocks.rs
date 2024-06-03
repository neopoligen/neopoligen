use crate::source_page::SourcePage;

impl SourcePage {
    pub fn mock1_20240101_alfa1234_minimal() -> SourcePage {
        //let source = "-- title\n\nHello World\n\n-- metadata\n-- id: alfa1234\n-- created: 2024-06-03T17:25:45-04:00";
        let source = "-- title\n\nHello World\n\n-- metadata\n-- id: 20240101_alfa1234";
        SourcePage::new_mock_from_str(source)
    }
}
