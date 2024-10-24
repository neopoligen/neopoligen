pub mod any_asterisk;
pub mod any_backtick;
pub mod any_caret;
pub mod any_colon;
pub mod any_hyphen;
pub mod any_lessthan;
pub mod any_newline;
pub mod any_pipe;
pub mod any_underscore;
pub mod code_shorthand;
pub mod colon;
pub mod colon_followed_by_multispace0;
pub mod colon_followed_by_multispace1;
pub mod colon_followed_by_space0;
pub mod colon_followed_by_space1;
pub mod colon_not_followed_by_space;
pub mod double_underscore;
pub mod em_shorthand;
pub mod escaped_asterisk;
pub mod escaped_backslash;
pub mod escaped_backtick;
pub mod escaped_caret;
pub mod escaped_colon;
pub mod escaped_greaterthan;
pub mod escaped_hashtag;
pub mod escaped_hyphen;
pub mod escaped_lessthan;
pub mod escaped_pipe;
pub mod escaped_underscore;
pub mod footnote_shorthand;
pub mod greaterthan;
pub mod hyphen_with_trailing_non_hyphen;
pub mod lessthan;
pub mod mocks;
pub mod more_than_two_carets;
pub mod more_than_two_underscores;
pub mod named_span;
pub mod pipe;
pub mod plaintext;
pub mod single_asterisk;
pub mod single_backtick;
pub mod single_caret;
pub mod single_greaterthan;
pub mod single_hashtag;
pub mod single_hyphen;
pub mod single_lessthan;
pub mod single_underscore;

use crate::span::code_shorthand::*;
// use crate::span::code_shorthand_single_pipe::*;
use crate::span::colon::*;
use crate::span::colon_not_followed_by_space::*;
use crate::span::em_shorthand::*;
use crate::span::escaped_backslash::*;
use crate::span::escaped_backtick::*;
use crate::span::escaped_caret::*;
use crate::span::escaped_colon::*;
use crate::span::escaped_greaterthan::*;
use crate::span::escaped_hyphen::*;
// use crate::span::escaped_lessthan::*;
use crate::span::escaped_pipe::*;
use crate::span::escaped_underscore::*;
use crate::span::footnote_shorthand::*;
// use crate::span::hyphen::*;
use crate::span::hyphen_with_trailing_non_hyphen::*;
use crate::span::more_than_two_carets::*;
use crate::span::more_than_two_underscores::*;
use crate::span::named_span::*;
use crate::span::plaintext::*;
use crate::span::single_backtick::*;
use crate::span::single_caret::*;
use crate::span::single_greaterthan::*;
// use crate::span::single_hyphen::*;
use crate::span::single_lessthan::*;
use crate::span::single_underscore::*;
// use crate::span::wordpart::*;
use crate::span_attr::*;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use nom::branch::alt;
//use nom::bytes::complete::is_not;
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
use std::sync::Arc;

