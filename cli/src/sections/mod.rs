use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Sections {
    pub basic: Vec<String>,
    pub checklist: Vec<String>,
    pub comment: Vec<String>,
    pub detail: Vec<String>,
    pub json: Vec<String>,
    pub list: Vec<String>,
    pub raw: Vec<String>,
    pub table: Vec<String>,
    pub yaml: Vec<String>,
}
