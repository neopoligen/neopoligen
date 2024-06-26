pub mod code_shorthand;
pub mod link_shorthand;
pub mod mocks;
pub mod object;
pub mod tokens;

use crate::span_attr_v39::SpanAttrV39;
use crate::span_attr_v39::SpanAttrV39Kind;
use crate::span_v39::code_shorthand::code_shorthand_v39;
use crate::span_v39::link_shorthand::link_shorthand_v39;
use minijinja::Value;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpanV39 {
    pub attrs: Vec<SpanAttrV39>,
    pub kind: SpanV39Kind,
    pub parsed_text: String,
    pub source_text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SpanV39Kind {
    Backtick,
    CodeShorthand,
    EscapedBacktick,
    LinkShorthand,
    Newline,
    Space,
    WordPart,
}

impl SpanV39 {
    pub fn attrs(&self) -> Result<Value, minijinja::Error> {
        Ok(Value::make_object_iterable(
            self.attrs.clone(),
            |attr_set| {
                Box::new(attr_set.iter().cloned().filter_map(|attr| match attr.kind {
                    SpanAttrV39Kind::KeyValue { .. } => Some(Value::from_object(attr)),
                    _ => None,
                }))
            },
        ))
    }

    pub fn classes(&self, args: &[Value]) -> Vec<String> {
        let mut class_list: Vec<String> = args
            .iter()
            .filter_map(|arg| {
                if arg.as_str() != Some("") {
                    Some(arg.to_string())
                } else {
                    None
                }
            })
            .collect();
        self.attrs.iter().for_each(|attr| match &attr.kind {
            SpanAttrV39Kind::KeyValue { key, value } => {
                if key == "class" {
                    class_list.push(value.to_string())
                }
            }
            _ => {}
        });
        class_list
    }

    pub fn flags(&self) -> Result<Value, minijinja::Error> {
        Ok(Value::make_object_iterable(
            self.attrs.clone(),
            |attr_set| {
                Box::new(attr_set.iter().cloned().filter_map(|attr| match attr.kind {
                    SpanAttrV39Kind::Flag { .. } => Some(Value::from_object(attr)),
                    _ => None,
                }))
            },
        ))
    }

    pub fn parsed_text(&self) -> Option<String> {
        Some(self.parsed_text.clone())
    }

    pub fn r#type(&self) -> Option<String> {
        match self.kind {
            SpanV39Kind::Backtick => Some("backtick".to_string()),
            SpanV39Kind::CodeShorthand { .. } => Some("codeshorthand".to_string()),
            SpanV39Kind::EscapedBacktick => Some("escapedbacktick".to_string()),
            SpanV39Kind::LinkShorthand { .. } => Some("linkshorthand".to_string()),
            SpanV39Kind::Newline => Some("newline".to_string()),
            SpanV39Kind::Space => Some("space".to_string()),
            SpanV39Kind::WordPart => Some("wordpart".to_string()),
        }
    }

    pub fn template_list(&self) -> Vec<String> {
        vec![format!("spans/{}.neoj", self.r#type().unwrap())]
    }
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

pub fn span_v39<'a>(
    source: &'a str,
    _spans: &'a Vec<String>,
) -> IResult<&'a str, SpanV39, ErrorTree<&'a str>> {
    let (source, span) = alt((
        code_shorthand_v39,
        link_shorthand_v39,
        escaped_backtick_v39,
        backtick_v39,
        word_part_v39,
        space_v39,
        newline_v39,
    ))(source)?;
    Ok((source, span))
}

pub fn backtick_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = pair(tag("`"), not(tag("`"))).context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        SpanV39 {
            attrs: vec![],
            source_text,
            parsed_text: "`".to_string(),
            kind: SpanV39Kind::Backtick,
        },
    ))
}

pub fn escaped_backtick_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    // Reminder: not doing the source/replace here because the
    // escape is captured by the slash. There's certainly
    // a way around that, but this is fine
    let (source, _) = pair(tag("\\"), tag("`")).context("").parse(source)?;
    Ok((
        source,
        SpanV39 {
            attrs: vec![],
            source_text: "\\`".to_string(),
            parsed_text: "`".to_string(),
            kind: SpanV39Kind::EscapedBacktick,
        },
    ))
}
pub fn newline_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tuple((space0, line_ending)).context("").parse(source)?;
    let (source, _) = not(tuple((space0, line_ending)))
        .context("")
        .parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        SpanV39 {
            attrs: vec![],
            source_text,
            parsed_text: "\n".to_string(),
            kind: SpanV39Kind::Newline,
        },
    ))
}

pub fn space_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = space1.context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        SpanV39 {
            attrs: vec![],
            source_text,
            parsed_text: " ".to_string(),
            kind: SpanV39Kind::Space,
        },
    ))
}

pub fn word_part_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, text) = is_not(" \n\t[]`").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        SpanV39 {
            attrs: vec![],
            source_text,
            parsed_text: text.to_string(),
            kind: SpanV39Kind::WordPart,
        },
    ))
}
