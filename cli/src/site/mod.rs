pub mod display;
pub mod new;
pub mod object;

use crate::config::Config;
use crate::page::Page;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Mutex;

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub pages: BTreeMap<String, Page>,
    pub cache: Mutex<BTreeMap<String, BTreeMap<String, Option<String>>>>,
    pub config: Config,
}
