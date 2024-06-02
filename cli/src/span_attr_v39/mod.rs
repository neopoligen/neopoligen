pub mod object;

use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpanAttrV39 {
    pub source_text: String,
    pub kind: SpanAttrV39Kind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SpanAttrV39Kind {
    KeyValue { key: String, value: String },
    Flag { value: String },
}

impl SpanAttrV39 {
    pub fn key(&self) -> Option<String> {
        match &self.kind {
            SpanAttrV39Kind::KeyValue { key, .. } => Some(key.to_string()),
            _ => None,
        }
    }

    pub fn value(&self) -> Option<String> {
        match &self.kind {
            SpanAttrV39Kind::KeyValue { value, .. } => Some(value.to_string()),
            _ => None,
        }
    }
}
