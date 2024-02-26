use std::collections::BTreeMap;

use crate::config::Config;
use crate::file_set::FileSet;
use crate::site_dev::SiteDev;
use std::sync::Mutex;

impl SiteDev {
    pub fn new(config: &Config, file_set: FileSet) -> SiteDev {
        let sd = SiteDev {
            cache: Mutex::new(BTreeMap::new()),
            config: config.clone(),
            pages: BTreeMap::new(),
            templates: BTreeMap::new(),
        };
        sd
    }
}
