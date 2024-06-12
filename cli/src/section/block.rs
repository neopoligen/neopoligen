use crate::section::Section;
use crate::section::SectionBounds;
use crate::section::SectionKind;
use crate::span::code_shorthand::code_shorthand;
use crate::span::code_shorthand_single_pipe::*;
use crate::span::colon::*;
use crate::span::escaped_backslash::*;
use crate::span::escaped_backtick::*;
use crate::span::escaped_greaterthan::*;
use crate::span::escaped_pipe::*;
use crate::span::hyphen::*;
use crate::span::named_span::*;
use crate::span::non_escape_backslash::*;
use crate::span::pipe::*;
use crate::span::single_backtick::*;
use crate::span::single_greaterthan::*;
use crate::span::single_lessthan::*;
use crate::span::wordpart::*;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

#[derive(Clone, Debug, PartialEq)]
pub struct Block {}

pub fn span_for_block_of_anything<'a>(
    source: &'a str,
) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((
        wordpart,
        space,
        newline,
        code_shorthand_single_pipe,
        code_shorthand,
        named_span,
        hyphen,
        pipe,
        colon,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        escaped_backtick,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
        non_escape_backslash,
    ))
    .context("")
    .parse(source)?;
    Ok((source, span))
}

pub fn block_of_anything<'a>(source: &'a str) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, spans) = many1(span_for_block_of_anything)
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

pub fn block_of_end_content<'a>(source: &'a str) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, _) = not(tag("[")).context("").parse(source)?;
    let (source, spans) = many1(alt((
        wordpart,
        space,
        newline,
        code_shorthand,
        code_shorthand_single_pipe,
        named_span,
        hyphen,
        pipe,
        colon,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        escaped_backtick,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
        non_escape_backslash,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

pub fn block_of_list_content<'a>(source: &'a str) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, spans) = many1(alt((
        wordpart,
        space,
        newline,
        code_shorthand,
        code_shorthand_single_pipe,
        named_span,
        hyphen,
        pipe,
        colon,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        escaped_backtick,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
        non_escape_backslash,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    #[rstest]
    #[case("alfa | bravo", "")]
    #[case("alfa - bravo", "")]
    #[case(
        "<<link|Perl|https://en.wikipedia.org/wiki/Perl>> on my own, though, is well",
        ""
    )]
    fn run_test(#[case] input: &str, #[case] left: &str) {
        let right = block_of_anything(input).unwrap().0;
        assert_eq!(left, right);
    }
}
