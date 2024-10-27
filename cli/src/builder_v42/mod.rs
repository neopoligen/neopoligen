pub mod inits;

use std::collections::BTreeMap;

use crate::{neo_error::NeoError, page_v42::PageV42, site::SiteV42, site_config::SiteConfigV42};
use anyhow::Result;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

pub struct BuilderV42 {
    pub errors: Vec<NeoError>,
    pub pages: Vec<PageV42>,
    pub site_config: SiteConfigV42,
    pub site: Option<SiteV42>,
}

impl BuilderV42 {
    #[instrument(skip(self))]
    pub fn load_pages_from_fs(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Pages From FS");
        for entry in WalkDir::new(&self.site_config.content_source_dir()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neo"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        match PageV42::new_from_fs_path(&path, &self.site_config) {
                            Ok(p) => {
                                dbg!(&p.ast());
                                //event![Level::INFO, "Added raw page: {}", path.display()];
                                self.pages.push(p)
                            }
                            Err(e) => {
                                // event![Level::ERROR, "Error loading raw page: {}", path.display()];
                                self.errors.push(e)
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[instrument(skip(self, site_config))]
    pub fn prep_site(&mut self, site_config: &SiteConfigV42) -> Result<()> {
        event!(Level::INFO, "Prepping Site");
        let mut site = SiteV42 {
            config: site_config.clone(),
            pages: BTreeMap::new(),
        };
        self.pages.iter().for_each(|p| match p.id() {
            Ok(id) => {
                site.pages.insert(id.to_string(), p.clone());
                ()
            }
            Err(e) => {
                // event!(Level::ERROR, "prep_site Error: {}", e);
                // self.errors.push(e)
            }
        });
        self.site = Some(site);
        Ok(())
    }

    //
}
