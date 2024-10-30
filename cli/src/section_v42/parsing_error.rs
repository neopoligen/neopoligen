use crate::bound::Bound;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::SectionV42;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ParsingErrorSectionV42 {
    pub attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42>,
    pub bound: Bound,
    pub children: Vec<SectionV42>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
    pub line: usize,
    pub column: usize,
    pub output_remainder: String,
    pub output_remainder_with_padding: String,
    pub output_source: String,
    pub raw_remainder: String,
    pub raw_source: String,
    pub message: String,
}
