use serde::Deserialize;
use serde::Serialize;

use crate::span_v42::SpanV42;

// TODO: Make span_key_value_attr_key

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SpanBaseAttrV42 {
    Flag(SpanFlagAttrV42),
    KeyValue { key: String, spans: Vec<SpanV42> },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpanFlagAttrV42 {
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpanKeyValueAttrV42 {
    pub key: String,
    pub spans: Vec<SpanV42>,
}
