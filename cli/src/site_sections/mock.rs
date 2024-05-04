use crate::site_sections::SiteSections;

impl SiteSections {
    pub fn mock1() -> SiteSections {
        SiteSections {
            basic: vec!["p".to_string()],
            json: vec!["metadata".to_string()],
            raw: vec!["code".to_string()],
            checklist: vec!["checklist".to_string()],
            list: vec!["list".to_string()],
        }
    }
}
