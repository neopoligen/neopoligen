use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;

pub fn known_span<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, _) = tag("<<").context("").parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, r#type) = (|src| known_span_type(src, spans))
        .context("")
        .parse(source)?;
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, spans) = many1(|src| span_finder(src, spans))
        .context("")
        .parse(source)?;
    // Make sure to look for key_value first so that ":" can
    // be used in URLs
    let (source, raw_attrs) = many0(alt((span_key_value_attr, span_flag_attr)))
        .context("")
        .parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;
    let mut flags: Vec<String> = vec![];
    let mut attrs = BTreeMap::new();
    raw_attrs.iter().for_each(|attr| match attr {
        SpanAttr::KeyValue { key, value } => {
            attrs.insert(key.to_string(), value.to_string());
        }
        SpanAttr::Flag { key } => flags.push(key.to_string()),
    });
    Ok((
        source,
        Span::KnownSpan {
            r#type: r#type.to_string(),
            spans,
            flags,
            attrs,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::site_config::SiteConfig;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_word() {
        let config = SiteConfig::mock1();
        let source = "<<span|alfa>>";
        let attrs = BTreeMap::new();
        let flags = vec![];
        let left = Span::KnownSpan {
            attrs,
            flags,
            spans: vec![Span::WordPart {
                text: "alfa".to_string(),
                r#type: "wordpart".to_string(),
            }],
            r#type: "span".to_string(),
        };
        let right = (|src| known_span(src, &config.spans))(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn with_attr() {
        let config = SiteConfig::mock1();
        let source = "<<span|alfa|class: bravo>>";
        let mut attrs = BTreeMap::new();
        attrs.insert("class".to_string(), "bravo".to_string());
        let flags = vec![];
        let left = Span::KnownSpan {
            attrs,
            flags,
            spans: vec![Span::WordPart {
                text: "alfa".to_string(),
                r#type: "wordpart".to_string(),
            }],
            r#type: "span".to_string(),
        };
        let right = (|src| known_span(src, &config.spans))(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn with_flag() {
        let config = SiteConfig::mock1();
        let source = "<<span|alfa|some-flag>>";
        let attrs = BTreeMap::new();
        let mut flags = vec![];
        flags.push("some-flag".to_string());
        let left = Span::KnownSpan {
            attrs,
            flags,
            spans: vec![Span::WordPart {
                text: "alfa".to_string(),
                r#type: "wordpart".to_string(),
            }],
            r#type: "span".to_string(),
        };
        let right = (|src| known_span(src, &config.spans))(source).unwrap().1;
        assert_eq!(left, right);
    }

    //
}
