// Deprecated: This is now in the config
// so this should be removed
use crate::site_sections::SiteSections;

impl SiteSections {
    pub fn mock1() -> SiteSections {
        SiteSections {
            basic: vec![
                "div".to_string(),
                "note".to_string(),
                "p".to_string(),
                "title".to_string(),
            ],
            json: vec!["metadata".to_string()],
            raw: vec!["code".to_string()],
            checklist: vec!["checklist".to_string()],
            list: vec!["list".to_string()],
        }
    }
}
