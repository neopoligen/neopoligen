use super::single_underscore::single_underscore;
use super::PlainTextSpan;
use crate::helpers::flatten_spans_v42;
use crate::span_attrs_v42::*;
use crate::span_v42::escaped_characters::*;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EmphasisShorthandV42 {
    attrs: BTreeMap<String, Vec<SpanV42>>,
    flags: Vec<String>,
    text: String,
}

// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// pub struct EmphasisShorthandKeyValueAttrPayloadV42 {
//     pub spans: Vec<SpanV42>,
//     pub text: String,
// }

pub fn emphasis_shorthand_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, tokens) = preceded(pair(tag("__"), space0), many1(emphasis_shorthand_content))
        .context("")
        .parse(source)?;
    let (source, (attrs, flags)) = get_emphasis_shorthand_attrs_and_flags
        .context("")
        .parse(source)?;
    let (source, _) = tag("__").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EmphasisShorthand(EmphasisShorthandV42 {
            attrs,
            flags,
            text: flatten_spans_v42(&tokens),
        }),
    ))
}

pub fn get_emphasis_shorthand_attrs_and_flags(
    source: &str,
) -> IResult<&str, (BTreeMap<String, Vec<SpanV42>>, Vec<String>), ErrorTree<&str>> {
    let (source, attrs_and_flags) = many0(emphasis_shorthand_attr_or_flag)
        .context("")
        .parse(source)?;
    let mut attrs = BTreeMap::new();
    let flags = attrs_and_flags
        .iter()
        .filter_map(|af| match af {
            SpanBaseAttrV42::Flag(f) => Some(f.text.clone()),
            SpanBaseAttrV42::KeyValue { key, spans } => {
                attrs.insert(key.to_string(), spans.clone());
                None
            }
        })
        .collect::<Vec<String>>();

    Ok((source, (attrs, flags)))
}

pub fn emphasis_shorthand_attr_or_flag(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    // Reminder: Make sure to look for key_value first so flags work properly
    let (source, attr_or_flag) = alt((
        emphasis_shorthand_key_value_attr,
        emphasis_shorthand_span_flag_attr,
    ))
    .context("")
    .parse(source)?;
    Ok((source, attr_or_flag))
}

pub fn emphasis_shorthand_content(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, token) = alt((
        not_underscore_or_pipe_or_escape,
        single_underscore,
        escaped_underscore,
        escaped_pipe,
        escaped_backslash,
    ))
    .context("")
    .parse(source)?;
    Ok((source, token))
}

pub fn not_underscore_or_pipe_or_escape(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_not("_\\|").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn emphasis_shorthand_span_flag_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, spans) = many1(alt((
        not_underscore_or_pipe_or_escape,
        escaped_pipe,
        escaped_underscore,
        single_underscore,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = not(tag(":")).context("").parse(source)?;
    Ok((
        source,
        SpanBaseAttrV42::Flag(SpanFlagAttrV42 {
            text: flatten_spans_v42(&spans),
        }),
    ))
}

pub fn emphasis_shorthand_key_value_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, key_spans) = terminated(
        many1(alt((alphanumeric1, is_a("-_")))),
        pair(tag(":"), space1),
    )
    .context("")
    .parse(source)?;
    let (source, spans) = many1(alt((
        not_underscore_or_pipe_or_escape,
        single_underscore,
        escaped_underscore,
        escaped_pipe,
        escaped_backslash,
    )))
    .context("")
    .parse(source)?;
    Ok((
        source,
        SpanBaseAttrV42::KeyValue {
            key: key_spans.join(""),
            spans,
        },
    ))
}

// pub fn emphasis_shorthand_key_value_attr(
//     source: &str,
// ) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
//     let (source, key_spans) = terminated(
//         many1(alt((alphanumeric1, is_a("-_")))),
//         pair(tag(":"), space1),
//     )
//     .context("")
//     .parse(source)?;
//     let (source, spans) = many1(alt((plaintext,))).context("").parse(source)?;
//     Ok((
//         source,
//         SpanBaseAttrV42::KeyValue {
//             key: key_spans.join(""),
//             spans,
//         },
//     ))
// }

#[cfg(test)]
mod test {
    // tests are in the top level spec tests
}
