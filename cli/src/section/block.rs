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
//use crate::span::escaped_exclamation::escaped_exclamation;
use crate::span::escaped_lessthan::*;
// use crate::span::single_exclamation::single_exclamation;
use crate::span::plaintext::plaintext;
use crate::span::single_lessthan::*;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

use self::any_colon::any_colon;
use self::any_hyphen::any_hyphen;
use self::any_pipe::any_pipe;
use self::code_shorthand::code_shorthand;
use self::escaped_asterisk::escaped_asterisk;
use self::escaped_backslash::escaped_backslash;
use self::escaped_backtick::escaped_backtick;
use self::escaped_caret::escaped_caret;
use self::escaped_greaterthan::escaped_greaterthan;
use self::escaped_underscore::escaped_underscore;
// use self::footnote_shorthand::footnote_shorthand;
// use self::escaped_hashtag::escaped_hashtag;
// use self::escaped_hyphen::escaped_hyphen;
use self::named_span::named_span_v42;
use self::single_asterisk::single_asterisk;
use self::single_backtick::single_backtick;
use self::single_caret::single_caret;
use self::single_greaterthan::single_greaterthan;
use self::single_underscore::single_underscore;
// use self::single_hashtag::single_hashtag;
// use self::single_hyphen::single_hyphen;

#[derive(Clone, Debug, PartialEq)]
pub struct Block {}

// TODO: Make this the same thing as main, but without
// newline. (i.e. extract main to a base set of things
// then only add newline to it)
pub fn block_of_section_attr_value_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, Vec<Span>, ErrorTree<&'a str>> {
    let (source, spans) = many1(alt((
        plaintext,
        newline,
        single_asterisk,
        escaped_asterisk,
        single_lessthan,
        escaped_lessthan,
        single_caret,
        escaped_caret,
        single_greaterthan,
        escaped_greaterthan,
        single_backtick,
        escaped_backtick,
        single_underscore,
        escaped_underscore,
        any_hyphen,
        any_pipe,
        any_colon,
        escaped_backslash,
        code_shorthand,
        named_span_v42,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((source, spans))
}

pub fn block_of_main_content_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, spans) = many1(alt((
        plaintext,
        newline,
        single_asterisk,
        escaped_asterisk,
        single_lessthan,
        escaped_lessthan,
        single_caret,
        escaped_caret,
        single_greaterthan,
        escaped_greaterthan,
        single_backtick,
        escaped_backtick,
        single_underscore,
        escaped_underscore,
        any_hyphen,
        any_pipe,
        any_colon,
        escaped_backslash,
        code_shorthand,
        named_span_v42,
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

pub fn block_of_list_item_content_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    // NOTE, this just to make sure it's not a new list item the
    // forwards to the other thing
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, section) = block_of_main_content_v42.context("").parse(source)?;
    Ok((source, section))
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

// pub fn block_of_list_content_start_v42_deprecated<'a>(
//     source: &'a str,
// ) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
//     let (source, spans) = many1(alt((base_span_for_all_text,)))
//         .context("")
//         .parse(source)?;
//     let (source, _) = multispace0.context("").parse(source)?;
//     Ok((
//         source,
//         Section {
//             attr_list: vec![],
//             flag_list: vec![],
//             attrs: vec![],
//             bounds: SectionBounds::Full,
//             kind: SectionKind::Block { spans },
//             r#type: "block-of-text".to_string(),
//         },
//     ))
// }

// pub fn block_of_list_content_v42_deprecated<'a>(
//     source: &'a str,
// ) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
//     let (source, _) = multispace0.context("").parse(source)?;
//     let (source, _) = not(eof).context("").parse(source)?;
//     let (source, _) = not(tag("-")).context("").parse(source)?;
//     let (source, spans) = many1(alt((base_span_for_list_item,)))
//         .context("")
//         .parse(source)?;
//     let (source, _) = multispace0.context("").parse(source)?;
//     Ok((
//         source,
//         Section {
//             attr_list: vec![],
//             flag_list: vec![],
//             attrs: vec![],
//             bounds: SectionBounds::Full,
//             kind: SectionKind::Block { spans },
//             r#type: "block-of-text".to_string(),
//         },
//     ))
// }

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
    #[case("hotel-union")]
    #[case("x\n")]
    #[case("<")]
    #[case("< ")]
    #[case("<\n")]
    #[case(r#"\<<"#)]
    #[case("<<span|ping>>")]
    #[case("` ")]
    #[case(r#"\``"#)]
    #[case("``alfa``")]
    #[case("*")]
    #[case(r#"\**"#)]
    #[case("^")]
    #[case(r#"\^^"#)]
    fn check_basic_parsing_passes(#[case] input: &str) {
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
    #[ignore]
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
                        kind: SpanKind::PlainText,
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
    #[ignore]
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
                        kind: SpanKind::PlainText,
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
    #[ignore]
    fn block_that_is_not_a_list_item_should_fail_on_list_item() {
        let source = "- foxtrot";
        assert!(block_that_is_not_a_list_item(source).is_err());
    }

    //
}
