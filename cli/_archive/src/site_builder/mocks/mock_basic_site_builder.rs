use crate::config::Config;
use crate::site_builder::SiteBuilder;
use crate::site_v2::SiteV2;
use minijinja::Environment;
use std::collections::BTreeMap;

impl SiteBuilder<'_> {
    pub fn mock_basic_builder() -> SiteBuilder<'static> {
        SiteBuilder {
            config: Config::mock_basic_config(),
            env: Environment::new(),
            pages: BTreeMap::new(),
            page_tests: BTreeMap::new(),
            site: SiteV2::new(Config::mock_basic_config()),
        }
    }
}
