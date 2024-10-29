use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::space0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CodeShorthandKeyValueAttrPayloadV42 {
    value: Vec<SpanV42>,
    text: String,
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
    let (source, _) = tag("``").context("").parse(source)?;
    Ok((source, SpanV42::CodeShorthand(CodeShorthandV42{
        attrs: BTreeMap::new(),
        flags: vec![],
        text: tokens.join(""),
        styled: None
    })))
}

pub fn code_shorthand_content(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, token) = is_not("`").context("").parse(source)?;
    Ok((source, token))
}

#[cfg(test)]
mod test {

    use super::*;
    //use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("``alfa``", "")]
    fn solo_code_shorthand_test_run(#[case] source: &str, #[case] remainder: &str) {
        let result = code_shorthand_span(source).unwrap();
        if result.clone().0 == remainder{
            assert!(true);
        } else {
            dbg!(&result);
            assert!(false);
        }

    }

}


