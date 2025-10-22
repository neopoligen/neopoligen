#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use crate::image::Image;
use crate::neo_error::NeoError;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use std::sync::Arc;
// use crate::neo_error::{NeoError, NeoErrorKind};
use crate::page_payload::PagePayload;
use crate::page_v42::PageV42;
use crate::site_config::SiteConfig; // TODO: Deprecate SiteConfig In Favor Of SiteConfigV42
use crate::site_config::SiteConfigV42;
use crate::source_page::SourcePage;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

// TODO: Deprecate this as SiteV42 comes online
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Site {
    pub absolute_page_urls: BTreeMap<String, String>,
    pub config: SiteConfig,
    pub images: BTreeMap<String, Image>,
    pub page_titles: BTreeMap<String, String>,
    pub pages: BTreeMap<String, PagePayload>,
}

impl Site {
    pub fn new_from_payloads(config: SiteConfig, payloads: &BTreeMap<String, PagePayload>) -> Site {
        let mut site = Site {
            config: config.clone(),
            images: BTreeMap::new(),
            absolute_page_urls: BTreeMap::new(),
            page_titles: BTreeMap::new(),
            pages: payloads.clone(),
        };
        payloads.iter().for_each(|payload| {
            if let Some(url) = payload.1.absolute_url.clone() {
                site.absolute_page_urls.insert(payload.0.clone(), url);
            }
            if let Some(title) = payload.1.title.clone() {
                site.page_titles.insert(payload.0.clone(), title);
            }
        });
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
