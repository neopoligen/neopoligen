use crate::{page_payload::PagePayload, site_config::SiteConfig};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Site {
    config: SiteConfig,
}

impl Site {
    pub fn new_from_payloads(
        config: SiteConfig,
        _payloads: &BTreeMap<String, PagePayload>,
    ) -> Site {
        let site = Site {
            config: config.clone(),
        };
        site
    }
}
