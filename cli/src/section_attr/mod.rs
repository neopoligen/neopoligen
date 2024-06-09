use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};

use self::wordpart::wordpart;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionAttr {
    pub kind: SectionAttrKind,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SectionAttrKind {
    KeyValue { key: String, value: String },
    KeyValueSpans { key: String, spans: Vec<Span> },
    Flag { flag: String },
}

pub fn section_attr_old<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    let (source, attr) = alt((section_key_value_attr_39, section_flag_attr))(source)?;
    Ok((source, attr))
}

pub fn section_attr<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    let (source, attr) = alt((section_key_value_attr, section_flag_attr))(source)?;
    Ok((source, attr))
}

pub fn section_key_value_attr<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    let (source, _) = tag("--").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, key) = is_not(": \n").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    // TODO: Add all the different span types that are allowed here
    let (source, spans) = many1(alt((wordpart, space))).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    Ok((
        source,
        SectionAttr {
            kind: SectionAttrKind::KeyValueSpans {
                key: key.trim().to_string(),
                spans,
            },
        },
    ))
}

// DEPRECATED: Replace with keyvaluespans version
pub fn section_key_value_attr_39<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    let (source, _) = tag("--").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, key) = is_not(": \n").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, value) = not_line_ending.context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    Ok((
        source,
        SectionAttr {
            kind: SectionAttrKind::KeyValue {
                key: key.trim().to_string(),
                value: value.trim().to_string(),
            },
        },
    ))
}

pub fn section_flag_attr<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    let (source, _) = tag("--").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, key) = is_not(":\n").context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    Ok((
        source,
        SectionAttr {
            kind: SectionAttrKind::Flag {
                flag: key.trim().to_string(),
            },
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn key_value_spans_basic() {
        let source = "-- key: value";
        let left = SectionAttr {
            kind: SectionAttrKind::KeyValueSpans {
                key: "key".to_string(),
                spans: vec![Span {
                    attrs: vec![],
                    kind: SpanKind::WordPart {},
                    parsed_text: "value".to_string(),
                    source_text: "value".to_string(),
                }],
            },
        };
        let right = section_attr(source).unwrap().1;
        assert_eq!(left, right);
    }
}
