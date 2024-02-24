pub mod new;

use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub struct PageData {
    pub filters: Vec<String>,
    pub tags: Vec<String>,
    pub title_for_url: Option<String>,
    pub full_title: Option<String>,
    pub url_path: Option<String>,
}
