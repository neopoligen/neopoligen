use crate::span::colon::colon;
use crate::span::escaped_pipe::escaped_pipe;
use crate::span::hyphen::hyphen;
use crate::span::pipe::pipe;
use crate::span::single_underscore::single_underscore;
use crate::span::wordpart::wordpart;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};

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

// pub fn section_attr_old<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
//     let (source, attr) = alt((section_key_value_attr_39, section_flag_attr))(source)?;
//     Ok((source, attr))
// }

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
    let (source, spans) = many1(alt((
        wordpart,
        space,
        hyphen,
        colon,
        escaped_pipe,
        pipe,
        single_underscore,
    )))
    .context("")
    .parse(source)?;
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

// // DEPRECATED: Replace with keyvaluespans version
// pub fn section_key_value_attr_39<'a>(
//     source: &'a str,
// ) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
//     let (source, _) = tag("--").context("").parse(source)?;
//     let (source, _) = space1.context("").parse(source)?;
//     let (source, key) = is_not(": \n").context("").parse(source)?;
//     let (source, _) = tag(":").context("").parse(source)?;
//     let (source, value) = not_line_ending.context("").parse(source)?;
//     let (source, _) = structure_empty_until_newline_or_eof
//         .context("")
//         .parse(source)?;
//     Ok((
//         source,
//         SectionAttr {
//             kind: SectionAttrKind::KeyValue {
//                 key: key.trim().to_string(),
//                 value: value.trim().to_string(),
//             },
//         },
//     ))
// }

pub fn section_flag_attr<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    // -- https://www.example.com/
    let (source, _) = tag("--").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, parts) = many1(alt((wordpart, colon, hyphen)))
        .context("")
        .parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let flag = parts
        .iter()
        .map(|part| part.parsed_text.to_string())
        .collect::<Vec<String>>()
        .join("");
    Ok((
        source,
        SectionAttr {
            kind: SectionAttrKind::Flag { flag },
        },
    ))
}

pub fn section_flag_attr_old<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
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
    use rstest::rstest;
    #[rstest]
    // TODO: Add all the rest of the possible options here
    #[case("basic key value test", "-- key: value", true)]
    #[case("colon and hyphen", "-- created: 2024-06-09T14:28:09-04:00", true)]
    #[case("pipe", "-- key: some | thing", true)]
    #[case("escaped pipe", "-- key: some \\| thing", true)]
    #[case("path with slash", "-- path: /", true)]
    #[case("type with hyphen", "-- type: home-page", true)]
    fn section_attr_basci_fixture(
        #[case] _description: &str,
        #[case] source: &str,
        #[case] should_pass: bool,
    ) {
        if should_pass {
            assert_eq!("", section_attr(source).unwrap().0);
        }
    }

    #[test]
    fn flag_attr_with_colons_in_it() {
        let source = "-- https://www.example.com";
        assert!(section_flag_attr(source).is_ok());
    }

    #[test]
    fn flag_attr_with_hyphen_init() {
        let source = "-- https://www.exa-mple.com";
        assert!(section_flag_attr(source).is_ok());
    }

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

    #[test]
    fn key_value_with_hyphen() {
        let source = "-- key: alfa-bravo";
        let left = SectionAttr {
            kind: SectionAttrKind::KeyValueSpans {
                key: "key".to_string(),
                spans: vec![
                    Span {
                        attrs: vec![],
                        kind: SpanKind::WordPart {},
                        parsed_text: "alfa".to_string(),
                        source_text: "alfa".to_string(),
                    },
                    Span {
                        attrs: vec![],
                        kind: SpanKind::Hyphen {},
                        parsed_text: "-".to_string(),
                        source_text: "-".to_string(),
                    },
                    Span {
                        attrs: vec![],
                        kind: SpanKind::WordPart {},
                        parsed_text: "bravo".to_string(),
                        source_text: "bravo".to_string(),
                    },
                ],
            },
        };
        let right = section_attr(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn key_value_with_single_underscore() {
        let source = "-- key: alfa_bravo";
        let left = SectionAttr {
            kind: SectionAttrKind::KeyValueSpans {
                key: "key".to_string(),
                spans: vec![
                    Span {
                        attrs: vec![],
                        kind: SpanKind::WordPart {},
                        parsed_text: "alfa".to_string(),
                        source_text: "alfa".to_string(),
                    },
                    Span {
                        attrs: vec![],
                        kind: SpanKind::SingleUnderscore {},
                        parsed_text: "_".to_string(),
                        source_text: "_".to_string(),
                    },
                    Span {
                        attrs: vec![],
                        kind: SpanKind::WordPart {},
                        parsed_text: "bravo".to_string(),
                        source_text: "bravo".to_string(),
                    },
                ],
            },
        };
        let right = section_attr(source).unwrap().1;
        assert_eq!(left, right);
    }
}
