use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpanTokenV39 {
    pub kind: SpanTokenV39Kind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SpanTokenV39Kind {
    EscapedBacktick {
        source_text: String,
        parsed_text: String,
    },
    EscapedPipe {
        source_text: String,
        parsed_text: String,
    },
    WordPart {
        source_text: String,
        parsed_text: String,
    },
}
