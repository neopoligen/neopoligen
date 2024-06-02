use nom::bytes::complete::tag;
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
    pub kind: SpanShorthandTokenV39Kind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SpanShorthandTokenV39Kind {
    EscapedBacktick {
        source_text: String,
        parsed_text: String,
    },
    EscapedPipe {
        source_text: String,
        parsed_text: String,
    },
    EscapedSlash {
        source_text: String,
        parsed_text: String,
    },
    WordPart {
        source_text: String,
        parsed_text: String,
    },
}

pub fn shorthand_token_escaped_backtick_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let (source, the_escape) = tag("\\").context("").parse(source)?;
    let (source, text) = tag("`").context("").parse(source)?;
    let token = SpanShorthandTokenV39 {
        kind: SpanShorthandTokenV39Kind::EscapedBacktick {
            source_text: format!("{}{}", the_escape, text),
            parsed_text: format!("{}", text),
        },
    };
    Ok((source, token))
}

pub fn shorthand_token_escaped_pipe_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let (source, the_escape) = tag("\\").context("").parse(source)?;
    let (source, text) = tag("|").context("").parse(source)?;
    let token = SpanShorthandTokenV39 {
        kind: SpanShorthandTokenV39Kind::EscapedPipe {
            source_text: format!("{}{}", the_escape, text),
            parsed_text: format!("{}", text),
        },
    };
    Ok((source, token))
}

pub fn shorthand_token_escaped_slash_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let (source, the_escape) = tag("\\").context("").parse(source)?;
    let (source, text) = tag("\\").context("").parse(source)?;
    let token = SpanShorthandTokenV39 {
        kind: SpanShorthandTokenV39Kind::EscapedSlash {
            source_text: format!("{}{}", the_escape, text),
            parsed_text: format!("{}", text),
        },
    };
    Ok((source, token))
}
