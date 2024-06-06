use crate::site_config::SiteConfig;
use crate::span::Span;
use crate::span::SpanKind;
use crate::span_attr::SpanAttrKind;
use html_escape;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSpan {
    ///
    /// TODO: aria-*
    ///
    ///
    /// TODO: attr_string
    ///
    /// TODO: The full output string for all attributes
    /// with their key/value pairs used as ``[@ span.attrs_string @]``
    /// which outputs both key/value attrs and flags
    /// (e.g. ``<<button|some text|form: some_form|hidden>>`` outputs:
    /// ``form="some_form" hidden"),
    ///
    /// TODO: Include output for class, id, and data too.
    /// i.e. make this the single thing that can be called in
    /// most circumstances
    pub attr_string: Option<String>,
    ///
    /// IN_PROGRESS: attrs
    ///
    /// key/value attributes (i.e. not flags)
    ///
    /// TODO: pull in the attributes
    /// config file to clear list them.
    ///
    /// Does not include: class, data, or id. Those are handled
    /// independently.
    ///
    /// TODO: Multiple entires with the same key are combined into
    /// a single output here with their values separated by
    /// a space
    ///
    /// TODO: Characters are HTML escaped. TODO: List the
    /// specific characters
    ///
    /// TODO: Figure out if other HTML elements should be
    /// escaped
    pub attrs: BTreeMap<String, String>,
    ///
    /// IN_PROGRESS: attrs_unescaped
    ///
    /// Same as ``attrs`` above, but the HTML characters are
    /// not escaped
    pub attrs_unescaped: BTreeMap<String, String>,
    ///
    /// NEEDS_DOCS: classes
    pub classes: Vec<String>, // combined classes
    pub classes_unescaped: Vec<String>, // combined classes
    ///
    /// TODO: custom_attrs
    ///
    /// Any key/value attributes that aren't defined in the
    /// config file. Quotes are escaped into ``&quot;``
    pub custom_attrs: BTreeMap<String, String>,
    ///
    /// TODO: custom_attrs_unescaped
    ///
    /// Same as custom_attrs, but quotes are not
    /// escaped into ``&quot;``
    pub custom_attrs_unescaped: BTreeMap<String, String>,
    ///
    /// TODO: data
    pub data: BTreeMap<String, String>,
    ///
    /// TODO: data_unescaped
    pub data_unescaped: BTreeMap<String, String>,
    ///
    /// NEEDS_DOCS: first_flag
    pub first_flag: Option<String>,
    ///
    /// TODO: first_flag_unescaped
    pub first_flag_unescaped: Option<String>,
    ///
    /// IN_PROGRESS: flags
    ///
    /// TODO: Add escaping
    pub flags: Vec<String>, // All the flags
    ///
    /// TODO: flats_unescaped
    pub flags_unescaped: Vec<String>,
    ///
    /// NEEDS_DOCS: id
    pub id: Option<String>,
    /// TODO: id
    pub id_unescaped: Option<String>, //
    ///
    /// NEEDS_DOCS: kind
    pub kind: String,
    ///
    /// NEEDS_DOCS: parsed_text
    pub parsed_text: String,
    ///
    /// NEEDS_DOCS: source_text
    pub source_text: String,
    ///
    /// NEEDS_DOCS: template_list
    pub template_list: Vec<String>,
}

