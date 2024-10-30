use crate::helpers::flatten_spans_in_template_v42;
use crate::page_v42::PageV42;
use crate::site_config::SiteConfigV42;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::ErrorKind;
use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
// use crate::neo_error::{NeoError, NeoErrorKind};
// use crate::image::Image;
// use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SiteV42 {
    pub config: SiteConfigV42,
    pub pages: BTreeMap<String, PageV42>,
}

impl SiteV42 {
    pub fn get_page_href_from_id(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() == 1 {
            let page_id = &args[0].to_string().trim().to_string();
            match self.pages.get(page_id) {
                Some(page) => page.href(),
                None => Ok(Value::from("/")),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                "COULD_NOT_GET_HREF",
            ))
        }
    }

    // TODO: Add error handling here
    pub fn get_page_href_from_span(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() == 1 {
            let x = &args[0].get_item(&Value::from("spans")).unwrap();
            let page_id = flatten_spans_in_template_v42(&[x.to_owned()]).unwrap();
            match self.pages.get(&page_id.to_string()) {
                Some(page) => page.href(),
                None => Ok(Value::from("/")),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                "NO_ID_FOR_PAGE_TITLE",
            ))
        }
    }

    pub fn get_page_title_from_id(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() == 1 {
            let page_id = &args[0].to_string().trim().to_string();
            match self.pages.get(page_id) {
                Some(page) => page.title(),
                None => Ok(Value::from(page_id)),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                "NO_ID_FOR_PAGE_TITLE",
            ))
        }
    }

    // TODO: Add error handling here
    pub fn get_page_title_from_span(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() == 1 {
            let x = &args[0].get_item(&Value::from("spans")).unwrap();
            let page_id = flatten_spans_in_template_v42(&[x.to_owned()]).unwrap();
            match self.pages.get(&page_id.to_string()) {
                Some(page) => page.title(),
                None => Ok(Value::from(page_id)),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                "NO_ID_FOR_PAGE_TITLE",
            ))
        }
    }

    // if args.len() == 1 {
    //     let x: &String = &args[0]
    //         .try_iter()?
    //         .map(|v| v.get_item(&Value::from("text")).unwrap().to_string())
    //         .collect::<Vec<_>>()
    //         .join("");
    //     Ok(Value::from(x))
    // } else {
    //     Err(Error::new(
    //         ErrorKind::InvalidOperation,
    //         "Missing argument for flatten_spans in template".to_string(),
    //     ))
    // }
}

impl Object for SiteV42 {
    fn call_method(
        self: &Arc<SiteV42>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "get_page_href_from_id" => self.get_page_href_from_id(args),
            "get_page_href_from_span" => self.get_page_href_from_span(args),
            "get_page_title_from_id" => self.get_page_title_from_id(args),
            "get_page_title_from_span" => self.get_page_title_from_span(args),
            _ => Ok(Value::from("")),
        }
    }
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
