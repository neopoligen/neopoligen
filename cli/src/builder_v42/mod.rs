use crate::{neo_error::NeoError, page_v42::PageV42, site_config::SiteConfigV42};
use anyhow::Result;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

pub struct BuilderV42 {
    pub errors: Vec<NeoError>,
    pub pages: Vec<PageV42>,
    pub site_config: SiteConfigV42,
}

impl BuilderV42 {
    pub fn new_from_site_config(site_config: &SiteConfigV42) -> Result<BuilderV42, NeoError> {
        Ok(BuilderV42 {
            errors: vec![],
            pages: vec![],
            site_config: site_config.clone(),
        })
    }
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
}