impl PayloadSpan {
    pub fn new_from_span(span: &Span, config: &SiteConfig) -> PayloadSpan {
        let mut attrs: BTreeMap<String, String> = BTreeMap::new();
        let mut attrs_unescaped: BTreeMap<String, String> = BTreeMap::new();
        span.attrs.iter().for_each(|attr| match &attr.kind {
            SpanAttrKind::KeyValue { key, value } => {
                if key.ne("class")
                    && key.ne("id")
                    && !key.starts_with("data-")
                    && config.span_attrs.contains(key)
                {
                    attrs.insert(
                        key.to_string(),
                        html_escape::encode_double_quoted_attribute(value).to_string(),
                    );
                    attrs_unescaped.insert(key.to_string(), value.to_string());
                }
            }
            _ => {}
        });
        let mut classes: Vec<String> = vec![];
        let mut classes_unescaped: Vec<String> = vec![];
        span.attrs.iter().for_each(|attr| match &attr.kind {
            SpanAttrKind::KeyValue { key, value } => {
                if key.eq("class") {
                    let parts = value.split(" ").collect::<Vec<&str>>();
                    parts.iter().for_each(|part| {
                        classes.push(html_escape::encode_double_quoted_attribute(part).to_string());
                        classes_unescaped.push(part.to_string());
                    });
                }
            }
            _ => {}
        });
        let mut custom_attrs: BTreeMap<String, String> = BTreeMap::new();
        let mut custom_attrs_unescaped: BTreeMap<String, String> = BTreeMap::new();
        span.attrs.iter().for_each(|attr| match &attr.kind {
            SpanAttrKind::KeyValue { key, value } => {
                if key.ne("class")
                    && key.ne("id")
                    && !key.starts_with("data-")
                    && !config.span_attrs.contains(key)
                {
                    custom_attrs.insert(
                        key.to_string(),
                        html_escape::encode_double_quoted_attribute(value).to_string(),
                    );
                    custom_attrs_unescaped.insert(key.to_string(), value.to_string());
                }
            }
            _ => {}
        });
        let mut data: BTreeMap<String, String> = BTreeMap::new();
        let mut data_unescaped: BTreeMap<String, String> = BTreeMap::new();
        span.attrs.iter().for_each(|attr| match &attr.kind {
            SpanAttrKind::KeyValue { key, value } => {
                if key.starts_with("data-") {
                    let key_parts = key.split_once("-").unwrap();
                    data.insert(
                        key_parts.1.to_string(),
                        html_escape::encode_double_quoted_attribute(value).to_string(),
                    );
                    data_unescaped.insert(key_parts.1.to_string(), value.to_string());
                }
            }
            _ => {}
        });
        let flags = span
            .attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SpanAttrKind::Flag { value } => {
                    Some(html_escape::encode_double_quoted_attribute(value).to_string())
                }
                _ => None,
            })
            .collect::<Vec<String>>();
        let flags_unescaped = span
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
        let first_flag_unescaped = if flags_unescaped.len() > 0 {
            Some(flags_unescaped[0].clone())
        } else {
            None
        };
        let id = span.attrs.iter().find_map(|attr| match &attr.kind {
            SpanAttrKind::KeyValue { key, value } => {
                if key.as_str() == "id" {
                    Some(html_escape::encode_double_quoted_attribute(value).to_string())
                } else {
                    None
                }
            }
            _ => None,
        });
        let id_unescaped = span.attrs.iter().find_map(|attr| match &attr.kind {
            SpanAttrKind::KeyValue { key, value } => {
                if key.as_str() == "id" {
                    Some(value.to_string())
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
            attr_string: None, // TODO
            attrs,
            attrs_unescaped,
            classes,
            classes_unescaped,
            custom_attrs,
            custom_attrs_unescaped,
            data,
            data_unescaped,
            first_flag,
            first_flag_unescaped,
            flags,
            flags_unescaped,
            id,
            id_unescaped,
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
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock2_named_link_with_flag_and_attrs(), &config);
        assert_eq!("nofollow", ps.attrs.get("rel").unwrap());
    }

    #[test]
    fn attrs_are_quote_escaped() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock3_named_image(), &config);
        assert_eq!(
            r#"This is &quot;some quoted&quot; alt text"#,
            ps.attrs.get("alt").unwrap()
        );
    }

    #[test]
    fn attrs_unescaped_does_not_have_quotes_escaped() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock3_named_image(), &config);
        assert_eq!(
            r#"This is "some quoted" alt text"#,
            ps.attrs_unescaped.get("alt").unwrap()
        );
    }

    #[test]
    fn basic_check() {
        let config = SiteConfig::mock1_basic();
        let payload_span = PayloadSpan::new_from_span(&Span::mock1_basic_wordpard(), &config);
        let left = "alfa";
        let right = payload_span.parsed_text;
        assert_eq!(left, right);
    }

    #[test]
    fn classes_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock4_class_test(), &config);
        assert_eq!(
            vec![
                "alfa".to_string(),
                "bravo".to_string(),
                "cha&quot;rlie".to_string()
            ],
            ps.classes
        );
    }

    #[test]
    fn custom_attrs_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock2_named_link_with_flag_and_attrs(), &config);
        assert_eq!(
            r#"custom&quot;value"#,
            ps.custom_attrs.get("custom-key").unwrap()
        );
    }

    #[test]
    fn custom_unescaped_attrs_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock2_named_link_with_flag_and_attrs(), &config);
        assert_eq!(
            r#"custom"value"#,
            ps.custom_attrs_unescaped.get("custom-key").unwrap()
        );
    }

    #[test]
    fn data_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock2_named_link_with_flag_and_attrs(), &config);
        assert_eq!(r#"bra&quot;vo"#, ps.data.get("ping").unwrap());
    }

    #[test]
    fn data_unescpaed_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock2_named_link_with_flag_and_attrs(), &config);
        assert_eq!(r#"bra"vo"#, ps.data_unescaped.get("ping").unwrap());
    }

    #[test]
    fn flags_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock5_flag_with_quote_in_it(), &config);
        assert_eq!(vec!["fox&quot;trot".to_string()], ps.flags);
    }

    #[test]
    fn flags_unescaped_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock5_flag_with_quote_in_it(), &config);
        assert_eq!(vec![r#"fox"trot"#.to_string()], ps.flags_unescaped);
    }

    #[test]
    fn first_flag_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock5_flag_with_quote_in_it(), &config);
        assert_eq!("fox&quot;trot", ps.first_flag.unwrap());
    }

    #[test]
    fn first_flag_unescaped_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock5_flag_with_quote_in_it(), &config);
        assert_eq!(r#"fox"trot"#, ps.first_flag_unescaped.unwrap());
    }

    #[test]
    fn id_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock6_id_with_qutoe_in_t(), &config);
        assert_eq!(r#"fox&quot;trot"#, ps.id.unwrap());
    }

    #[test]
    fn id_unescaped_check() {
        let config = SiteConfig::mock1_basic();
        let ps = PayloadSpan::new_from_span(&Span::mock6_id_with_qutoe_in_t(), &config);
        assert_eq!(r#"fox"trot"#, ps.id_unescaped.unwrap());
    }

    #[test]
    fn template_list_check() {
        let config = SiteConfig::mock1_basic();
        let payload_span = PayloadSpan::new_from_span(&Span::mock1_basic_wordpard(), &config);
        let left = vec![
            "spans/wordpart.neoj".to_string(),
            "spans/generic.neoj".to_string(),
        ];
        let right = payload_span.template_list;
        assert_eq!(left, right);
    }

    //
}
