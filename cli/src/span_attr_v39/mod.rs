use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpanAttrV39 {
    // TODO: Move source text up here
    pub kind: SpanAttrV39Kind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SpanAttrV39Kind {
    KeyValue {
        source_text: String,
        key: String,
        value: String,
    },
    Flag {
        source_text: String,
        key: String,
    },
}