impl Object for Span {
    fn call_method(
        self: &Arc<Span>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "kind" => self.kind(),
            "parsed_text" => self.parsed_text(),
            "template_list" => self.template_list(),
            _ => Ok(Value::from("")),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Span {
    pub attrs: Vec<SpanAttr>,
    pub kind: SpanKind,
    pub parsed_text: String,
}

impl Span {
    pub fn kind(&self) -> Result<Value, Error> {
        match &self.kind {
            SpanKind::PlainText => Ok(Value::from("plaintext")),
            _ => Ok(Value::from("unknown")),
        }
    }

    pub fn parsed_text(&self) -> Result<Value, Error> {
        match &self.kind {
            SpanKind::AnyAsterisk => Ok(Value::from(&self.parsed_text)),
            SpanKind::AnyBacktick => Ok(Value::from(&self.parsed_text)),
            SpanKind::AnyCaret => Ok(Value::from(&self.parsed_text)),
            SpanKind::AnyColon => Ok(Value::from(&self.parsed_text)),
            SpanKind::AnyHyphen => Ok(Value::from(&self.parsed_text)),
            SpanKind::AnyLessThan => Ok(Value::from(&self.parsed_text)),
            SpanKind::AnyNewline => Ok(Value::from(&self.parsed_text)),
            SpanKind::AnyPipe => Ok(Value::from(&self.parsed_text)),
            SpanKind::AnyUnderscore => Ok(Value::from(&self.parsed_text)),
            SpanKind::CodeShorthand { .. } => {
                Ok(Value::from("TODO: combine spans for code shorthand"))
            }
            SpanKind::Colon => Ok(Value::from(&self.parsed_text)),
            SpanKind::ColonFollowedByMultispace0 => Ok(Value::from(&self.parsed_text)),
            SpanKind::ColonFollowedByMultispace1 => Ok(Value::from(&self.parsed_text)),
            SpanKind::ColonFollowedBySpace0 => Ok(Value::from(&self.parsed_text)),
            SpanKind::ColonFollowedBySpace1 => Ok(Value::from(&self.parsed_text)),
            SpanKind::ColonNotFollowedBySpace => Ok(Value::from(&self.parsed_text)),
            SpanKind::DashWithTrailingNonDash => Ok(Value::from(&self.parsed_text)),
            SpanKind::DoubleUnderscore => Ok(Value::from(&self.parsed_text)),
            SpanKind::EmShorthand => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedAsterisk => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedBacktick => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedBackslash => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedCaret => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedColon => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedGreaterThan => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedHashtag => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedLessThan => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedHyphen => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedPipe => Ok(Value::from(&self.parsed_text)),
            SpanKind::EscapedUnderscore => Ok(Value::from(&self.parsed_text)),
            SpanKind::FootnoteShorthand => Ok(Value::from(&self.parsed_text)),
            SpanKind::GreaterThan => Ok(Value::from(&self.parsed_text)),
            SpanKind::HyphenWithTrailingNonHyphen => Ok(Value::from(&self.parsed_text)),
            SpanKind::LessThan => Ok(Value::from(&self.parsed_text)),
            SpanKind::LinkShorthand => Ok(Value::from(&self.parsed_text)),
            SpanKind::MoreThanTwoUnderscores => Ok(Value::from(&self.parsed_text)),
            SpanKind::MoreThanTwoCarets => Ok(Value::from(&self.parsed_text)),
            SpanKind::Newline => Ok(Value::from(&self.parsed_text)),
            SpanKind::NamedSpan { .. } => Ok(Value::from(
                "[NEOPOLIGEN_TODO: figure out what to do with named span top level text]",
            )),
            SpanKind::Pipe => Ok(Value::from(&self.parsed_text)),
            SpanKind::SingleAsterisk => Ok(Value::from(&self.parsed_text)),
            SpanKind::SingleBacktick => Ok(Value::from(&self.parsed_text)),
            SpanKind::SingleCaret => Ok(Value::from(&self.parsed_text)),
            SpanKind::SingleHashtag => Ok(Value::from(&self.parsed_text)),
            SpanKind::SingleHyphen => Ok(Value::from(&self.parsed_text)),
            SpanKind::SingleUnderscore => Ok(Value::from(&self.parsed_text)),
            SpanKind::SingleGreaterThan => Ok(Value::from(&self.parsed_text)),
            SpanKind::SingleLessThan => Ok(Value::from(&self.parsed_text)),
            SpanKind::Space => Ok(Value::from(&self.parsed_text)),
            SpanKind::PlainText => Ok(Value::from(&self.parsed_text)),
        }
    }

    pub fn template_list(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize::<Vec<Value>>(vec![
            Value::from(format!("spans/{}.neoj", self.kind().unwrap())),
            Value::from("spans/unknown.neoj"),
        ]))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum SpanKind {
    AnyAsterisk,
    AnyBacktick,
    AnyCaret,
    AnyColon,
    AnyHyphen,
    AnyLessThan,
    AnyNewline,
    AnyPipe,
    AnyUnderscore,
    CodeShorthand { children: Vec<Span> },
    Colon, // TODO: Deprecate this in favor of AnyColon
    ColonFollowedByMultispace0,
    ColonFollowedByMultispace1,
    ColonFollowedBySpace0,
    ColonFollowedBySpace1,
    ColonNotFollowedBySpace,
    DashWithTrailingNonDash,
    DoubleUnderscore,
    EmShorthand,
    EscapedAsterisk,
    EscapedBacktick,
    EscapedBackslash,
    EscapedCaret,
    EscapedColon,
    //EscapedExclamation,
    EscapedGreaterThan,
    EscapedHashtag,
    EscapedLessThan,
    EscapedHyphen,
    EscapedPipe,
    EscapedUnderscore,
    FootnoteShorthand,
    GreaterThan,
    HyphenWithTrailingNonHyphen,
    LessThan, // TODO: Deprecate in favor of AnyLessThan
    LinkShorthand,
    MoreThanTwoUnderscores,
    MoreThanTwoCarets,
    NamedSpan { r#type: String, children: Vec<Span> },
    Newline,
    Pipe,
    PlainText,
    SingleAsterisk,
    SingleBacktick,
    SingleCaret,
    SingleGreaterThan,
    SingleHashtag,
    SingleHyphen,
    SingleLessThan,
    SingleUnderscore,
    Space,
}

pub fn shorthand<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((code_shorthand, em_shorthand, footnote_shorthand))
        .context("")
        .parse(source)?;
    Ok((source, span))
}

// TODO: Make and move to a V42 version if necessary
pub fn base_span_for_all_text<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((
        alt((
            space,
            newline,
            shorthand,
            named_span,
            //code_shorthand_single_pipe,
            // hyphen,
            // hyphen_with_trailing_non_hyphen,
            colon,
            single_caret,
            single_lessthan,
            single_greaterthan,
            single_backtick,
            single_underscore,
        )),
        alt((
            escaped_backslash,
            escaped_backtick,
            escaped_caret,
            escaped_colon,
            escaped_hyphen,
            escaped_greaterthan,
            escaped_pipe,
            escaped_underscore,
            more_than_two_underscores,
            more_than_two_carets,
        )),
    ))
    .context("")
    .parse(source)?;
    Ok((source, span))
}
// TODO: Make and move to a V42 version if necessary
pub fn base_span_for_list_item<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((
        alt((
            space,
            newline,
            shorthand,
            named_span,
            // code_shorthand_single_pipe,
            // hyphen,
            // hyphen_with_trailing_non_hyphen,
            colon,
            single_caret,
            single_lessthan,
            single_greaterthan,
            single_backtick,
            single_underscore,
        )),
        alt((
            escaped_backslash,
            escaped_backtick,
            escaped_caret,
            escaped_colon,
            escaped_hyphen,
            escaped_greaterthan,
            escaped_pipe,
            escaped_underscore,
            more_than_two_underscores,
            more_than_two_carets,
        )),
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

pub fn span_for_not_a_list_item<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((space,)).context("").parse(source)?;
    Ok((source, span))
}

// DEPRECATED: replace with base_span_for_all_blocks
pub fn span_for_body_text<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((span_base, code_shorthand, named_span))
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
    let (source, _) = tuple((space0, line_ending)).context("").parse(source)?;
    let (source, _) = not(tuple((space0, line_ending)))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: "\n".to_string(),
            kind: SpanKind::Newline,
        },
    ))
}

