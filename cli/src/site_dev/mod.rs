pub mod new;

use crate::config::Config;
use crate::page::Page;
use std::collections::BTreeMap;
use std::sync::Mutex;

pub struct SiteDev {
    pub pages: BTreeMap<String, Page>,
    pub templates: BTreeMap<String, String>,
    pub cache: Mutex<BTreeMap<String, BTreeMap<String, Option<String>>>>,
    pub config: Config,
}
