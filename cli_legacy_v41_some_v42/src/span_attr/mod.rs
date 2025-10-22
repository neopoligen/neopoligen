use crate::span::Span;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpanAttr {
    pub kind: SpanAttrKind,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SpanAttrKind {
    KeyValue { key: String, value: String },
    KeyValueSpansV42 { key: Vec<Span>, value: Vec<Span> },
    Flag { value: String },
    FlagV42 { value: Vec<Span> },
}

// TODO: implement something like .flag(), .key() and, .value()
// that returns the flattened value for the
// corresponding item
//
