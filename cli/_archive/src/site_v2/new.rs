use crate::config::Config;
use crate::site_v2::SiteV2;
use std::collections::BTreeMap;
use std::sync::Mutex;

impl SiteV2 {
    pub fn new(config: Config) -> SiteV2 {
        SiteV2 {
            config,
            pages: BTreeMap::new(),
            page_templates: BTreeMap::new(),
            holder: Mutex::new(BTreeMap::new()),
        }
    }
}