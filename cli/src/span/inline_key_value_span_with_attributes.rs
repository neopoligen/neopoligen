use crate::config::Config;
use crate::span::span;
use crate::span::Span;
use crate::span_attribute::TagAttribute;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::none_of;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::Err;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn inline_key_value_span_with_attributes<'a>(
    source: &'a str,
    config: &'a Config,
) -> IResult<&'a str, Span> {



    let (source, _) = tag_no_case("<<")(source)?;
    let (source, span_type) = match match config.key_value_spans.iter().find_map(|t| {
        match tag_no_case::<&str, &str, Error<&str>>(t.as_str())(source) {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }) {
        Some(x) => Ok(x),
        None => Err(Err::Error(Error {
            input: "\n",
            code: ErrorKind::Tag,
        })),
    } {
        Ok(x) => Ok(x),
        Err(_) => Err(Err::Error(Error::new(source, ErrorKind::Tag))),
    }?;

    let (source, _) = tag(":")(source)?;
    let (source, _) = space1(source)?;
    let (source, value) = take_until("|")(source)?;
    let (source, _) = tag("|")(source)?;
    let (source, content) = many1(|src| span(src, config))(source)?;
    let (source, attributes) =
        many1(alt((tag_boolean_attribute, tag_key_value_attribute)))(source)?;
    let (source, _) = tag(">>")(source)?;
    let mut key_value_attributes = BTreeMap::new();
    attributes.iter().for_each(|attr| {
        if let TagAttribute::KeyValue { key, value } = attr {
            key_value_attributes.insert(key.to_string(), value.to_string());
        }
    });
    let mut flag_attributes = BTreeSet::new();
    attributes.iter().for_each(|attr| {
        if let TagAttribute::Boolean { key } = attr {
            flag_attributes.insert(key.to_string());
        }
    });
    Ok((
        source,
        Span::KeyValueSpan {
            flag_attributes,
            key_value_attributes,
            span_type: span_type.to_string(),
            spans: content,
            template: format!("spans/{}.neojinja", span_type),
            value: value.to_string(),
        },
    ))
}

// TODO: Move this when it's solid
fn tag_boolean_attribute(source: &str) -> IResult<&str, TagAttribute> {
    let (source, _) = tag("|")(source)?;
    let (source, key) = is_not("|>")(source)?;
    let (verify, _) = opt(take_until(":"))(key)?;
    let (_, _) = not(tag(": "))(verify)?;
    Ok((
        source,
        TagAttribute::Boolean {
            key: key.to_string(),
        },
    ))
}

