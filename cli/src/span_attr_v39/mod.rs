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
