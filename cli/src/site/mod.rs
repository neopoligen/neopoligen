use crate::page::Page;
use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pages: BTreeMap<String, Page>,
}