// TODO: Move this when it's solid
fn tag_key_value_attribute(source: &str) -> IResult<&str, TagAttribute> {
    let (source, _) = tag("|")(source)?;
    let (source, key) = is_not(":")(source)?;
    let (source, _) = tag(":")(source)?;
    let (source, _) = space1(source)?;
    let (source, value) = terminated(is_not("|>"), not(none_of("|>")))(source)?;
    Ok((
        source,
        TagAttribute::KeyValue {
            key: key.to_string(),
            value: value.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeSet;

    #[test]
    // #[ignore]
    fn basic_link() {
        let source = "<<ilink: a1b2c3d4|Sierra|id: delta|example_flag>>";
        let config = Config::set1();
        let mut key_value_attributes = BTreeMap::new();
        key_value_attributes.insert("id".to_string(), "delta".to_string());
        let mut flag_attributes = BTreeSet::new();
        flag_attributes.insert("example_flag".to_string());
        let left = Ok((
            "",
            Span::KeyValueSpan {
                span_type: "ilink".to_string(),
                spans: vec![Span::Word {
                    text: "Sierra".to_string(),
                    template: "spans/word.neojinja".to_string(),
                }],
                key_value_attributes,
                flag_attributes,
                template: "spans/ilink.neojinja".to_string(),
                value: "a1b2c3d4".to_string(),
            },
        ));
        let right = inline_key_value_span_with_attributes(source, &config);
        assert_eq!(left, right);
    }

    // #[test]
    // // #[ignore]
    // fn basic_link() {
    //     let source = "<<link|bravo|example.com>>";
    //     let config = Config::set1();
    //     let mut flag_attributes = BTreeSet::new();
    //     flag_attributes.insert("example.com".to_string());
    //     let left = Ok((
    //         "",
    //         Span::StandardSpan {
    //             span_type: "link".to_string(),
    //             spans: vec![Span::Word {
    //                 text: "bravo".to_string(),
    //                 template: "spans/word.neojinja".to_string(),
    //             }],
    //             key_value_attributes: BTreeMap::new(),
    //             flag_attributes,
    //             template: "spans/link.neojinja".to_string(),
    //             classes: None,
    //             id: None,
    //         },
    //     ));
    //     let right = inline_standard_span_with_attributes(source, &config);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn multiple_attributes() {
    //     let source = "<<link|Sierra|id: bravo|hidden>>";
    //     let config = Config::set1();
    //     let mut key_value_attributes = BTreeMap::new();
    //     key_value_attributes.insert("id".to_string(), "bravo".to_string());
    //     let mut flag_attributes = BTreeSet::new();
    //     flag_attributes.insert("hidden".to_string());
    //     let left = Ok((
    //         "",
    //         Span::StandardSpan {
    //             span_type: "link".to_string(),
    //             spans: vec![Span::Word {
    //                 text: "Sierra".to_string(),
    //                 template: "spans/word.neojinja".to_string(),
    //             }],
    //             key_value_attributes,
    //             flag_attributes,
    //             template: "spans/link.neojinja".to_string(),
    //             classes: None,
    //             id: Some("bravo".to_string()),
    //         },
    //     ));
    //     let right = inline_standard_span_with_attributes(source, &config);
    //     assert_eq!(left, right);
    // }

    // // MOVE THESE WITHE THE SOURCE WHEN THAT MOVE HAPPENS
    // #[test]
    // // #[ignore]
    // fn tag_boolean_attribute_basic() {
    //     let source = "|example>>";
    //     let left = Ok((
    //         ">>",
    //         TagAttribute::Boolean {
    //             key: "example".to_string(),
    //         },
    //     ));
    //     let right = tag_boolean_attribute(source);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn tag_key_value_attribute_basic() {
    //     let source = "|class: alfa>>";
    //     let left = Ok((
    //         ">>",
    //         TagAttribute::KeyValue {
    //             key: "class".to_string(),
    //             value: "alfa".to_string(),
    //         },
    //     ));
    //     let right = tag_key_value_attribute(source);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn dont_treat_link_as_key_value() {
    //     let source = "|https://www.example.com/>>";
    //     let left = Ok((
    //         ">>",
    //         TagAttribute::Boolean {
    //             key: "https://www.example.com/".to_string(),
    //         },
    //     ));
    //     let right = tag_boolean_attribute(source);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn more_checking() {
    //     let source = "|echo.example.com>>\n\ncharlie <<em|delta>>";
    //     let left = Ok((
    //         ">>\n\ncharlie <<em|delta>>",
    //         TagAttribute::Boolean {
    //             key: "echo.example.com".to_string(),
    //         },
    //     ));
    //     let right = tag_boolean_attribute(source);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn and_more() {
    //     let source = "|https://www.alanwsmith.com/neopolitan/|class: red>>";
    //     let left = Ok((
    //         "|class: red>>",
    //         TagAttribute::Boolean {
    //             key: "https://www.alanwsmith.com/neopolitan/".to_string(),
    //         },
    //     ));
    //     let right = tag_boolean_attribute(source);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn absolute_paths() {
    //     let source = "|/some-path/asdf>>";
    //     let left = Ok((
    //         ">>",
    //         TagAttribute::Boolean {
    //             key: "/some-path/asdf".to_string(),
    //         },
    //     ));
    //     let right = tag_boolean_attribute(source);
    //     assert_eq!(left, right);
    // }
}
