use crate::config::Config;
use crate::span::span;
use crate::span::Span;
use nom::bytes::complete::tag;
// use nom::bytes::complete::tag_no_case;
// use nom::bytes::complete::take_until;
// use nom::error::Error;
// use nom::error::ErrorKind;
use nom::multi::many1;
// use nom::Err;
use crate::span_attribute::TagAttribute;
use nom::bytes::complete::is_not;
use nom::multi::many0;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn code<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Span> {
    let (source, _) = tag("`")(source)?;
    let (source, content) = many1(|src| span(src, config))(source)?;
    let (source, _) = tag("`")(source)?;
    let (source, attrs) = many0(|src| code_flag_attr(src, config))(source)?;
    let (source, _) = tag("`")(source)?;
    Ok((
        source,
        Span::Code {
            spans: content,
            key_value_attributes: BTreeMap::new(),
            flag_attributes: BTreeSet::new(),
            span_type: "code".to_string(),
            template: format!("spans/code.neojinja"),
        },
    ))
}

pub fn code_flag_attr<'a>(source: &'a str, _config: &'a Config) -> IResult<&'a str, TagAttribute> {
    let (source, _) = tag("|")(source)?;
    let (source, content) = is_not("`:|>")(source)?;
    Ok((
        source,
        TagAttribute::Boolean {
            key: content.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::Error;
    use nom::error::ErrorKind;
    use nom::Err;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_code() {
        let source = "`example``";
        let config = Config::set1();
        let left = Ok((
            "",
            Span::Code {
                span_type: "code".to_string(),
                spans: vec![Span::WordSegment {
                    text: "example".to_string(),
                    template: "spans/word_segment.neojinja".to_string(),
                }],
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                template: "spans/code.neojinja".to_string(),
            },
        ));
        let right = code(source, &config);
        assert_eq!(left, right);
    }

    // #[test]
    // fn full_test_with_flag_attribute() {
    //     let source = "`bravo|rust``";
    //     let config = Config::set1();
    //     let mut flags = BTreeSet::new();
    //     let kvs = BTreeMap::new();
    //     flags.insert("rust".to_string());
    //     let left = Ok((
    //         "",
    //         Span::Code {
    //             span_type: "code".to_string(),
    //             spans: vec![Span::WordSegment {
    //                 text: "bravo".to_string(),
    //                 template: "spans/word_segment.neojinja".to_string(),
    //             }],
    //             key_value_attributes: kvs,
    //             flag_attributes: flags,
    //             template: "spans/code.neojinja".to_string(),
    //         },
    //     ));
    //     let right = code(source, &config);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_flag_attr_direct_test() {
    //     let source = "|example`";
    //     let config = Config::set1();
    //     let left = Ok((
    //         "`",
    //         TagAttribute::Boolean {
    //             key: "example".to_string(),
    //         },
    //     ));
    //     let right = code_flag_attr(source, &config);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_flag_attr_no_value() {
    //     let source = "`";
    //     let config = Config::set1();
    //     let left = Err(Err::Error(Error::new("`", ErrorKind::IsNot)));
    //     let right = code_flag_attr(source, &config);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_flag_attr_followed_by_other_thing() {
    //     let source = "alfa|something: else`";
    //     let config = Config::set1();
    //     let left = Ok((
    //         "|something: else`",
    //         TagAttribute::Boolean {
    //             key: "example".to_string(),
    //         },
    //     ));
    //     let right = code_flag_attr(source, &config);
    //     assert_eq!(left, right);
    // }
}
