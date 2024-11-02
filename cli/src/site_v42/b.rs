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
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SiteV42b {
    pub config: SiteConfigV42,
    pub pages: BTreeMap<String, PageV42b>,
    pub generated_at: Option<DateTime<Local>>,
}

impl SiteV42b {
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
