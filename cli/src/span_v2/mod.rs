use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::multi::many1;
use nom::IResult;
use nom_supreme::error::ErrorTree;
//use nom_supreme::error::GenericErrorTree;
use nom::combinator::not;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(content = "content", rename_all = "lowercase", tag = "type")]
pub enum SpanV2 {
    Space {
        text: String,
    },
    Tag {
        spans: Vec<SpanV2>,
        r#type: String,
        attrs: Vec<SpanAttrV2>,
    },
    WordSegment {
        text: String,
    },
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(content = "content", rename_all = "lowercase", tag = "type")]
pub enum SpanAttrV2 {
    Flag { key: String },
    KeyValue { key: String, value: String },
}

pub fn spans_v2(source: &str) -> IResult<&str, Vec<SpanV2>, ErrorTree<&str>> {
    let (source, content) = many1(span_v2)(source)?;
    Ok((source, content))
}

pub fn span_v2(source: &str) -> IResult<&str, SpanV2, ErrorTree<&str>> {
    let (source, content) = alt((space, span_tag, word_segment))(source)?;
    Ok((source, content))
}

fn space(source: &str) -> IResult<&str, SpanV2, ErrorTree<&str>> {
    let (source, content) = space1(source)?;
    Ok((
        source,
        SpanV2::Space {
            text: content.to_string(),
        },
    ))
}

fn span_attr_v2(source: &str) -> IResult<&str, SpanAttrV2, ErrorTree<&str>> {
    let (source, _) = tag("|")(source)?;
    let (source, attr) = alt((span_attr_flag_v2, span_attr_kv_v2))(source)?;
    Ok((source, attr))
}

fn span_attr_flag_v2(source: &str) -> IResult<&str, SpanAttrV2, ErrorTree<&str>> {
    let (source, flag) = is_not("|:>")(source)?;
    let (source, _) = not(tag(":"))(source)?;

    Ok((
        source,
        SpanAttrV2::Flag {
            key: flag.to_string(),
        },
    ))
}

fn span_attr_kv_v2(source: &str) -> IResult<&str, SpanAttrV2, ErrorTree<&str>> {
    let (source, key) = is_not(":")(source)?;
    let (source, _) = tag(":")(source)?;
    let (source, _) = space0(source)?;
    let (source, value) = is_not("|>")(source)?;
    Ok((
        source,
        SpanAttrV2::KeyValue {
            key: key.to_string(),
            value: value.to_string(),
        },
    ))
}

fn span_tag(source: &str) -> IResult<&str, SpanV2, ErrorTree<&str>> {
    let (source, _) = tag("<<")(source)?;
    let (source, r#type) = is_not("|")(source)?;
    let (source, _) = tag("|")(source)?;
    let (source, spans) = spans_v2(source)?;
    let (source, attrs) = many0(span_attr_v2)(source)?;
    let (source, _) = tag(">>")(source)?;
    Ok((
        source,
        SpanV2::Tag {
            r#type: r#type.to_string(),
            spans,
            attrs,
        },
    ))
}

fn word_segment(source: &str) -> IResult<&str, SpanV2, ErrorTree<&str>> {
    let (source, content) = is_not("<> :|")(source)?;
    Ok((
        source,
        SpanV2::WordSegment {
            text: content.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn word_segment() {
        let source = "alfa bravo";
        let left = vec![
            SpanV2::WordSegment {
                text: "alfa".to_string(),
            },
            SpanV2::Space {
                text: " ".to_string(),
            },
            SpanV2::WordSegment {
                text: "bravo".to_string(),
            },
        ];
        let right = spans_v2(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn inline_tag_no_attr() {
        let source = "<<em|charlie>>";
        let attrs = vec![];
        let left = vec![SpanV2::Tag {
            r#type: "em".to_string(),
            spans: vec![SpanV2::WordSegment {
                text: "charlie".to_string(),
            }],
            attrs,
        }];
        let right = spans_v2(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn inline_tag_with_flag_attr() {
        let source = "<<strong|delta|echo>>";
        let attrs = vec![SpanAttrV2::Flag {
            key: "echo".to_string(),
        }];
        let left = SpanV2::Tag {
            r#type: "strong".to_string(),
            spans: vec![SpanV2::WordSegment {
                text: "delta".to_string(),
            }],
            attrs,
        };
        let right = span_v2(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn inline_tag_with_kv_attr() {
        let source = "<<bold|echo|foxtrot: golf>>";
        let attrs = vec![SpanAttrV2::KeyValue {
            key: "foxtrot".to_string(),
            value: "golf".to_string(),
        }];
        let left = SpanV2::Tag {
            r#type: "bold".to_string(),
            spans: vec![SpanV2::WordSegment {
                text: "echo".to_string(),
            }],
            attrs,
        };
        let right = span_v2(source).unwrap().1;
        assert_eq!(left, right);
    }

    // #[test]
    // fn basic_space() {
    //     let source = " x";
    //     let left = Ok((
    //         "x",
    //         SpanV2::Space {
    //             text: " ".to_string(),
    //         },
    //     ));
    //     let right = span_v2(source);
    //     assert_eq!(left, right);
    // }

    //
}
