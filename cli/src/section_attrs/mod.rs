use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionAttrs {
    Flag { key: String },
    KeyValue { key: String, value: String },
}
