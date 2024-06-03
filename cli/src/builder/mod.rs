use std::fs;

use crate::neo_error::NeoError;
use crate::source_page::SourcePage;
use crate::{engine_config::EngineConfig, site_config::SiteConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Builder {
    config: Option<SiteConfig>,
    errors: Vec<NeoError>,
    pages: Vec<SourcePage>,
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
            errors: vec![],
            pages: vec![],
        };
        Ok(b)
    }
}

impl Builder {
    #[instrument(skip(self))]
    pub fn generate_missing_asts(&mut self) {
        self.pages.iter_mut().for_each(|page| {
            if let Err(e) = page.generate_ast() {
                self.errors.push(e);
            }
        })
    }

    #[instrument(skip(self))]
    pub fn load_pages_from_fs(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Source Content Files");
        for entry in WalkDir::new(&self.config.as_ref().unwrap().content_source_dir()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neo"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        let page = SourcePage::new_from_source_path(
                            &path,
                            self.config.as_ref().unwrap().clone(),
                        )?;
                        self.pages.push(page);
                    }
                }
            }
        }
        Ok(())
    }

    #[instrument(skip(self, thing))]
    pub fn todo(&self, thing: &str) {
        event!(Level::INFO, "TODO: {}", thing);
    }
}

