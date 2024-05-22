use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;

pub fn code_shorthand<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, _) = tag("``").context("").parse(source)?;
    let (source, spans) = many1(|src| span_finder(src, spans))
        .context("")
        .parse(source)?;
    let (source, raw_attrs) = many0(alt((
        code_shorthand_key_value_attr,
        code_shorthand_flag_attr,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = tag("``").context("").parse(source)?;
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
            attrs,
            flags,
            spans,
            r#type: "code".to_string(),
        },
    ))
}

pub fn code_shorthand_key_value_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, key) = is_not(" |\n\t:`").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, value) = is_not("|`").context("").parse(source)?;
    Ok((
        source,
        SpanAttr::KeyValue {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        },
    ))
}

pub fn code_shorthand_flag_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, key) = is_not(" |\n\t:`").context("").parse(source)?;
    Ok((
        source,
        SpanAttr::Flag {
            key: key.trim().to_string(),
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
        let source = "``alfa``";
        let attrs = BTreeMap::new();
        let flags = vec![];
        let left = Span::KnownSpan {
            attrs,
            flags,
            spans: vec![Span::WordPart {
                text: "alfa".to_string(),
                r#type: "wordpart".to_string(),
            }],
            r#type: "code".to_string(),
        };
        let right = (|src| code_shorthand(src, &config.spans))(source)
            .unwrap()
            .1;
        assert_eq!(left, right);
    }

    #[test]
    fn with_attr() {
        let config = SiteConfig::mock1();
        let source = "``alfa|class: bravo``";
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
            r#type: "code".to_string(),
        };
        let right = (|src| code_shorthand(src, &config.spans))(source)
            .unwrap()
            .1;
        assert_eq!(left, right);
    }

    #[test]
    fn with_flag() {
        let config = SiteConfig::mock1();
        let source = "``alfa|some-flag``";
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
            r#type: "code".to_string(),
        };
        let right = (|src| code_shorthand(src, &config.spans))(source)
            .unwrap()
            .1;
        assert_eq!(left, right);
    }

    //
}
