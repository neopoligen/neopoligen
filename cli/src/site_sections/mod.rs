pub mod mock;

use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SiteSections {
    pub basic: Vec<String>,
    pub checklist: Vec<String>,
    pub json: Vec<String>,
    pub list: Vec<String>,
    pub raw: Vec<String>,
}
