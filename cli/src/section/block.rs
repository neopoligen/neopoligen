use crate::section::Section;
use crate::section::SectionBounds;
use crate::section::SectionKind;
// use crate::span::hyphen_with_trailing_non_hyphen::hyphen_with_trailing_non_hyphen;
use crate::span::pipe::*;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
// use nom::character::complete::space1;
use crate::span::escaped_exclamation::escaped_exclamation;
use crate::span::escaped_lessthan::*;
use crate::span::single_exclamation::single_exclamation;
use crate::span::single_lessthan::*;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

use self::escaped_hyphen::escaped_hyphen;
use self::single_hyphen::single_hyphen;

#[derive(Clone, Debug, PartialEq)]
pub struct Block {}

pub fn block_of_main_content_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, spans) = many1(alt((
        take_until_special_characters,
        newline,
        escaped_hyphen,
        single_hyphen,
        single_exclamation,
        escaped_lessthan,
        single_lessthan,
        escaped_exclamation,
        single_exclamation,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attr_list: vec![],
            flag_list: vec![],
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

// TODO: Depreacte this in favor of block_of_main_body_content_V42
pub fn block_of_anything<'a>(source: &'a str) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    // NOTE: Ideally this would look for "--" when it's not inside a
    // list and "-" when it's in a list. In order to do that
    // a new "list_depth" argument needs to be passed around.
    // For now, everything is just using "-". That means you can't
    // start a block of text with just a dash.
    let (source, spans) = many1(alt((base_span_for_all_text, pipe)))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attr_list: vec![],
            flag_list: vec![],
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

pub fn block_that_is_not_a_list_item<'a>(
    source: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("-")).context("").parse(source)?;
    // NOTE: Ideally this would look for "--" when it's not inside a
    // list and "-" when it's in a list. In order to do that
    // a new "list_depth" argument needs to be passed around.
    // For now, everything is just using "-". That means you can't
    // start a block of text with just a dash.
    let (source, spans) = many1(span_for_not_a_list_item).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attr_list: vec![],
            flag_list: vec![],
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
            attr_list: vec![],
            flag_list: vec![],
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

pub fn block_of_end_content_that_is_not_a_list_item<'a>(
    source: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    // dbg!(&source);
    // let (source, _) = multispace0.context("").parse(source)?;
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, spans) = many1(alt((base_span_for_all_text,)))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attr_list: vec![],
            flag_list: vec![],
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

pub fn block_of_list_content_start_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, spans) = many1(alt((base_span_for_all_text,)))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attr_list: vec![],
            flag_list: vec![],
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

pub fn block_of_list_content_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, spans) = many1(alt((base_span_for_list_item,)))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attr_list: vec![],
            flag_list: vec![],
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

pub fn block_of_list_content<'a>(source: &'a str) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, spans) = many1(alt((base_span_for_all_text,)))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attr_list: vec![],
            flag_list: vec![],
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
            attr_list: vec![],
            flag_list: vec![],
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
    #[case("alfa")]
    #[case(" bravo charle")]
    #[case("x\n")]
    #[case("<")]
    #[case("< ")]
    #[case("<\n")]
    #[case(r#"\<<"#)]
    #[case("! ")]
    #[case(r#"\!!"#)]
    #[case("! ")]
    #[case("- ")]
    #[case(r#"\--"#)]

    fn solo_check_basic_parsing_passes(#[case] input: &str) {
        let response = block_of_main_content_v42(input);
        if response.is_ok() {
            let (remainder, _) = response.as_ref().unwrap();
            if *remainder == "" {
                assert!(true);
            } else {
                dbg!(&response);
                assert!(false);
            }
        } else {
            dbg!(&response);
            assert!(false);
        }
    }

    // #[rstest]
    // #[case("alfa | bravo", "")]
    // #[case("alfa - bravo", "")]
    // #[case(
    //     "<<link|Perl|https://en.wikipedia.org/wiki/Perl>> on my own, though, is well",
    //     ""
    // )]
    // fn run_test(#[case] input: &str, #[case] left: &str) {
    //     let right = block_of_anything(input).unwrap().0;
    //     assert_eq!(left, right);
    // }

    #[test]
    fn block_of_anything_basic() {
        let source = "alfa";
        let left = (
            "",
            Section {
                attr_list: vec![],
                flag_list: vec![],
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::Block {
                    spans: vec![Span {
                        attrs: vec![],
                        kind: SpanKind::WordPart,
                        parsed_text: "alfa".to_string(),
                    }],
                },
                r#type: "block-of-text".to_string(),
            },
        );
        let right = block_of_anything(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn block_that_is_not_a_list_item_with_content() {
        let source = "alfa";
        let left = (
            "",
            Section {
                attr_list: vec![],
                flag_list: vec![],
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::Block {
                    spans: vec![Span {
                        attrs: vec![],
                        kind: SpanKind::WordPart,
                        parsed_text: "alfa".to_string(),
                    }],
                },
                r#type: "block-of-text".to_string(),
            },
        );
        let right = block_that_is_not_a_list_item(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn block_that_is_not_a_list_item_should_fail_on_list_item() {
        let source = "- foxtrot";
        assert!(block_that_is_not_a_list_item(source).is_err());
    }

    //
}
