pub mod escaped_pipe;
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
pub mod word;

use crate::config::Config;
use crate::span::escaped_pipe::escaped_pipe;
use crate::span::greater_than::greater_than;
use crate::span::inline_key_value_span_no_attributes::inline_key_value_span_no_attributes;
use crate::span::inline_key_value_span_with_attributes::inline_key_value_span_with_attributes;
use crate::span::inline_standard_span_no_attributes::inline_standard_span_no_attributes;
use crate::span::inline_standard_span_with_attributes::inline_standard_span_with_attributes;
use crate::span::less_than::less_than;
use crate::span::pipe_by_itself_in_code::pipe_by_itself_in_code;
use crate::span::single_newline::single_newline;
use crate::span::space::space;
use crate::span::word::word;
use nom::branch::alt;
use nom::IResult;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(content = "content", rename_all = "lowercase", tag = "type")]
pub enum Span {
    EscapedPipe {
        text: String,
        template: String,
    },
    GreaterThan {
        text: String,
        template: String,
    },
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
    Word {
        text: String,
        template: String,
    },
}

pub fn span<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Span> {
    let (source, content) = alt((
        |src| inline_key_value_span_no_attributes(src, config),
        |src| inline_key_value_span_with_attributes(src, config),
        |src| inline_standard_span_with_attributes(src, config),
        |src| inline_standard_span_no_attributes(src, config),
        pipe_by_itself_in_code,
        escaped_pipe,
        space,
        less_than,
        greater_than,
        single_newline,
        word,
    ))(source)?;
    Ok((source, content))
}

#[cfg(test)]
mod test {
    // testing it done at the lower level
}
