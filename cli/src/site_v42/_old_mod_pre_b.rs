pub mod b;

use crate::helpers::flatten_spans_in_template_v42;
use crate::helpers::highlight_span_v42;
use crate::page_v42::PageV42;
use crate::site_config::SiteConfigV42;
use html_escape::*;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::ErrorKind;
use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

// TODO: Deprecate in favor of SiteV42b
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SiteV42 {
    pub config: SiteConfigV42,
    pub pages: BTreeMap<String, PageV42>,
}

// TODO: Deprecate in favor of SiteV42b
impl SiteV42 {
    // TODO: Add tests to this and error handling
    // TODO: escape for html
    // TODO: Set up config of specific attrs/flags to hoist
    //       and only add those to the output
    pub fn get_span_attrs(&self, args: &[Value]) -> Result<Value, Error> {
        //let mut classes: Vec<String> = vec![];
        let mut attr_string: Vec<String> = vec![];
        if args.len() > 0 {
            let attrs = args[0].get_item(&Value::from("attrs")).unwrap();
            for key in attrs.try_iter()? {
                let spans = attrs.get_item(&key)?;
                let values = spans
                    .try_iter()?
                    .filter_map(|v| {
                        if key.to_string() != "class".to_string() {
                            Some(
                                encode_quoted_attribute(
                                    &v.get_item(&Value::from("text")).unwrap().to_string(),
                                )
                                .to_string(),
                            )
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>();
                if values.len() > 0 {
                    attr_string.push(format!(r#"{}="{}""#, key.to_string(), values.join(" ")));
                }
            }
            let mut classes: Vec<String> = vec![];
            for key in attrs.try_iter()? {
                let spans = attrs.get_item(&key)?;
                classes = spans
                    .try_iter()?
                    .filter_map(|v| {
                        if key.to_string() == "class".to_string() {
                            Some(
                                encode_quoted_attribute(
                                    &v.get_item(&Value::from("text")).unwrap().to_string(),
                                )
                                .to_string(),
                            )
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>();
            }
            if args.len() == 2 {
                classes.push(args[1].to_string());
            }
            if classes.len() > 0 {
                attr_string.push(format!(r#"class="{}""#, classes.join(" ")));
            }
            let flags = args[0].get_item(&Value::from("flags")).unwrap();
            for flag in flags.try_iter()? {
                attr_string.push(format!(r#"{}"#, flag));
            }
            Ok(Value::from(format!(" {}", attr_string.join(" "))))
        } else {
            Ok(Value::from(""))
        }
    }

    // // MIGRATED
    // // TODO: Add error handling here
    // pub fn get_page_href_from_id(&self, args: &[Value]) -> Result<Value, Error> {
    //     if args.len() == 1 {
    //         let page_id = &args[0].to_string().trim().to_string();
    //         match self.pages.get(page_id) {
    //             Some(page) => page.href(),
    //             None => Ok(Value::from("/")),
    //         }
    //     } else {
    //         Err(Error::new(
    //             ErrorKind::InvalidOperation,
    //             "ERROR_COULD_NOT_GET_HREF",
    //         ))
    //     }
    // }

    // // MIGRATED
    // // TODO: Add error handling here
    // pub fn get_page_href_from_span(&self, args: &[Value]) -> Result<Value, Error> {
    //     if args.len() == 1 {
    //         let x = &args[0].get_item(&Value::from("spans")).unwrap();
    //         let page_id = flatten_spans_in_template_v42(&[x.to_owned()]).unwrap();
    //         match self.pages.get(&page_id.to_string()) {
    //             Some(page) => page.href(),
    //             None => Ok(Value::from("/")),
    //         }
    //     } else {
    //         Err(Error::new(
    //             ErrorKind::InvalidOperation,
    //             "NO_ID_FOR_PAGE_TITLE",
    //         ))
    //     }
    // }

    // // MIGRATED
    // // TODO: Add error handling here
    // pub fn get_page_title_from_id(&self, args: &[Value]) -> Result<Value, Error> {
    //     if args.len() == 1 {
    //         let page_id = &args[0].to_string().trim().to_string();
    //         match self.pages.get(page_id) {
    //             Some(page) => page.title(),
    //             None => Ok(Value::from(page_id)),
    //         }
    //     } else {
    //         Err(Error::new(
    //             ErrorKind::InvalidOperation,
    //             "NO_ID_FOR_PAGE_TITLE",
    //         ))
    //     }
    // }

    // // MIGRATED
    // // TODO: Add error handling here
    // pub fn get_page_title_from_span(&self, args: &[Value]) -> Result<Value, Error> {
    //     if args.len() == 1 {
    //         let x = &args[0].get_item(&Value::from("spans")).unwrap();
    //         let page_id = flatten_spans_in_template_v42(&[x.to_owned()]).unwrap();
    //         match self.pages.get(&page_id.to_string()) {
    //             Some(page) => page.title(),
    //             None => Ok(Value::from(page_id)),
    //         }
    //     } else {
    //         Err(Error::new(
    //             ErrorKind::InvalidOperation,
    //             "NO_ID_FOR_PAGE_TITLE",
    //         ))
    //     }
    // }

    pub fn highlight_syntax(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() == 1 {
            let span_type = &args[0].get_item(&Value::from("type")).unwrap();
            if span_type.to_string() == "code_shorthand".to_string() {
                let code = &args[0].get_item(&Value::from("text")).unwrap();
                let lang = &args[0]
                    .get_item(&Value::from("flags"))
                    .unwrap()
                    .try_iter()?
                    .nth(0);
                match lang {
                    Some(l) => Ok(Value::from(highlight_span_v42(
                        &code.to_string(),
                        &l.to_string(),
                    ))),
                    None => Ok(Value::from(highlight_span_v42(&code.to_string(), "txt"))),
                }
            } else {
                Ok(Value::from("ERROR_IN_CODE_HIGHLIGHT"))
            }
        } else {
            Ok(Value::from("ERROR_IN_CODE_HIGHLIGHT"))
        }
    }
}

impl Object for SiteV42 {
    fn call_method(
        self: &Arc<SiteV42>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "get_span_attrs" => self.get_span_attrs(args),
            "get_page_href_from_id" => self.get_page_href_from_id(args),
            "get_page_href_from_span" => self.get_page_href_from_span(args),
            "get_page_title_from_id" => self.get_page_title_from_id(args),
            "get_page_title_from_span" => self.get_page_title_from_span(args),
            "highlight_syntax" => self.highlight_syntax(args),
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
