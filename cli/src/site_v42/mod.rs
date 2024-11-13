use crate::helpers::flatten_spans_from_template_v42;
use crate::page_v42::PageV42b;
use crate::site_config::SiteConfigV42;
use chrono::prelude::*;
use html_escape::*;
use minijinja::value::Object;
use minijinja::value::ValueKind;
use minijinja::Error;
use minijinja::ErrorKind;
use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SiteV42b {
    pub config: SiteConfigV42,
    pub pages: BTreeMap<String, PageV42b>,
    pub pages_without_ids: BTreeMap<PathBuf, PageV42b>,
    pub generated_at: Option<DateTime<Local>>,
}

impl SiteV42b {
    pub fn new_with_pages(
        config: &SiteConfigV42,
        raw_pages: &BTreeMap<PathBuf, PageV42b>,
    ) -> SiteV42b {
        let mut pages: BTreeMap<String, PageV42b> = BTreeMap::new();
        let mut pages_without_ids: BTreeMap<PathBuf, PageV42b> = BTreeMap::new();
        for (_, page) in raw_pages {
            match &page.id {
                Some(id) => {
                    pages.insert(id.to_string(), page.clone());
                    ()
                }
                None => {
                    pages_without_ids.insert(page.source_absolute_path.clone(), page.clone());
                    ()
                }
            }
        }
        let site = SiteV42b {
            config: config.clone(),
            pages,
            pages_without_ids,
            generated_at: Some(chrono::prelude::Local::now()),
        };
        site
    }
}

impl SiteV42b {
    // For time stamp output
    pub fn generated_at(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() == 1 {
            Ok(Value::from(
                self.generated_at
                    .unwrap()
                    .format(&args[0].to_string())
                    .to_string(),
            ))
        } else {
            Ok(Value::from(
                self.generated_at.unwrap().format("%B").to_string(),
            ))
        }
    }

    // Get the href for a page which is used
    // for all linking.
    pub fn href_for_page(&self, args: &[Value]) -> Option<String> {
        match &self.get_page_id(args) {
            Some(id) => match &self.pages.get(id) {
                Some(page) => page.href.clone(),
                None => None,
            },
            None => {
                // TODO: Hoist Error
                None
            }
        }
    }

    // The base href where Neopoligen stores
    // the files it works with (e.g. theme and image files)
    pub fn neo_files_href(&self) -> Option<String> {
        Some(format!("/{}", &self.config.neo_files_output_folder_name()))
    }

    // The base href for the current theme. All the
    // support files from a theme go under here and
    // this method can be used to construct links
    // and paths for them
    pub fn theme_folder_href(&self) -> Option<String> {
        Some(format!(
            "/{}/{}",
            &self.config.neo_files_output_folder_name(),
            &self.config.theme_files_output_folder_name()
        ))
    }

    // The title for a page based off it's ID. Uses
    // .get_page_id() which accepts spans whose first
    // plain-text span has is the text of the id,
    // or the ID passed in directly
    pub fn title_for_page(&self, args: &[Value]) -> Option<String> {
        match &self.get_page_id(args) {
            Some(id) => match &self.pages.get(id) {
                Some(page) => page.title.clone(),
                None => None,
            },
            None => {
                // TODO: Hoist Error
                None
            }
        }
    }

    // TODO: Add tests for this
    // Takes in either text or spans. If it's text, it's
    // returned directly. If it's spans the first plain-text
    // span is pulled out and it's text is returned.
    pub fn get_page_id(&self, args: &[Value]) -> Option<String> {
        if args.len() == 1 {
            match &args[0].kind() {
                ValueKind::Map => {
                    match &args[0].get_attr("spans") {
                        Ok(spans) => match spans.get_item_by_index(0) {
                            Ok(span) => match span.get_attr("text") {
                                Ok(text) => Some(text.to_string()),
                                Err(_e) => {
                                    // TODO: Hoist Error Here
                                    None
                                }
                            },
                            Err(_e) => {
                                // TODO: Hoist Error Here
                                None
                            }
                        },
                        Err(_e) => {
                            // TODO: Hoist Error Here
                            None
                        }
                    }
                    //Ok(Value::from("VALUEKIND: MAP"))
                }
                ValueKind::String => Some(args[0].to_string()),
                _ => {
                    // TODO: Hoist Error Here
                    None
                }
            }
        } else {
            // TODO: Hoist Error Here
            None
        }
    }

