use crate::span_v42::SpanV42;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionKeyValueAttrPayloadV42 {
    pub spans: Vec<SpanV42>,
}
