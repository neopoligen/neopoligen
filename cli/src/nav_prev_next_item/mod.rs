use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavPrevNextItem {
    /*
    pub href: Option<String>,
    pub menu_title: Option<String>,
    pub menu_title_link_or_text: Option<String>,
    pub prev_next_title: Option<String>,
    pub prev_next_title_link_or_text: Option<String>,
    pub title: Option<String>,
    */
    pub page_id: String,
    pub title_link_or_text: Option<String>,
}
