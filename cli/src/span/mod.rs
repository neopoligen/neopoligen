pub mod code_shorthand;
pub mod code_shorthand_single_pipe;
pub mod colon;
pub mod colon_not_followed_by_space;
pub mod double_underscore;
pub mod em_shorthand;
pub mod escaped_backslash;
pub mod escaped_backtick;
pub mod escaped_colon;
pub mod escaped_greaterthan;
pub mod escaped_pipe;
pub mod escaped_underscore;
pub mod greaterthan;
pub mod hyphen;
pub mod lessthan;
pub mod mocks;
pub mod more_than_two_underscores;
pub mod named_span;
pub mod non_escape_backslash;
pub mod pipe;
pub mod single_backtick;
pub mod single_greaterthan;
pub mod single_lessthan;
pub mod single_underscore;
pub mod wordpart;

use crate::span::code_shorthand::*;
use crate::span::code_shorthand_single_pipe::*;
use crate::span::colon::*;
use crate::span::colon_not_followed_by_space::*;
use crate::span::em_shorthand::*;
use crate::span::escaped_backslash::*;
use crate::span::escaped_backtick::*;
use crate::span::escaped_colon::*;
use crate::span::escaped_greaterthan::*;
use crate::span::escaped_pipe::*;
use crate::span::escaped_underscore::*;
use crate::span::greaterthan::*;
use crate::span::hyphen::*;
use crate::span::lessthan::*;
use crate::span::more_than_two_underscores::*;
use crate::span::named_span::*;
use crate::span::non_escape_backslash::*;
use crate::span::single_backtick::*;
use crate::span::single_greaterthan::*;
use crate::span::single_lessthan::*;
use crate::span::single_underscore::*;
use crate::span::wordpart::*;
use crate::span_attr::*;
use nom::branch::alt;
// use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Span {
    pub attrs: Vec<SpanAttr>,
    pub kind: SpanKind,
    pub parsed_text: String,
    // DEPRECATED: Don't use source_text. Remove it when
    // you have the time to go through all the tests
    pub source_text: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum SpanKind {
    CodeShorthand,
    Colon,
    ColonNotFollowedBySpace,
    DoubleUnderscore,
    EmShorthand,
    EscapedBacktick,
    EscapedBackslash,
    EscapedColon,
    EscapedGreaterThan,
    EscapedPipe,
    EscapedUnderscore,
    GreaterThan,
    Hyphen,
    LessThan,
    LinkShorthand,
    MoreThanTwoUnderscores,
    NamedSpan { r#type: String, children: Vec<Span> },
    Newline,
    NonEscapeBackslash,
    Pipe,
    SingleBacktick,
    SingleUnderscore,
    SingleGreaterThan,
    SingleLessThan,
    Space,
    WordPart,
}

pub fn base_span_for_all_text<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((
        wordpart,
        space,
        newline,
        code_shorthand_single_pipe,
        code_shorthand,
        em_shorthand,
        named_span,
        hyphen,
        colon,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        single_underscore,
        escaped_backtick,
        escaped_colon,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
        non_escape_backslash,
        more_than_two_underscores,
    ))
    .context("")
    .parse(source)?;
    Ok((source, span))
}

// Reminder: This doesn't output a span for content
// it's only for the structure of the file
pub fn structure_empty_until_newline_or_eof<'a>(
    source: &'a str,
) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, _) = alt((
        tuple((space0, line_ending)),
        tuple((multispace0, eof.map(|_| ""))),
    ))
    .context("")
    .parse(source)?;
    Ok((source, ""))
}

// DEPRECATED: replace with base_span_for_all_blocks
pub fn span_for_body_text<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((wordpart, span_base, code_shorthand, named_span))
        .context("")
        .parse(source)?;
    Ok((source, span))
}

// DEPRECATED: use the individual base span types
pub fn span_base<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    // Reminder, don't put spaces in here so these can
    // be used for keys. Also, don't put colon in here
    // since that's also part of the key process
    let (source, span) = alt((
        space,
        newline,
        single_greaterthan,
        single_backtick,
        single_lessthan,
        colon,
        escaped_backslash,
        escaped_backtick,
        escaped_greaterthan,
        escaped_pipe,
    ))(source)?;
    Ok((source, span))
}

