#![allow(unused_imports)]
use crate::helpers::flatten_spans_in_template_v42;
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
                    pages_without_ids.insert(page.source_path.clone(), page.clone());
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
}

impl Object for SiteV42b {
    // fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
    //     match key.as_str()? {
    //         "sections" => self.sections.clone(),
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
            _ => Ok(Value::from("")),
        }
    }
}
