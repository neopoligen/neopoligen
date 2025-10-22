pub mod escaped_character;
pub mod named_span;
pub mod plaintext;
pub mod shorthand_code;
pub mod shorthand_emphasis;
pub mod shorthand_footnote;
pub mod shorthand_html;
pub mod shorthand_image;
pub mod shorthand_link;
pub mod shorthand_mark;
pub mod shorthand_strikethrough;
pub mod shorthand_strong;
pub mod shorthand_underline;
pub mod space0_line_ending_or_eof;
pub mod string_parsers;

use self::named_span::NamedSpanV42;
use crate::span_v42::escaped_character::EscapedCharacterSpanV42;
use crate::span_v42::plaintext::*;
use crate::span_v42::shorthand_code::CodeShorthandV42;
use crate::span_v42::shorthand_emphasis::EmphasisShorthandV42;
use crate::span_v42::shorthand_footnote::FootnoteShorthandV42;
use crate::span_v42::shorthand_html::HtmlShorthandV42;
use crate::span_v42::shorthand_image::ImageShorthandV42;
use crate::span_v42::shorthand_link::LinkShorthandV42;
use crate::span_v42::shorthand_mark::MarkShorthandV42;
use crate::span_v42::shorthand_strikethrough::StrikethroughShorthandV42;
use crate::span_v42::shorthand_strong::StrongShorthandV42;
use crate::span_v42::shorthand_underline::UnderlineShorthandV42;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SpanV42 {
    #[serde(rename = "code-shorthand")]
    CodeShorthand(CodeShorthandV42),
    #[serde(rename = "emphasis-shorthand")]
    EmphasisShorthand(EmphasisShorthandV42),
    #[serde(rename = "escaped-character")]
    EscapedCharacter(EscapedCharacterSpanV42),
    #[serde(rename = "footnote-shorthand")]
    FootnoteShorthand(FootnoteShorthandV42),
    #[serde(rename = "html-shorthand")]
    HtmlShorthand(HtmlShorthandV42),
    #[serde(rename = "image-shorthand")]
    ImageShorthand(ImageShorthandV42),
    #[serde(rename = "link-shorthand")]
    LinkShorthand(LinkShorthandV42),
    #[serde(rename = "mark-shorthand")]
    MarkShorthand(MarkShorthandV42),
    #[serde(rename = "named-span")]
    NamedSpan(NamedSpanV42),
    #[serde(rename = "plain-text")]
    PlainText(PlainTextSpan),
    #[serde(rename = "strong-shorthand")]
    StrongShorthand(StrongShorthandV42),
    #[serde(rename = "strike-shorthand")]
    StrikethroughShorthand(StrikethroughShorthandV42),
    #[serde(rename = "underline-shorthand")]
    UnderlineShorthand(UnderlineShorthandV42),
}