// DEPRECATED: make and use base_span_for_shorthand_flag
pub fn span_for_shorthand_flag<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((span_base, space, newline, colon_not_followed_by_space))(source)?;
    Ok((source, span))
}

// DEPRECATED: make and use base_span_for_shorthand_text
pub fn span_for_shorthand_text<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((span_base, space, newline, colon))(source)?;
    Ok((source, span))
}

// DEPRECATED: make and use base_span_for_shorthand_key_value_key
pub fn span_for_shorthand_attr_key<'a>(
    source: &'a str,
) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((span_base,))(source)?;
    Ok((source, span))
}

// DEPRECATED: make and use base_span_for_shorthand_key_value_value
pub fn span_for_shorthand_attr_value<'a>(
    source: &'a str,
) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((span_base, colon))(source)?;
    Ok((source, span))
}

// pub fn span_without_shorthands_or_single_pipe<'a>(
//     source: &'a str,
// ) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
//     let (source, span) = alt((
//         escaped_pipe,
//         escaped_backtick,
//         escaped_backslash,
//         escaped_colon,
//         single_backtick,
//         wordpart,
//         space,
//         newline,
//     ))(source)?;
//     Ok((source, span))
// }

// TODO: Move to own file with tests
pub fn newline(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tuple((space0, line_ending)).context("").parse(source)?;
    let (source, _) = not(tuple((space0, line_ending)))
        .context("")
        .parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        Span {
            attrs: vec![],
            source_text,
            parsed_text: "\n".to_string(),
            kind: SpanKind::Newline,
        },
    ))
}

// TODO: Move to own file with tests
pub fn space(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = space1.context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        Span {
            attrs: vec![],
            source_text,
            parsed_text: " ".to_string(),
            kind: SpanKind::Space,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("a", "")]
    #[case("b", "")]
    #[case(":", "")]
    #[case("<<alfa|bravo>>", "")]
    #[case("<<alfa-bravo|charlie>>", "")]
    #[case("<<alfa_bravo|charlie>>", "")]
    #[case("<<alfa|bravo-charlie>>", "")]
    #[case("<<alfa|bravo_charlie>>", "")]
    #[case("<<alfa|bravo|charlie>>", "")]
    #[case("<<alfa|bravo|charlie-delta>>", "")]
    #[case("<<alfa|bravo|charlie_delta>>", "")]
    #[case("______", "")]
    fn run_test(#[case] input: &str, #[case] left: &str) {
        let right = base_span_for_all_text(input).unwrap().0;
        assert_eq!(left, right);
    }

    #[test]
    fn nested_span() {
        let source = "<<em|delta <<strong|echo>> foxtrot>>";
        let left = Span {
            attrs: vec![],
            source_text: "<<em|delta <<strong|echo>> foxtrot>>".to_string(),
            parsed_text: "".to_string(),
            kind: SpanKind::NamedSpan {
                r#type: "em".to_string(),
                children: vec![
                    Span {
                        attrs: vec![],
                        kind: SpanKind::WordPart,
                        parsed_text: "delta ".to_string(),
                        source_text: "delta ".to_string(),
                    },
                    Span {
                        attrs: vec![],
                        parsed_text: "".to_string(),
                        source_text: "<<strong|echo>>".to_string(),
                        kind: SpanKind::NamedSpan {
                            r#type: "strong".to_string(),
                            children: vec![Span {
                                attrs: vec![],
                                kind: SpanKind::WordPart,
                                parsed_text: "echo".to_string(),
                                source_text: "echo".to_string(),
                            }],
                        },
                    },
                    Span {
                        attrs: vec![],
                        kind: SpanKind::WordPart,
                        parsed_text: " foxtrot".to_string(),
                        source_text: " foxtrot".to_string(),
                    },
                ],
            },
        };
        let right = span_for_body_text(source).unwrap().1;
        assert_eq!(left, right);
    }
}
