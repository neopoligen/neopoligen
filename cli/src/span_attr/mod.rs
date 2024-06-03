use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpanAttr {
    pub source_text: String,
    pub kind: SpanAttrKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SpanAttrKind {
    KeyValue { key: String, value: String },
    Flag { value: String },
}
