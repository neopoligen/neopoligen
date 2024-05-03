use crate::section_attribute::SectionAttribute;
use serde::Serialize;


#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct TextPlugin {
    pub attributes: Vec<SectionAttribute>,
    pub bounds: String,
    pub template: String,
    pub text: Option<String>,
    pub r#type: String,
}