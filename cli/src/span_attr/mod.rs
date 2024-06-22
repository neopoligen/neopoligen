use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpanAttr {
    pub kind: SpanAttrKind,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SpanAttrKind {
    KeyValue { key: String, value: String },
    Flag { value: String },
}
