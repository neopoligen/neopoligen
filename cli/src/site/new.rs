use crate::site::Site;
use std::collections::BTreeMap;
use std::sync::Mutex;

impl Site {
    pub fn new() -> Site {
        let site = Site {
            pages: BTreeMap::new(),
            cache: Mutex::new(BTreeMap::new()),
        };
        site.prep_cache();
        site
    }
}
