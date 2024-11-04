#![allow(unused_imports)]
use crate::helpers::flatten_spans_in_template_v42;
use crate::helpers::flatten_spans_v42;
use crate::helpers::highlight_span_v42;
use crate::page_v42::PageV42b;
use crate::site_config::SiteConfigV42;
use chrono::prelude::*;
use html_escape::*;
use minijinja::value::Object;
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

    pub fn get_page_ids(&self, _args: &[Value]) -> Result<Value, Error> {
        Ok(Value::from_serialize(
            &self.pages.keys().cloned().collect::<Vec<String>>(),
        ))
    }

    pub fn get_page_with_id(&self, args: &[Value]) -> Result<Value, Error> {
        if args.len() > 3 {
            Ok(Value::from("TODO: Get page by id"))
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                r#"Not enough values pased to site.get_page_with_id
Needs to be 'site.get_page_with_id(site, page, ID_YOU_WANT)'"#
                    .to_string(),
            ))
        }
    }

    pub fn get_section_attrs(&self, args: &[Value]) -> Result<Value, Error> {
        let mut attr_collection: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut classes = vec![];
        if args.len() >= 3 {
            let attrs = args[2].get_item(&Value::from("attrs")).unwrap();
            for key in attrs.try_iter()? {
                if key.to_string() != "class".to_string() {
                    let spans = attrs
                        .get_item(&key)?
                        .get_item(&Value::from("spans"))
                        .unwrap();
                    let flattened = flatten_spans_in_template_v42(&[spans]).unwrap().to_string();
                    let escaped_value = encode_quoted_attribute(&flattened);
                    attr_collection.insert(key.to_string(), vec![escaped_value.to_string()]);
                }

                if key.to_string() == "class".to_string() {
                    let spans = attrs
                        .get_item(&key)?
                        .get_item(&Value::from("spans"))
                        .unwrap();
                    let flattened = flatten_spans_in_template_v42(&[spans]).unwrap().to_string();
                    let escaped_value = encode_quoted_attribute(&flattened);
                    classes.push(escaped_value.to_string());
                }

                //let spans = attrs.get_item(&key)?.get_item(&Value::from("spans"));
                //let values = spans
                //    .iter()
                //    .filter_map(|v| {
                //        if key.to_string() != "class".to_string() {
                //            dbg!(key.to_string());
                //            //let internal_spans = &v.get_item(&Value::from("spans")).unwrap();
                //            //dbg!(internal_spans);
                //            dbg!(&spans);
                //            // let thing = encode_quoted_attribute(
                //            //     &flatten_spans_in_template_v42(&[v
                //            //         .get_item(&Value::from("spans"))
                //            //         .unwrap()])
                //            //     .expect("unwrapping")
                //            //     .as_str()
                //            //     .unwrap()
                //            //     .to_string(),
                //            // )
                //            // .to_string();
                //            //dbg!(&thing);
                //            //Some(thing)
                //        } else {
                //            None
                //        }
                //    })
                //    .collect::<Vec<String>>();
                //if values.len() > 0 {
                //    attr_collection.push(format!(r#"{}="{}""#, key.to_string(), values.join(" ")));
                //}

                //
            }

            if args.len() >= 4 {
                classes.push(args[3].to_string());
            };

            // if classes.len() > 0 {
            //     Ok(Value::from(format!(r#" class="{}""#, classes.join(" "))))
            // } else {
            //     // TODO: Pull the rest of the attrs and flags here
            //     Ok(Value::from(""))
            // }

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

            // dbg!(&attr_items);
            // dbg!(&attr_collection);
            // Ok(Value::from(r#" class="TODO""#))

            //
        } else {
            // TODO: Turn this into an error and hoist it.
            Ok(Value::from(""))
        }
    }

    //// TODO: Test this, it was copied over direction from the prior version, but hopefully still
    //// works out of the box
    //// TODO: Add tests to this and error handling
    //// TODO: escape for html
    //// TODO: Set up config of specific attrs/flags to hoist
    ////       and only add those to the output
    //pub fn get_span_attrs(&self, args: &[Value]) -> Result<Value, Error> {
    //    //let mut classes: Vec<String> = vec![];
    //    let mut attr_string: Vec<String> = vec![];
    //    if args.len() > 0 {
    //        let attrs = args[0].get_item(&Value::from("attrs")).unwrap();
    //        for key in attrs.try_iter()? {
    //            let spans = attrs.get_item(&key)?;
    //            let values = spans
    //                .try_iter()?
    //                .filter_map(|v| {
    //                    if key.to_string() != "class".to_string() {
    //                        Some(
    //                            encode_quoted_attribute(
    //                                &v.get_item(&Value::from("text")).unwrap().to_string(),
    //                            )
    //                            .to_string(),
    //                        )
    //                    } else {
    //                        None
    //                    }
    //                })
    //                .collect::<Vec<String>>();
    //            if values.len() > 0 {
    //                attr_string.push(format!(r#"{}="{}""#, key.to_string(), values.join(" ")));
    //            }
    //        }
    //        let mut classes: Vec<String> = vec![];
    //        for key in attrs.try_iter()? {
    //            let spans = attrs.get_item(&key)?;
    //            classes = spans
    //                .try_iter()?
    //                .filter_map(|v| {
    //                    if key.to_string() == "class".to_string() {
    //                        Some(
    //                            encode_quoted_attribute(
    //                                &v.get_item(&Value::from("text")).unwrap().to_string(),
    //                            )
    //                            .to_string(),
    //                        )
    //                    } else {
    //                        None
    //                    }
    //                })
    //                .collect::<Vec<String>>();
    //        }
    //        if args.len() == 2 {
    //            classes.push(args[1].to_string());
    //        }
    //        if classes.len() > 0 {
    //            attr_string.push(format!(r#"class="{}""#, classes.join(" ")));
    //        }
    //        let flags = args[0].get_item(&Value::from("flags")).unwrap();
    //        for flag in flags.try_iter()? {
    //            attr_string.push(format!(r#"{}"#, flag));
    //        }
    //        Ok(Value::from(format!(" {}", attr_string.join(" "))))
    //    } else {
    //        Ok(Value::from(""))
    //    }
    //}

    //
}

