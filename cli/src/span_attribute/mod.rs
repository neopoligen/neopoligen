use serde::Serialize;

// These might be deprecated, need to verify

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(content = "content", rename_all = "lowercase", tag = "type")]
pub enum TagAttribute {
    Boolean { key: String },
    KeyValue { key: String, value: String }
}