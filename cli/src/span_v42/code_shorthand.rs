use crate::helpers::flatten_spans_v42;
use crate::span_v42::any_hyphens::any_hyphens;
use crate::span_v42::any_underscores::any_underscores;
use crate::span_v42::plaintext::plaintext;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CodeShorthandKeyValueAttrPayloadV42 {
    pub value: Vec<SpanV42>,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CodeShorthandV42 {
    attrs: BTreeMap<String, CodeShorthandKeyValueAttrPayloadV42>,
    flags: Vec<String>,
    text: String,
    styled: Option<String>,
}

pub fn code_shorthand_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, tokens) = preceded(pair(tag("``"), space0), many1(code_shorthand_content))
        .context("")
        .parse(source)?;
    let (source, (_attrs, _flags)) = get_code_shorthand_attrs_and_flags
        .context("")
        .parse(source)?;
    let (source, _) = tag("``").context("").parse(source)?;
    Ok((
        source,
        SpanV42::CodeShorthand(CodeShorthandV42 {
            attrs: BTreeMap::new(),
            flags: vec![],
            text: flatten_spans_v42(&tokens),
            styled: None,
        }),
    ))
}

pub fn get_code_shorthand_attrs_and_flags(
    source: &str,
) -> IResult<
    &str,
    (
        BTreeMap<String, CodeShorthandKeyValueAttrPayloadV42>,
        Vec<String>,
    ),
    ErrorTree<&str>,
> {
    let (source, _attrs_and_flags) = many0(code_shorthand_attr_or_flag)
        .context("")
        .parse(source)?;
    let attrs = BTreeMap::new();
    let flags = vec![];
    Ok((source, (attrs, flags)))
}

pub fn code_shorthand_attr_or_flag(
    source: &str,
) -> IResult<&str, CodeShorthandBaseAttrV42, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, attr_or_flag) =
        alt((code_shorthand_span_flag_attr, code_shorthand_key_value_attr))
            .context("")
            .parse(source)?;
    Ok((source, attr_or_flag))
}

pub fn code_shorthand_content(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, token) = alt((plaintext, any_hyphens)).context("").parse(source)?;
    Ok((source, token))
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CodeShorthandFlagV42 {
    value: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CodeShorthandKeyValueAttrV42 {
    key: String,
    value: Vec<SpanV42>,
    text: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CodeShorthandBaseAttrV42 {
    Flag(CodeShorthandFlagV42),
    KeyValue(CodeShorthandKeyValueAttrV42),
}

pub fn code_shorthand_span_flag_attr(
    source: &str,
) -> IResult<&str, CodeShorthandBaseAttrV42, ErrorTree<&str>> {
    let (source, spans) = many1(alt((plaintext, any_hyphens, any_underscores)))
        .context("")
        .parse(source)?;
    let (source, _) = not(tag(":")).context("").parse(source)?;
    Ok((
        source,
        CodeShorthandBaseAttrV42::Flag(CodeShorthandFlagV42 {
            value: flatten_spans_v42(&spans),
        }),
    ))
}

pub fn code_shorthand_key_value_attr(
    source: &str,
) -> IResult<&str, CodeShorthandBaseAttrV42, ErrorTree<&str>> {
    let (source, key_spans) = terminated(many1(alt((plaintext,))), pair(tag(":"), space1))
        .context("")
        .parse(source)?;
    let (source, spans) = many1(alt((plaintext,))).context("").parse(source)?;
    Ok((
        source,
        CodeShorthandBaseAttrV42::KeyValue(CodeShorthandKeyValueAttrV42 {
            key: flatten_spans_v42(&key_spans),
            value: spans,
            text: "".to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {

    use super::*;
    //use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("``alfa``etc...", "etc...")]
    #[case("``alfa|bravo``etc...", "etc...")]
    #[case("``alfa|bravo: charlie``etc...", "etc...")]
    #[case("``-- neo sections are allowed``etc...", "etc...")]
    fn code_shorthand_test_run(#[case] source: &str, #[case] remainder: &str) {
        let result = code_shorthand_span(source).unwrap();
        if result.clone().0 == remainder {
            assert!(true);
        } else {
            dbg!(&result);
            assert!(false);
        }
    }
}
