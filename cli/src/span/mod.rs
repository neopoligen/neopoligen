use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Span {
    Newline { text: String },
    Space { text: String },
    WordPart { text: String },
}

pub fn newline(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, result) = line_ending.context("span").parse(source)?;
    Ok((
        source,
        Span::Newline {
            text: result.to_string(),
        },
    ))
}

pub fn space(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = space1.context("space").parse(source)?;
    Ok((
        source,
        Span::Space {
            text: text.to_string(),
        },
    ))
}

pub fn span(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, span) = alt((newline, space, word_part))
        .context("span")
        .parse(source)?;
    Ok((source, span))
}

pub fn spans(source: &str) -> IResult<&str, Vec<Span>, ErrorTree<&str>> {
    let (source, spans) = many0(span).context("spans").parse(source)?;
    Ok((source, spans))
}

pub fn word_part(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = is_not(" \n").context("word_part").parse(source)?;
    Ok((
        source,
        Span::WordPart {
            text: text.to_string(),
        },
    ))
}
