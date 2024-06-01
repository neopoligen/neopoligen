use std::collections::BTreeMap;

use crate::builder_v39::BuilderV39;
use crate::site_config::SiteConfig;

impl BuilderV39 {
    pub fn mock1() -> BuilderV39 {
        let config = SiteConfig::mock1();
        let pages = BTreeMap::new();
        let issues = vec![];
        BuilderV39 {
            config,
            issues,
            pages,
        }
    }
}