impl Object for SiteV42b {
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            _ => None,
        }
    }

    fn call_method(
        self: &Arc<SiteV42b>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "generated_at" => self.generated_at(args),
            "get_section_attrs" => self.get_section_attrs(args),
            "get_page_ids" => self.get_page_ids(args),
            _ => Ok(Value::from("")),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        bound::Bound,
        section_attr_v42::SectionAttrV42,
        section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42,
        section_v42::{basic::BasicSectionV42, SectionV42},
        span_v42::{named_span::KeyValueSpanAttrV42, plaintext::PlainTextSpan, SpanV42},
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_section_attrs_empty_test() {
        let site = SiteV42b {
            config: SiteConfigV42::mock1_basic(),
            generated_at: None,
            pages: BTreeMap::new(),
            pages_without_ids: BTreeMap::new(),
        };
        let section = Value::from_serialize(BasicSectionV42 {
            attrs: BTreeMap::new(),
            bound: Bound::Full,
            children: vec![],
            end_section: None,
            flags: vec![],
            kind: "generic-basic".to_string(),
            template_list: vec![],
        });
        let expected = Value::from(r#""#);
        let got = site
            .get_section_attrs(&[Value::from(""), Value::from(""), section])
            .unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn get_section_attrs_with_id() {
        let site = SiteV42b {
            config: SiteConfigV42::mock1_basic(),
            generated_at: None,
            pages: BTreeMap::new(),
            pages_without_ids: BTreeMap::new(),
        };
        let mut attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
        attrs.insert(
            "id".to_string(),
            SectionKeyValueAttrPayloadV42 {
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "alfa".to_string(),
                })],
            },
        );
        let section = Value::from_serialize(BasicSectionV42 {
            attrs,
            bound: Bound::Full,
            children: vec![],
            end_section: None,
            flags: vec![],
            kind: "generic-basic".to_string(),
            template_list: vec![],
        });
        let left = Value::from(r#" id="alfa""#);
        let right = site
            .get_section_attrs(&[Value::from(""), Value::from(""), section])
            .unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn get_section_attrs_with_attr_class() {
        let site = SiteV42b {
            config: SiteConfigV42::mock1_basic(),
            generated_at: None,
            pages: BTreeMap::new(),
            pages_without_ids: BTreeMap::new(),
        };
        let mut attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
        attrs.insert(
            "class".to_string(),
            SectionKeyValueAttrPayloadV42 {
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "bravo".to_string(),
                })],
            },
        );
        let section = Value::from_serialize(BasicSectionV42 {
            attrs,
            bound: Bound::Full,
            children: vec![],
            end_section: None,
            flags: vec![],
            kind: "generic-basic".to_string(),
            template_list: vec![],
        });
        let left = Value::from(r#" class="bravo""#);
        let right = site
            .get_section_attrs(&[Value::from(""), Value::from(""), section])
            .unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn get_section_attrs_with_extra_class() {
        let site = SiteV42b {
            config: SiteConfigV42::mock1_basic(),
            generated_at: None,
            pages: BTreeMap::new(),
            pages_without_ids: BTreeMap::new(),
        };
        let attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
        // attrs.insert(
        //     "class".to_string(),
        //     SectionKeyValueAttrPayloadV42 {
        //         spans: vec![SpanV42::PlainText(PlainTextSpan {
        //             text: "bravo".to_string(),
        //         })],
        //     },
        // );
        let section = Value::from_serialize(BasicSectionV42 {
            attrs,
            bound: Bound::Full,
            children: vec![],
            end_section: None,
            flags: vec![],
            kind: "generic-basic".to_string(),
            template_list: vec![],
        });
        let left = Value::from(r#" class="charlie""#);
        let right = site
            .get_section_attrs(&[
                Value::from(""),
                Value::from(""),
                section,
                Value::from("charlie"),
            ])
            .unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn get_section_attrs_with_attr_class_and_extra_class_and_id() {
        let site = SiteV42b {
            config: SiteConfigV42::mock1_basic(),
            generated_at: None,
            pages: BTreeMap::new(),
            pages_without_ids: BTreeMap::new(),
        };
        let mut attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
        attrs.insert(
            "id".to_string(),
            SectionKeyValueAttrPayloadV42 {
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "delta".to_string(),
                })],
            },
        );
        attrs.insert(
            "class".to_string(),
            SectionKeyValueAttrPayloadV42 {
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "echo".to_string(),
                })],
            },
        );
        let section = Value::from_serialize(BasicSectionV42 {
            attrs,
            bound: Bound::Full,
            children: vec![],
            end_section: None,
            flags: vec![],
            kind: "generic-basic".to_string(),
            template_list: vec![],
        });
        let left = Value::from(r#" class="echo foxtrot" id="delta""#);
        let right = site
            .get_section_attrs(&[
                Value::from(""),
                Value::from(""),
                section,
                Value::from("foxtrot"),
            ])
            .unwrap();
        assert_eq!(left, right);
    }

    //
}