    // TODO: Deprecate this in favor of just getting IDs and
    // calling the pages from the site object without having
    // to clone in a new page
    pub fn get_page_from_span(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() == 1 {
            let x = &args[0].get_item(&Value::from("spans")).unwrap();
            let page_id = flatten_spans_from_template_v42(&[x.to_owned()]).unwrap();
            match self.pages.get(&page_id.to_string()) {
                Some(page) => Ok(Value::from_object(page.clone())),
                None => Err(Error::new(
                    ErrorKind::InvalidOperation,
                    "Could not get page from get_page_from_span",
                )),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                "Could not get page from get_page_from_span",
            ))
        }
    }

    // All the ids for all pages in the site.
    // TODO: Probably deprecate this
    pub fn get_page_ids(&self, _args: &[Value]) -> Result<Value, Error> {
        Ok(Value::from_serialize(
            &self.pages.keys().cloned().collect::<Vec<String>>(),
        ))
    }

    // TODO: Figure out if this is still needed. It's
    // what was originally used for error output, but
    // it might not be needed when the errors are
    // sent to the builder or an external object
    pub fn get_page_with_display_path(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() >= 3 {
            match &self.pages_without_ids.get(&PathBuf::from(&args[2].to_string())) {
                Some(p) => {
                    let p2 = *p;
                    Ok(Value::from_object(p2.clone()))
                }
                None => Err(Error::new(
                    ErrorKind::InvalidOperation,
                    r#"Could not get page with display_path. TODO: Add requested display_path here"#.to_string(),
                )),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                r#"Not enough values pased to site.get_page_with_display_path.
Needs to be 'site.get_page_with_id(site, page, ID_YOU_WANT)'"#
                    .to_string(),
            ))
        }
    }

    pub fn get_page_with_id(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() >= 3 {
            match &self.pages.get(&args[2].to_string()) {
                Some(p) => {
                    let p2 = *p;
                    Ok(Value::from_object(p2.clone()))
                }
                None => Err(Error::new(
                    ErrorKind::InvalidOperation,
                    r#"Could not get page with id. TODO: Add requested id here"#.to_string(),
                )),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                r#"Not enough values pased to site.get_page_with_id
Needs to be 'site.get_page_with_id(site, page, ID_YOU_WANT)'"#
                    .to_string(),
            ))
        }
    }

    // TODO: Should be able to deprecate this when new
    // error handling at the builder or external object
    // level is done.
    pub fn get_keys_for_pages_without_ids(&self, _args: &[Value]) -> Result<Value, Error> {
        Ok(Value::from_serialize(
            &self
                .pages_without_ids
                .keys()
                .cloned()
                .collect::<Vec<PathBuf>>(),
        ))
    }

    // Gets the attributes for a section.
    // TODO: Also get flags
    // TODO: Add tests
    pub fn get_section_attrs(&self, args: &[Value]) -> Result<Value, Error> {
        let mut attr_collection: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut classes = vec![];
        if args.len() >= 1 {
            let kind = args[0].get_attr("kind").unwrap();
            classes.push(format!("{}-section", kind));
            let attrs = args[0].get_attr("attrs").unwrap();
            for key in attrs.try_iter()? {
                if key.to_string() == "class".to_string() {
                    let items = attrs.get_attr("class").unwrap();
                    let stuff: Value = items
                        .try_iter()
                        .unwrap()
                        // .flat_map(|s| s.get_attr("spans"))
                        .flat_map(|t| t.get_attr("text"))
                        .collect();
                    let out_string: String = stuff
                        .try_iter()
                        .unwrap()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    classes.push(out_string.clone());
                    // dbg!(&out_string);
                    // let

                    // let flattened = flatten_spans_from_template_v42(&[stuff])
                    //     .unwrap()
                    //     .to_string();
                    // let escaped_value = encode_quoted_attribute(&flattened);
                    //classes.push(escaped_value.to_string());

                    // let spans = attrs
                    //     .get_item(&key)?
                    //     .get_item(&Value::from("spans"))
                    //     .unwrap();
                    // let flattened = flatten_spans_from_template_v42(&[spans])
                    //     .unwrap()
                    //     .to_string();
                    // let escaped_value = encode_quoted_attribute(&flattened);
                    // classes.push(escaped_value.to_string());

                    //
                }

                if key.to_string() != "class".to_string() {
                    // let spans = attrs
                    //     .get_item(&key)?
                    //     .get_item(&Value::from("spans"))
                    //     .unwrap();
                    // let flattened = flatten_spans_from_template_v42(&[spans])
                    //     .unwrap()
                    //     .to_string();
                    // let escaped_value = encode_quoted_attribute(&flattened);
                    // attr_collection.insert(key.to_string(), vec![escaped_value.to_string()]);
                }
            }
            if args.len() >= 2 {
                if &args[1].to_string() != "" {
                    classes.push(args[1].to_string());
                }
            };
            if classes.len() > 0 {
                attr_collection.insert("class".to_string(), vec![classes.join(" ").to_string()]);
            }
            let attr_items = attr_collection
                .iter()
                .map(|(k, v)| format!(r#"{}="{}""#, k.to_string(), v.join(" ")))
                .collect::<Vec<String>>();
            if attr_items.len() > 0 {
                Ok(Value::from(format!(r#" {}"#, attr_items.join(" "))))
            } else {
                Ok(Value::from(""))
            }
        } else {
            // TODO: Hoist an error here that there wasn't
            // enough arguments passed to the method
            Ok(Value::from(""))
        }
    }

    // // TODO: Delte this it's the backup version
    // pub fn get_section_attrs(&self, args: &[Value]) -> Result<Value, Error> {
    //     let mut attr_collection: BTreeMap<String, Vec<String>> = BTreeMap::new();
    //     let mut classes = vec![];
    //     if args.len() >= 1 {
    //         let kind = args[0].get_attr("kind").unwrap();
    //         classes.push(format!("{}-section", kind));
    //         let attrs = args[0].get_item(&Value::from("attrs")).unwrap();
    //         for key in attrs.try_iter()? {
    //             if key.to_string() != "class".to_string() {
    //                 let spans = attrs
    //                     .get_item(&key)?
    //                     .get_item(&Value::from("spans"))
    //                     .unwrap();
    //                 let flattened = flatten_spans_from_template_v42(&[spans])
    //                     .unwrap()
    //                     .to_string();
    //                 let escaped_value = encode_quoted_attribute(&flattened);
    //                 attr_collection.insert(key.to_string(), vec![escaped_value.to_string()]);
    //             }
    //             if key.to_string() == "class".to_string() {
    //                 let spans = attrs
    //                     .get_item(&key)?
    //                     .get_item(&Value::from("spans"))
    //                     .unwrap();
    //                 let flattened = flatten_spans_from_template_v42(&[spans])
    //                     .unwrap()
    //                     .to_string();
    //                 let escaped_value = encode_quoted_attribute(&flattened);
    //                 classes.push(escaped_value.to_string());
    //             }
    //         }
    //         if args.len() >= 2 {
    //             if &args[1].to_string() != "" {
    //                 classes.push(args[1].to_string());
    //             }
    //         };
    //         if classes.len() > 0 {
    //             attr_collection.insert("class".to_string(), vec![classes.join(" ").to_string()]);
    //         }
    //         let attr_items = attr_collection
    //             .iter()
    //             .map(|(k, v)| format!(r#"{}="{}""#, k.to_string(), v.join(" ")))
    //             .collect::<Vec<String>>();
    //         if attr_items.len() > 0 {
    //             Ok(Value::from(format!(r#" {}"#, attr_items.join(" "))))
    //         } else {
    //             Ok(Value::from(""))
    //         }
    //     } else {
    //         // TODO: Hoist an error here that there wasn't
    //         // enough arguments passed to the method
    //         Ok(Value::from(""))
    //     }
    // }

    // get_span_attrs() used to get the attrs from a span and
    // return them with a possibly added class that comes in
    // as a second argument.
    // TODO: capture flags as well
    // TODO: hoist errors
    // TODO: only hoist attrs defined in the HOIST setting
    pub fn get_span_attrs(&self, args: &[Value]) -> Result<Value, Error> {
        let mut attr_collection: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut classes = vec![];
        if args.len() >= 1 {
            let kind = args[0].get_attr("kind").unwrap();
            classes.push(format!("{}-span", kind));
            let attrs = args[0].get_item(&Value::from("attrs")).unwrap();
            for key in attrs.try_iter()? {
                if key.to_string() != "class".to_string() {
                    let spans = attrs
                        .get_item(&key)?
                        .get_item(&Value::from("spans"))
                        .unwrap();
                    let flattened = flatten_spans_from_template_v42(&[spans])
                        .unwrap()
                        .to_string();
                    let escaped_value = encode_quoted_attribute(&flattened);
                    attr_collection.insert(key.to_string(), vec![escaped_value.to_string()]);
                }
                if key.to_string() == "class".to_string() {
                    let spans = attrs
                        .get_item(&key)?
                        .get_item(&Value::from("spans"))
                        .unwrap();
                    let flattened = flatten_spans_from_template_v42(&[spans])
                        .unwrap()
                        .to_string();
                    let escaped_value = encode_quoted_attribute(&flattened);
                    classes.push(escaped_value.to_string());
                }
            }
            if args.len() >= 2 {
                if &args[1].to_string() != "" {
                    classes.push(args[1].to_string());
                }
            };
            if classes.len() > 0 {
                attr_collection.insert("class".to_string(), vec![classes.join(" ").to_string()]);
            };
            let attr_items = attr_collection
                .iter()
                .map(|(k, v)| format!(r#"{}="{}""#, k.to_string(), v.join(" ")))
                .collect::<Vec<String>>();
            if attr_items.len() > 0 {
                Ok(Value::from(format!(r#" {}"#, attr_items.join(" "))))
            } else {
                Ok(Value::from(""))
            }
        } else {
            // TODO: Hoist an error here that there wasn't
            // enough arguments passed to the method
            Ok(Value::from(""))
        }

        //Ok(Value::from(r#" class="red""#))

        //
    }

    // TODO: Deprecate this and move error stuff to a builder
    // or to an independent object
    pub fn get_issue_count(&self, _args: &[Value]) -> usize {
        let mut count = 0;
        for (_, page) in &self.pages {
            if page.issues.len() > 0 {
                count += 1;
            }
        }
        count
    }

    // TODO: Takes spans inside a template (as opposed to
    // a content file) and renders them to text
    pub fn template_spans_to_text(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() >= 3 {
            let x: &String = &args[0]
                .try_iter()?
                .map(|v| v.get_item(&Value::from("text")).unwrap().to_string())
                .collect::<Vec<_>>()
                .join("");
            Ok(Value::from(x))
        } else {
            Err(Error::new(ErrorKind::InvalidOperation, "".to_string()))
        }
    }

    //
}

impl Object for SiteV42b {
    // TODO: Probably don't use this. Try to keep
    // everything as method calls so the API is
    // always site.something() with the parens at
    // the end
    // fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
    //     match key.as_str()? {
    //         _ => None,
    //     }
    // }

    fn call_method(
        self: &Arc<SiteV42b>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "generated_at" => self.generated_at(args),
            "get_keys_for_pages_without_ids" => self.get_keys_for_pages_without_ids(args),
            "get_issue_count" => Ok(Value::from(self.get_issue_count(args))),
            // Deprecate the ones of these that go to a page
            // in favor of pulling in the page as an object
            // and accessing things that way to start
            "get_page_from_span" => self.get_page_from_span(args),
            //"get_page_id" => self.get_page_id(args),
            "get_page_ids" => self.get_page_ids(args),
            "get_page_with_id" => self.get_page_with_id(args),
            "get_page_with_display_path" => self.get_page_with_display_path(args),
            "get_section_attrs" => self.get_section_attrs(args),
            "get_span_attrs" => self.get_span_attrs(args),
            "href_for_page" => Ok(Value::from(self.href_for_page(args))),
            "site_root" => Ok(Value::from(self.config.site_root.clone())),
            "neo_files_href" => Ok(Value::from(self.neo_files_href())),
            "theme_folder_href" => Ok(Value::from(self.theme_folder_href())),
            "title_for_page" => Ok(Value::from(self.title_for_page(args))),
            _ => Ok(Value::from(format!(
                "[Warning: tried to use 'site.{}(...)' which does not exist]",
                name
            ))),
        }
    }
}
