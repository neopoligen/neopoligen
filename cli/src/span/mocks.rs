use crate::span::Span;
use crate::span::SpanKind;

impl Span {
    pub fn mock1_basic_wordpard() -> Span {
        Span {
            attrs: vec![],
            kind: SpanKind::WordPart,
            parsed_text: "alfa".to_string(),
            source_text: "alfa".to_string(),
        }
    }
}
