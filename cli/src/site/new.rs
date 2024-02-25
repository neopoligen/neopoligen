use crate::config::Config;
use crate::site::Site;
use std::collections::BTreeMap;
use std::sync::Mutex;

impl Site {
    pub fn new(config: Config) -> Site {
        let site = Site {
            pages: BTreeMap::new(),
            cache: Mutex::new(BTreeMap::new()),
            config,
        };
        site.prep_cache();
        site
    }
}
