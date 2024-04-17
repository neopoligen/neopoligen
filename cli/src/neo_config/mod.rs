use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
pub struct NeoConfig {
    pub active_site: Option<String>,
    pub page_cache_path: Option<PathBuf>,
}
