use crate::{payload_span_attr::PayloadSpanAttr, span::Span};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSpan {
    pub attrs: Vec<PayloadSpanAttr>,
    pub flags: Vec<String>,
    pub kind: String,
    pub parsed_text: String,
    pub source_text: String,
}

impl PayloadSpan {
    pub fn new_from_span(span: &Span) -> PayloadSpan {
        PayloadSpan {
            attrs: vec![],
            flags: vec![],
            kind: "wordpart".to_string(),
            parsed_text: "x".to_string(),
            source_text: "c".to_string(),
        }
    }
}
