pub mod escaped_char;
pub mod escaped_pipe;
pub mod footnote;
pub mod greater_than;
pub mod inline_key_value_span_no_attributes;
pub mod inline_key_value_span_with_attributes;
pub mod inline_standard_span_no_attributes;
pub mod inline_standard_span_with_attributes;
pub mod less_than;
pub mod pipe_by_itself_in_code;
pub mod single_newline;
pub mod space;
pub mod tag_span;
pub mod tag_word;
// pub mod word; // deprecated in favor of word_segment
pub mod word_segment;

use crate::config::Config;
use crate::span::escaped_char::escaped_char;
use crate::span::escaped_pipe::escaped_pipe; // TODO: Deprecate this for escaped_char
                                             // use crate::span::footnote::footnote;
use crate::span::greater_than::greater_than;
// use crate::span::inline_key_value_span_no_attributes::inline_key_value_span_no_attributes;
// use crate::span::inline_key_value_span_with_attributes::inline_key_value_span_with_attributes;
use crate::span::inline_standard_span_no_attributes::inline_standard_span_no_attributes;
use crate::span::inline_standard_span_with_attributes::inline_standard_span_with_attributes;
use crate::span::less_than::less_than;
use crate::span::pipe_by_itself_in_code::pipe_by_itself_in_code;
use crate::span::single_newline::single_newline;
use crate::span::space::space;
// use crate::span::word::word; // deprecated by word_segment
use crate::span::word_segment::word_segment;
use nom::branch::alt;
use nom::IResult;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(content = "content", rename_all = "lowercase", tag = "type")]
pub enum Span {
    EscapedChar {
        text: String,
        template: String,
    },
    // Deprecate this and move it to escaped char
    EscapedPipe {
        text: String,
        template: String,
    },
    GreaterThan {
        text: String,
        template: String,
    },
    // Deprecated: Remove these
    KeyValueSpan {
        key_value_attributes: BTreeMap<String, String>,
        flag_attributes: BTreeSet<String>,
        span_type: String,
        spans: Vec<Span>,
        template: String,
        value: String,
    },
    LessThan {
        text: String,
        template: String,
    },
    PipeByItselfInCode {
        text: String,
        template: String,
    },
    Space {
        text: String,
        template: String,
    },
    StandardSpan {
        key_value_attributes: BTreeMap<String, String>,
        flag_attributes: BTreeSet<String>,
        span_type: String,
        spans: Vec<Span>,
        template: String,
    },

    //Word {
    //   text: String,
    //  template: String,
    //},
    WordSegment {
        text: String,
        template: String,
    },
}

pub fn span<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Span> {
    let (source, content) = alt((
        // |src| inline_key_value_span_no_attributes(src, config),
        // |src| inline_key_value_span_with_attributes(src, config),
        |src| inline_standard_span_with_attributes(src, config),
        |src| inline_standard_span_no_attributes(src, config),
        pipe_by_itself_in_code,
        escaped_char,
        space,
        less_than,
        greater_than,
        single_newline,
        word_segment,
    ))(source)?;
    Ok((source, content))
}

#[cfg(test)]
mod test {}
