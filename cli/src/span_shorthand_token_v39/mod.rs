use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpanShorthandTokenV39 {
    pub kind: SpanShorthandTokenV39Kind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SpanShorthandTokenV39Kind {
    EscapedBacktick {
        source_text: String,
        parsed_text: String,
    },
    EscapedPipe {
        source_text: String,
        parsed_text: String,
    },
    EscapedSlash {
        source_text: String,
        parsed_text: String,
    },
    WordPart {
        source_text: String,
        parsed_text: String,
    },
}
