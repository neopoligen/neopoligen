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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SiteV42 {
    pub config: SiteConfigV42,
    pub pages: BTreeMap<String, PageV42>,
}

impl SiteV42 {
    //pub fn ping(&self, args: &[Value]) -> Value {
    //    self.pages.get("dev-alfa").unwrap().dev_ping(args)
    //    //Value::from("THIS IS IT")
    //}

    // pub fn new_from_source_pages(
    //     // TODO: Switch thsi to PageV42
    //     source_pages: &BTreeMap<PathBuf, SourcePage>,
    // ) -> Result<SiteV42, NeoError> {
    //     let mut site = SiteV42 {
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

impl SiteV42 {
    // pub fn page_url(&self, args: &[Value]) -> Result<Value, Error> {
    //     if args.len() == 1 {
    //         let key = args[0].to_string();
    //         if let Some(p) = self.pages.get(&key) {
    //             if let Ok(id) = p.id() {
    //                 // TODO: call default language properly here
    //                 Ok(Value::from(format!("/en/{}/", id)))
    //             } else {
    //                 Ok(Value::from("/"))
    //             }
    //         } else {
    //             Ok(Value::from("/"))
    //         }
    //     } else {
    //         // TODO: Figure out what error to throw here
    //         Ok(Value::from("/"))
    //     }
    // }

    // pub fn page_title(&self, args: &[Value]) -> Result<Value, Error> {
    //     if args.len() == 1 {
    //         let key = args[0].to_string();
    //         if let Some(p) = self.pages.get(&key) {
    //             if let Ok(title) = p.title() {
    //                 Ok(title)
    //             } else {
    //                 // TODO: Send error here about missing argument
    //                 // for page title
    //                 Ok(Value::from(""))
    //             }
    //         } else {
    //             // TODO: Send error here about missing argument
    //             // for page title
    //             Ok(Value::from(""))
    //         }
    //     } else {
    //         // TODO: Send error here about missing argument
    //         // for page title
    //         Ok(Value::from(""))
    //     }
    // }

    //
}

impl Object for SiteV42 {
    fn call_method(
        self: &Arc<SiteV42>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            // "page_title" => self.page_title(args),
            // "page_url" => self.page_url(args),
            // "ping" => Ok(Value::from(self.ping(args))),
            _ => Ok(Value::from("")),
        }
    }
}
