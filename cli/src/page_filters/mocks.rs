use crate::page_filters::*;

impl PageFilterOrSet {
    pub fn mock1_status_published() -> PageFilterOrSet {
        PageFilterOrSet {
            and_groups: vec![PageFilterAndGroup {
                filters: vec![PageFilter::Status {
                    exclude: false,
                    value: "published".to_string(),
                }],
            }],
        }
    }
}