// TODO: Move to own file with tests
pub fn space(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = space1.context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
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
    #[case("bravo", "")]
    #[case(" charlie delta ", "")]

    // #[case("b", "")]
    // #[case(":", "")]
    // #[case("<<alfa|bravo>>", "")]
    // #[case("<<alfa-bravo|charlie>>", "")]
    // #[case("<<alfa_bravo|charlie>>", "")]
    // #[case("<<alfa|bravo-charlie>>", "")]
    // #[case("<<alfa|bravo_charlie>>", "")]
    // #[case("<<alfa|bravo|charlie>>", "")]
    // #[case("<<alfa|bravo|charlie-delta>>", "")]
    // #[case("<<alfa|bravo|charlie_delta>>", "")]
    // #[case("______", "")]
    // #[case("^", "")]
    // #[case("\\^", "")]
    // #[case("^^^", "")]
    fn plaintext_basic(#[case] input: &str, #[case] left: &str) {
        let right = plaintext(input).unwrap().0;
        assert_eq!(left, right);
    }

    // Deprecated
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
    #[case("^", "")]
    #[case("\\^", "")]
    #[case("^^^", "")]
    fn run_test(#[case] input: &str, #[case] left: &str) {
        let right = base_span_for_all_text(input).unwrap().0;
        assert_eq!(left, right);
    }

    #[test]
    fn nested_span() {
        let source = "<<em|delta <<strong|echo>> foxtrot>>";
        let left = Span {
            attrs: vec![],
            parsed_text: "".to_string(),
            kind: SpanKind::NamedSpan {
                r#type: "em".to_string(),
                children: vec![
                    Span {
                        attrs: vec![],
                        kind: SpanKind::PlainText,
                        parsed_text: "delta ".to_string(),
                    },
                    Span {
                        attrs: vec![],
                        parsed_text: "".to_string(),
                        kind: SpanKind::NamedSpan {
                            r#type: "strong".to_string(),
                            children: vec![Span {
                                attrs: vec![],
                                kind: SpanKind::PlainText,
                                parsed_text: "echo".to_string(),
                            }],
                        },
                    },
                    Span {
                        attrs: vec![],
                        kind: SpanKind::PlainText,
                        parsed_text: " foxtrot".to_string(),
                    },
                ],
            },
        };
        let right = span_for_body_text(source).unwrap().1;
        assert_eq!(left, right);
    }
}
