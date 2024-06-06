use crate::span::Span;
use crate::span::SpanKind;
use crate::span_attr::SpanAttrKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSpan {
    // TODO: Escape everything that needs it
    pub attrs: BTreeMap<String, String>, // allowed attrs from the config (minus id, classes, data-
    pub attrs_unescaped: BTreeMap<String, String>, // allowed attrs from the config (minus id, classes, data-
    pub classes: Vec<String>,                      // combined classes
    pub classes_unescaped: Vec<String>,            // combined classes
    pub class_string: Option<String>, // TODO: output ``class="the classes"`` as an entire string
    pub custom_attrs: BTreeMap<String, String>, // any attrs that are not defined in the config
    pub custom_attrs_unescaped: BTreeMap<String, String>, // any attrs that are not defined in the config
    pub data: BTreeMap<String, String>,
    pub data_unescaped: BTreeMap<String, String>,
    pub first_flag: Option<String>,           // first flag passed in
    pub first_flag_unescaped: Option<String>, // first flag passed in
    pub flags: Vec<String>,                   // All the flags
    pub flags_unescaped: Vec<String>,         // non-html escaped versions of the flags
    pub id: Option<String>,                   //
    pub id_unescaped: Option<String>,         //
    pub id_string: Option<String>,
    pub kind: String,
    pub parsed_text: String,
    pub source_text: String,
    pub template_list: Vec<String>,
}

impl PayloadSpan {
    pub fn new_from_span(span: &Span) -> PayloadSpan {
        let mut attrs: BTreeMap<String, String> = BTreeMap::new();
        let mut attrs_unescaped: BTreeMap<String, String> = BTreeMap::new();
        span.attrs.iter().for_each(|attr| match &attr.kind {
            SpanAttrKind::KeyValue { key, value } => {
                if key.ne("class") && key.ne("id") && !key.starts_with("data-") {
                    attrs.insert(key.to_string(), value.replace(r#"""#, "&quot;").to_string());
                    attrs_unescaped.insert(key.to_string(), value.to_string());
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
            attrs_unescaped,
            class_string: None,        // TODO
            classes: vec![],           // TODO
            classes_unescaped: vec![], // TODO
            custom_attrs: BTreeMap::new(),
            custom_attrs_unescaped: BTreeMap::new(),
            data: BTreeMap::new(),
            data_unescaped: BTreeMap::new(),
            first_flag,
            first_flag_unescaped: None,
            flags,
            flags_unescaped: vec![],
            id,
            id_string,
            id_unescaped: None,
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
    fn attrs_are_quote_escaped() {
        let ps = PayloadSpan::new_from_span(&Span::mock3_named_image());
        assert_eq!(
            r#"This is &quot;some quoted&quot; alt text"#,
            ps.attrs.get("alt").unwrap()
        );
    }

    #[test]
    fn attrs_unescaped_does_not_have_quotes_escaped() {
        let ps = PayloadSpan::new_from_span(&Span::mock3_named_image());
        assert_eq!(
            r#"This is "some quoted" alt text"#,
            ps.attrs_unescaped.get("alt").unwrap()
        );
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
