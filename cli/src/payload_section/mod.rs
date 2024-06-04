use serde::{Deserialize, Serialize};

use crate::{
    payload_section_attr::PayloadSectionAttr,
    section::{SectionBounds, SectionKind},
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSection {
    pub attrs: Vec<PayloadSectionAttr>,
    pub bounds: SectionBounds,
    pub kind: SectionKind,
    pub r#type: String,
    pub template_list: Vec<String>,
}
