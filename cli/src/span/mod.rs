use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::IResult;
use nom_supreme::error::ErrorTree;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Span {
    WordPart { text: String },
}

pub fn span(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, span) = alt((word_part, word_part))(source)?;
    Ok((source, span))
}

fn word_part(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = is_not(" ")(source)?;
    Ok((
        source,
        Span::WordPart {
            text: text.to_string(),
        },
    ))
}
