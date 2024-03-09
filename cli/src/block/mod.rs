use crate::child::Child;
use crate::config::Config;
use crate::span::span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many0;
use nom::IResult;

pub fn block<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let (source, _) = multispace0(source)?;
    let (source, _) = not(tag("--"))(source)?;
    let (source, response) = many0(|src| span(src, config))(source)?;
    let (source, _) = alt((tag("\n"), eof))(source)?;
    Ok((source, Child::Block(response)))
}
