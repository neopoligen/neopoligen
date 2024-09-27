use crate::section::Section;
use crate::section::SectionBounds;
use crate::section::SectionKind;
use crate::span::pipe::*;
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

pub fn block_of_anything<'a>(source: &'a str) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    // NOTE: Ideally this would look for "--" when it's not inside a
    // list and "-" when it's in a list. In order to do that
    // a new "list_depth" argument needs to be passed around.
    // For now, everything is just using "-". That means you can't
    // start a block of text with just a dash.
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, spans) = many1(alt((base_span_for_all_text, pipe)))
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
    let (source, spans) = many1(alt((base_span_for_all_text,)))
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
    let (source, spans) = many1(alt((base_span_for_all_text,)))
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

pub fn block_of_checklist_content<'a>(
    source: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, _) = not(tag("[")).context("").parse(source)?;
    let (source, spans) = many1(alt((base_span_for_all_text,)))
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
