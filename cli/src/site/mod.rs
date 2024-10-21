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
use crate::page::PageV41;
use crate::page_payload::PagePayload;
use crate::site_config::SiteConfig; // TODO: Deprecate SiteConfig In Favor Of SiteConfigV41
use crate::site_config::SiteConfigV41;
use crate::source_page::SourcePage;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SiteV41 {
    pub config: SiteConfigV41,
    pub pages: BTreeMap<String, PageV41>,
}

impl SiteV41 {
    pub fn ping(&self, args: &[Value]) -> Value {
        self.pages.get("dev-alfa").unwrap().dev_ping(args)
        //Value::from("THIS IS IT")
    }

    // pub fn new_from_source_pages(
    //     // TODO: Switch thsi to PageV41
    //     source_pages: &BTreeMap<PathBuf, SourcePage>,
    // ) -> Result<SiteV41, NeoError> {
    //     let mut site = SiteV41 {
    //         config: None,
    //         pages: BTreeMap::new(),
    //     };
    //     site.load_source_pages(&source_pages);
    //     Ok(site)
    // }

    // pub fn load_source_pages(&self, source_pages: &BTreeMap<PathBuf, SourcePage>) {
    //     source_pages
    //         .iter()
    //         .for_each(|(source_path, source_content)| {
    //             dbg!(&source_content);
    //             ()
    //         })
    // }

    //
}

// TODO: Deprecate this as SiteV41 comes online
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

impl Object for SiteV41 {
    fn call_method(
        self: &Arc<SiteV41>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "ping" => Ok(Value::from(self.ping(args))),
            _ => Ok(Value::from("")),
        }
    }
}
