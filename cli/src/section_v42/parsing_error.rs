use crate::bound::Bound;
use crate::section_v42::SectionV42;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ParsingErrorSectionV42 {
    pub bounds: Bound,
    pub children: Vec<SectionV42>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
    pub source: Option<String>,
}
