use crate::span::Span;
use crate::span::SpanKind;
use crate::span_attr::SpanAttrKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSpan {
    pub attrs: BTreeMap<String, String>,
    pub classes: Vec<String>,
    pub class_string: Option<String>, // TODO: output ``class="the classes"`` as an entire string
    pub first_flag: Option<String>,
    pub flags: Vec<String>,
    pub id: Option<String>,
    pub id_string: Option<String>,
    pub kind: String,
    pub parsed_text: String,
    pub source_text: String,
    pub template_list: Vec<String>,
}

impl PayloadSpan {
    pub fn new_from_span(span: &Span) -> PayloadSpan {
        let mut attrs: BTreeMap<String, String> = BTreeMap::new();
        span.attrs.iter().for_each(|attr| match &attr.kind {
            SpanAttrKind::KeyValue { key, value } => {
                if key.ne("class") && key.ne("id") && !key.starts_with("data-") {
                    attrs.insert(key.to_string(), value.to_string());
                }
            }
            _ => {}
        });
        let flags = span
            .attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SpanAttrKind::Flag { value } => Some(value.to_string()),
                _ => None,
            })
            .collect::<Vec<String>>();
        let first_flag = if flags.len() > 0 {
            Some(flags[0].clone())
        } else {
            None
        };
        let id = span.attrs.iter().find_map(|attr| match &attr.kind {
            SpanAttrKind::KeyValue { key, value } => {
                if key.as_str() == "id" {
                    Some(value.clone())
                } else {
                    None
                }
            }
            _ => None,
        });
        let id_string = span.attrs.iter().find_map(|attr| match &attr.kind {
            SpanAttrKind::KeyValue { key, value } => {
                if key.as_str() == "id" {
                    Some(format!(r#"id="{}""#, value.clone()))
                } else {
                    None
                }
            }
            _ => None,
        });
        let kind = match &span.kind {
            SpanKind::CodeShorthand => "codeshorthand".to_string(),
            SpanKind::Colon => "colon".to_string(),
            SpanKind::ColonNotFollowedBySpace => "colonnotfollowedbyspace".to_string(),
            SpanKind::EscapedBackslash => "escapedbackslash".to_string(),
            SpanKind::EscapedBacktick => "escapedbacktick".to_string(),
            SpanKind::EscapedColon => "escapedcolon".to_string(),
            SpanKind::EscapedGreaterThan => "escapedgreaterthan".to_string(),
            SpanKind::EscapedPipe => "escapedpipe".to_string(),
            SpanKind::GreaterThan => "greaterthan".to_string(),
            SpanKind::Hyphen => "hyphen".to_string(),
            SpanKind::LessThan => "lessthan".to_string(),
            SpanKind::LinkShorthand => "linkshorthand".to_string(),
            SpanKind::Newline => "newline".to_string(),
            SpanKind::NonEscapeBackslash => "nonescapebackslash".to_string(),
            SpanKind::SingleBacktick => "singlebacktick".to_string(),
            SpanKind::SingleGreaterThan => "singlegreaterthan".to_string(),
            SpanKind::SingleLessThan => "singlelessthan".to_string(),
            SpanKind::Space => "space".to_string(),
            SpanKind::WordPart => "wordpart".to_string(),
            SpanKind::NamedSpan { r#type, .. } => r#type.to_string(),
            SpanKind::Pipe => "pipe".to_string(),
        };

        PayloadSpan {
            attrs,
            classes: vec![],    // TODO
            class_string: None, // TODO
            first_flag,
            flags,
            id,
            id_string,
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
    fn attrs_check() {
        let ps = PayloadSpan::new_from_span(&Span::mock2_named_link_with_flag_and_attrs());
        assert_eq!("nofollow", ps.attrs.get("rel").unwrap());
    }

    #[test]
    fn basic_check() {
        let payload_span = PayloadSpan::new_from_span(&Span::mock1_basic_wordpard());
        let left = "alfa";
        let right = payload_span.parsed_text;
        assert_eq!(left, right);
    }

    #[test]
    fn flags_check() {
        let ps = PayloadSpan::new_from_span(&Span::mock2_named_link_with_flag_and_attrs());
        assert_eq!(vec!["https://www.example.com/".to_string()], ps.flags);
    }

    #[test]
    fn first_flag_check() {
        let ps = PayloadSpan::new_from_span(&Span::mock2_named_link_with_flag_and_attrs());
        assert_eq!("https://www.example.com/", ps.first_flag.unwrap());
    }

    #[test]
    fn id_check() {
        let ps = PayloadSpan::new_from_span(&Span::mock2_named_link_with_flag_and_attrs());
        assert_eq!("bravo", ps.id.unwrap());
        assert_eq!(r#"id="bravo""#, ps.id_string.unwrap());
    }

    #[test]
    fn id_string_check() {
        let ps = PayloadSpan::new_from_span(&Span::mock2_named_link_with_flag_and_attrs());
        assert_eq!(r#"id="bravo""#, ps.id_string.unwrap());
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

    //
}
