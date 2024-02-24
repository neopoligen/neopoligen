use crate::config::Config;
use crate::site_builder::SiteBuilder;
use crate::site_v2::SiteV2;
use minijinja::Environment;
use std::collections::BTreeMap;
use tracing::instrument;

impl SiteBuilder<'_> {
    #[instrument]
    pub fn new(config: Config) -> SiteBuilder<'static> {
        let mut site = SiteV2::new(config.clone());
        site.load_pages();
        SiteBuilder {
            config: config.clone(),
            env: Environment::new(),
            page_tests: BTreeMap::new(),
            pages: BTreeMap::new(),
            site,
        }
    }
}
