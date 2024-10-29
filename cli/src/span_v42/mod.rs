pub mod any_colons;
pub mod any_hyphens;
pub mod any_paren_closes;
pub mod any_paren_opens;
pub mod any_underscores;
pub mod block_line_ending;
pub mod code_shorthand;
pub mod escaped_character;
pub mod named_span;
pub mod not_pipe_or_greater_than_or_escape;
pub mod plaintext;
pub mod plaintext_without_spaces;
pub mod single_exclamation;
pub mod single_greater_than;
pub mod single_hyphen;
pub mod single_line_ending;
pub mod single_underscore;
pub mod space0_line_ending_or_eof;

use self::named_span::NamedSpanV42;
use crate::span_v42::code_shorthand::CodeShorthandV42;
use crate::span_v42::escaped_character::*;
use crate::span_v42::plaintext::*;
use nom::branch::alt;
use nom::combinator::not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SpanV42 {
    EscapedCharacter(EscapedCharacterSpanV42),
    PlainText(PlainTextSpan),
    NamedSpan(NamedSpanV42),
    CodeShorthand(CodeShorthandV42),
}

pub fn span_v42<'a>(source: &'a str) -> IResult<&'a str, SpanV42, ErrorTree<&'a str>> {
    let (source, span) = alt((plaintext,)).context("").parse(source)?;
    Ok((source, span))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa-bravo", "-bravo")]
    fn valid_cases_with_remainders(#[case] source: &str, #[case] remainder: &str) {
        assert_eq!(remainder, plaintext(source).unwrap().0);
    }
}
