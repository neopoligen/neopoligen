use crate::helpers::flatten_spans_in_template_v42;
use crate::helpers::highlight_span_v42;
use crate::page_v42::PageV42b;
use crate::site_config::SiteConfigV42;
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
}
