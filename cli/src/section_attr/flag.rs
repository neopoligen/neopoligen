use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionFlagAttrV42 {
    value: String,
}
