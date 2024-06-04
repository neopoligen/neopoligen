use serde::{Deserialize, Serialize};

use crate::{
    section::{SectionBounds, SectionKind},
    section_attr::SectionAttr,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSection {
    pub attrs: Vec<SectionAttr>,
    pub bounds: SectionBounds,
    pub kind: SectionKind,
    pub r#type: String,
    pub template_list: Vec<String>,
}
