use crate::span_v42::plaintext::plaintext;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

use crate::span_v42::SpanV42;

// TODO: Make span_key_value_attr_key

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SpanBaseAttrV42 {
    Flag(SpanFlagAttrV42),
    KeyValue { key: String, spans: Vec<SpanV42> },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpanFlagAttrV42 {
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpanKeyValueAttrV42 {
    pub key: String,
    pub spans: Vec<SpanV42>,
}

pub fn span_key_value_attr_key(source: &str) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, key_spans) = terminated(
        many1(alt((alphanumeric1, is_a("-_")))),
        pair(tag(":"), space1),
    )
    .context("")
    .parse(source)?;
    let (source, spans) = many1(alt((plaintext,))).context("").parse(source)?;
    Ok((
        source,
        SpanBaseAttrV42::KeyValue {
            key: key_spans.join(""),
            spans,
        },
    ))
}
