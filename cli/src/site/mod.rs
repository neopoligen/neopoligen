use crate::image::Image;
use crate::{page_payload::PagePayload, site_config::SiteConfig};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
// use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Site {
    pub config: SiteConfig,
    pub images: BTreeMap<String, Image>,
}

impl Site {
    pub fn new_from_payloads(
        config: SiteConfig,
        _payloads: &BTreeMap<String, PagePayload>,
    ) -> Site {
        let site = Site {
            config: config.clone(),
            images: BTreeMap::new(),
        };
        site
    }

    // pub fn load_images(&mut self) {
    //     self.images.insert(
    //         "example-fish".to_string(),
    //         Image {
    //             dir: PathBuf::from("/neo-images/example-fish"),
    //             extension: "jpg".to_string(),
    //             raw_width: 280,
    //             raw_height: 280,
    //         },
    //     );
    //     self.images.insert(
    //         "foreground".to_string(),
    //         Image {
    //             dir: PathBuf::from("/neo-images/foreground"),
    //             extension: "jpg".to_string(),
    //             raw_width: 280,
    //             raw_height: 280,
    //         },
    //     );
    // }
}
