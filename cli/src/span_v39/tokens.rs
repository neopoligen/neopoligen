use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpanTokenV39 {
    pub source_text: String,
    pub parsed_text: String,
    pub kind: SpanTokenV39Kind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SpanTokenV39Kind {
    EscapedBackslash,
    EscapedBacktick,
    EscapedCloseBracket,
    EscapedColon,
    EscapedOpenBracket,
    EscapedPipe,
    SingleBacktick,
    SingleBackslash,
    WordPart,
}

pub fn token_escaped_backtick_v39(source: &str) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("\\").context("").parse(source)?;
    let (source, _) = tag("`").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let token = SpanTokenV39 {
        source_text,
        parsed_text: "`".to_string(),
        kind: SpanTokenV39Kind::EscapedBacktick,
    };
    Ok((source, token))
}

pub fn token_escaped_close_bracket_v39(
    source: &str,
) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("\\").context("").parse(source)?;
    let (source, _) = tag("]").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let token = SpanTokenV39 {
        source_text,
        parsed_text: "]".to_string(),
        kind: SpanTokenV39Kind::EscapedCloseBracket,
    };
    Ok((source, token))
}

pub fn token_escaped_colon_v39(source: &str) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("\\").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let token = SpanTokenV39 {
        source_text,
        parsed_text: ":".to_string(),
        kind: SpanTokenV39Kind::EscapedColon,
    };
    Ok((source, token))
}

pub fn token_escaped_open_bracket_v39(
    source: &str,
) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("\\").context("").parse(source)?;
    let (source, _) = tag("[").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let token = SpanTokenV39 {
        source_text,
        parsed_text: "[".to_string(),
        kind: SpanTokenV39Kind::EscapedOpenBracket,
    };
    Ok((source, token))
}

pub fn token_escaped_pipe_v39(source: &str) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let (source, the_escape) = tag("\\").context("").parse(source)?;
    let (source, text) = tag("|").context("").parse(source)?;
    let token = SpanTokenV39 {
        source_text: format!("{}{}", the_escape, text),
        parsed_text: format!("{}", text),
        kind: SpanTokenV39Kind::EscapedPipe,
    };
    Ok((source, token))
}

pub fn token_escaped_backslash_v39(source: &str) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let (source, the_escape) = tag("\\").context("").parse(source)?;
    let (source, text) = tag("\\").context("").parse(source)?;
    let token = SpanTokenV39 {
        source_text: format!("{}{}", the_escape, text),
        parsed_text: format!("{}", text),
        kind: SpanTokenV39Kind::EscapedBackslash,
    };
    Ok((source, token))
}

pub fn token_single_backtick_v39(source: &str) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = pair(tag("`"), not(tag("`"))).context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let token = SpanTokenV39 {
        source_text,
        parsed_text: "`".to_string(),
        kind: SpanTokenV39Kind::SingleBacktick,
    };
    Ok((source, token))
}

pub fn token_single_backslash_v39(source: &str) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = pair(tag("\\"), not(tag("\\"))).context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let token = SpanTokenV39 {
        source_text,
        parsed_text: "\\".to_string(),
        kind: SpanTokenV39Kind::SingleBackslash,
    };
    Ok((source, token))
}
