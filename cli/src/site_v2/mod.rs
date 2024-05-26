use crate::{page_v2::PageV2, site_config::SiteConfig};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct SiteV2 {
    pub config: SiteConfig,
    pub pages: BTreeMap<String, PageV2>,
}

impl SiteV2 {
    pub fn new(config: &SiteConfig, source_pages: &BTreeMap<PathBuf, PageV2>) -> SiteV2 {
        let mut pages: BTreeMap<String, PageV2> = BTreeMap::new();
        source_pages.iter().for_each(|p| {
            if let Some(id) = p.1.id() {
                pages.insert(id, p.1.clone());
            }
        });
        SiteV2 {
            config: config.clone(),
            pages,
        }
    }
}
