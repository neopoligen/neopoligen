// DEPRECATED: TODO - Remove this in favor of
// the span tokens
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;
// use crate::span_attr_v39::SpanAttrV39Kind;
// use crate::span_v39::*;
// use nom::branch::alt;
// use nom::bytes::complete::is_not;
// use nom::multi::many0;
// use nom::character::complete::line_ending;
// use nom::character::complete::multispace0;
// use nom::character::complete::space0;
// use nom::character::complete::space1;
// use nom::combinator::eof;
// use nom::combinator::not;
// use nom::sequence::tuple;
// use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpanShorthandTokenV39 {
    pub source_text: String,
    pub parsed_text: String,
    pub kind: SpanShorthandTokenV39Kind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SpanShorthandTokenV39Kind {
    EscapedBackslash,
    EscapedBacktick,
    EscapedColon,
    EscapedPipe,
    SingleBacktick,
    SingleBackslash,
    WordPart,
}

pub fn shorthand_token_escaped_backtick_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("\\").context("").parse(source)?;
    let (source, _) = tag("`").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let token = SpanShorthandTokenV39 {
        source_text,
        parsed_text: "`".to_string(),
        kind: SpanShorthandTokenV39Kind::EscapedBacktick,
    };
    Ok((source, token))
}

pub fn shorthand_token_escaped_colon_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("\\").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let token = SpanShorthandTokenV39 {
        source_text,
        parsed_text: ":".to_string(),
        kind: SpanShorthandTokenV39Kind::EscapedColon,
    };
    Ok((source, token))
}

pub fn shorthand_token_escaped_pipe_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let (source, the_escape) = tag("\\").context("").parse(source)?;
    let (source, text) = tag("|").context("").parse(source)?;
    let token = SpanShorthandTokenV39 {
        source_text: format!("{}{}", the_escape, text),
        parsed_text: format!("{}", text),
        kind: SpanShorthandTokenV39Kind::EscapedPipe,
    };
    Ok((source, token))
}

pub fn shorthand_token_escaped_backslash_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let (source, the_escape) = tag("\\").context("").parse(source)?;
    let (source, text) = tag("\\").context("").parse(source)?;
    let token = SpanShorthandTokenV39 {
        source_text: format!("{}{}", the_escape, text),
        parsed_text: format!("{}", text),
        kind: SpanShorthandTokenV39Kind::EscapedBackslash,
    };
    Ok((source, token))
}

pub fn shorthand_token_single_backtick_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = pair(tag("`"), not(tag("`"))).context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let token = SpanShorthandTokenV39 {
        source_text,
        parsed_text: "`".to_string(),
        kind: SpanShorthandTokenV39Kind::SingleBacktick,
    };
    Ok((source, token))
}

pub fn shorthand_token_single_backslash_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = pair(tag("\\"), not(tag("\\"))).context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let token = SpanShorthandTokenV39 {
        source_text,
        parsed_text: "\\".to_string(),
        kind: SpanShorthandTokenV39Kind::SingleBackslash,
    };
    Ok((source, token))
}
