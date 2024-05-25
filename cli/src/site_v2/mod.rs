use crate::{page_v2::PageV2, site_config::SiteConfig};
use std::collections::BTreeMap;
use std::path::PathBuf;

pub struct SiteV2 {
    pub config: SiteConfig,
    pub pages: BTreeMap<String, PageV2>,
}

impl SiteV2 {
    pub fn new(config: &SiteConfig, source_pages: &BTreeMap<PathBuf, PageV2>) -> SiteV2 {
        let mut pages = BTreeMap::new();
        source_pages.iter().for_each(|p| {
            dbg!(&p.1.id());
            ()
        });

        // .iter().for_each(|p| {
        //     dbg!(&p.id());
        // });
        SiteV2 {
            config: config.clone(),
            pages,
        }
    }
}
