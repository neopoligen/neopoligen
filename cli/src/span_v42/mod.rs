pub mod any_characters;
pub mod block_line_ending;
pub mod escaped_characters;
pub mod named_span;
pub mod not_pipe_or_greater_than_or_less_than_or_escape;
pub mod plaintext;
pub mod plaintext_without_spaces;
pub mod shorthand_code;
pub mod shorthand_emphasis;
pub mod shorthand_footnote;
pub mod shorthand_html;
pub mod shorthand_image;
pub mod shorthand_mark;
pub mod shorthand_strikethrough;
pub mod shorthand_strong;
pub mod shorthand_underline;
pub mod single_characters;
pub mod space0_line_ending_or_eof;

use self::named_span::NamedSpanV42;
use crate::span_v42::escaped_characters::*;
use crate::span_v42::plaintext::*;
use crate::span_v42::shorthand_code::CodeShorthandV42;
use crate::span_v42::shorthand_emphasis::EmphasisShorthandV42;
use crate::span_v42::shorthand_footnote::FootnoteShorthandV42;
use crate::span_v42::shorthand_html::HtmlShorthandV42;
use crate::span_v42::shorthand_image::ImageShorthandV42;
use crate::span_v42::shorthand_mark::MarkShorthandV42;
use crate::span_v42::shorthand_strikethrough::StrikethroughShorthandV42;
use crate::span_v42::shorthand_strong::StrongShorthandV42;
use crate::span_v42::shorthand_underline::UnderlineShorthandV42;
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
    #[serde(rename = "shorthand-code")]
    CodeShorthand(CodeShorthandV42),
    #[serde(rename = "shorthand-emphasis")]
    EmphasisShorthand(EmphasisShorthandV42),
    #[serde(rename = "escaped-character")]
    EscapedCharacter(EscapedCharacterSpanV42),
    #[serde(rename = "shorthand-footnote")]
    FootnoteShorthand(FootnoteShorthandV42),
    #[serde(rename = "shorthand-html")]
    HtmlShorthand(HtmlShorthandV42),
    #[serde(rename = "shorthand-image")]
    ImageShorthand(ImageShorthandV42),
    #[serde(rename = "shorthand-mark")]
    MarkShorthand(MarkShorthandV42),
    #[serde(rename = "named-span")]
    NamedSpan(NamedSpanV42),
    #[serde(rename = "plain-text")]
    PlainText(PlainTextSpan),
    #[serde(rename = "shorthand-strong")]
    StrongShorthand(StrongShorthandV42),
    #[serde(rename = "shorthand-strikethrough")]
    StrikethroughShorthand(StrikethroughShorthandV42),
    #[serde(rename = "shorthand-underline")]
    UnderlineShorthand(UnderlineShorthandV42),
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
