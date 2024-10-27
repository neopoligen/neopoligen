use super::BuilderV42;
use crate::neo_error::NeoError;
use crate::site_config::SiteConfigV42;

impl BuilderV42 {
    pub fn new_from_site_config(site_config: &SiteConfigV42) -> Result<BuilderV42, NeoError> {
        Ok(BuilderV42 {
            errors: vec![],
            pages: vec![],
            site_config: site_config.clone(),
            site: None,
        })
    }
}
