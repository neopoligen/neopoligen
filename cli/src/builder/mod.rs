use std::fs;

use crate::{engine_config::EngineConfig, site_config::SiteConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use tracing::{event, instrument, Level};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Builder {
    config: Option<SiteConfig>,
}

impl Builder {
    pub fn new_from_engine_config(engine_config: &EngineConfig) -> Result<Builder> {
        let project_root = engine_config
            .sites_dir
            .join(engine_config.active_site.as_str());
        let config_path = project_root.join("admin").join("config.json");
        let text = fs::read_to_string(config_path)?;
        let mut config = serde_json::from_str::<SiteConfig>(&text)?;
        config.project_root = Some(project_root);
        config.load_sections();
        let b = Builder {
            config: Some(config),
        };
        Ok(b)
    }
}

impl Builder {
    #[instrument(skip(self, thing))]
    pub fn todo(&self, thing: &str) {
        event!(Level::INFO, "TODO: {}", thing);
    }
}

