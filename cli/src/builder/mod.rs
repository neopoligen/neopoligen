use crate::{engine_config::EngineConfig, site_config::SiteConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{event, instrument, Level};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Builder {
    config: Option<SiteConfig>,
}

impl Builder {
    pub fn new_from_engine_config(engine_config: &EngineConfig) -> Result<Builder> {
        let b = Builder { config: None };
        Ok(b)
    }
}

impl Builder {
    #[instrument(skip(self, thing))]
    pub fn todo(&self, thing: &str) {
        event!(Level::INFO, "TODO: {}", thing);
    }
}

