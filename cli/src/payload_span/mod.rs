use crate::span::SpanKind;
use crate::{payload_span_attr::PayloadSpanAttr, span::Span};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSpan {
    pub attrs: Vec<PayloadSpanAttr>,
    pub classes: Vec<String>,
    pub flags: Vec<String>,
    pub kind: String,
    pub parsed_text: String,
    pub source_text: String,
    pub template_list: Vec<String>,
}

impl PayloadSpan {
    pub fn new_from_span(span: &Span) -> PayloadSpan {
        let kind = match &span.kind {
            SpanKind::CodeShorthand => "codeshorthand".to_string(),
            SpanKind::Colon => "colon".to_string(),
            SpanKind::ColonNotFollowedBySpace => "colonnotfollowedbyspace".to_string(),
            SpanKind::EscapedBackslash => "escapedbackslash".to_string(),
            SpanKind::EscapedBacktick => "escapedbacktick".to_string(),
            SpanKind::EscapedColon => "escapedcolon".to_string(),
            SpanKind::EscapedGreaterThan => "escapedgreaterthan".to_string(),
            SpanKind::EscapedPipe => "escapedpipe".to_string(),
            SpanKind::LinkShorthand => "linkshorthand".to_string(),
            SpanKind::Newline => "newline".to_string(),
            SpanKind::SingleBacktick => "singlebacktick".to_string(),
            SpanKind::SingleGreaterThan => "singlegreaterthan".to_string(),
            SpanKind::Space => "space".to_string(),
            SpanKind::WordPart => "wordpart".to_string(),
            SpanKind::NamedSpan { name } => name.to_string(),
        };
        PayloadSpan {
            attrs: vec![],
            classes: vec![],
            flags: vec![],
            kind: kind.clone(),
            parsed_text: span.parsed_text.clone().to_string(),
            source_text: span.source_text.clone().to_string(),
            template_list: vec![
                format!("spans/{}.neoj", kind.clone()),
                format!("spans/generic.neoj"),
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn basic_check() {
        let payload_span = PayloadSpan::new_from_span(&Span::mock1_basic_wordpard());
        let left = "alfa";
        let right = payload_span.parsed_text;
        assert_eq!(left, right);
    }

    #[test]
    fn template_list_check() {
        let payload_span = PayloadSpan::new_from_span(&Span::mock1_basic_wordpard());
        let left = vec![
            "spans/wordpart.neoj".to_string(),
            "spans/generic.neoj".to_string(),
        ];
        let right = payload_span.template_list;
        assert_eq!(left, right);
    }
}
