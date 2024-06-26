use crate::span_attr_v39::SpanAttrV39Kind;
use crate::span_shorthand_token_v39::*;
use crate::span_v39::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
// use nom::character::complete::line_ending;
// use nom::character::complete::multispace0;
// use nom::character::complete::space0;
// use nom::character::complete::space1;
// use nom::combinator::eof;
// use nom::combinator::not;
// use nom::sequence::tuple;
// use serde::Serialize;

pub fn link_shorthand_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("[[").context("").parse(source)?;
    let (source, tokens) = many1(link_shorthand_token_v39).context("").parse(source)?;
    let (source, attrs) = many0(link_shorthand_attr_v39).context("").parse(source)?;
    let (source, _) = tag("]]").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let parsed_text = tokens
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    Ok((
        source,
        SpanV39 {
            attrs,
            source_text,
            parsed_text,
            kind: SpanV39Kind::LinkShorthand,
        },
    ))
}

pub fn link_shorthand_attr_v39(source: &str) -> IResult<&str, SpanAttrV39, ErrorTree<&str>> {
    let (source, attr) = alt((
        link_shorthand_key_value_attr_v39,
        link_shorthand_flag_attr_v39,
    ))
    .context("")
    .parse(source)?;
    Ok((source, attr))
}

pub fn link_shorthand_flag_attr_v39(source: &str) -> IResult<&str, SpanAttrV39, ErrorTree<&str>> {
    let (source, the_tag) = tag("|").context("").parse(source)?;
    let (source, words) = many1(link_shorthand_token_v39).context("").parse(source)?;
    let source_text = words
        .iter()
        .map(|word| word.source_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let value = words
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let attr = SpanAttrV39 {
        source_text: format!("{}{}", the_tag, source_text),
        kind: SpanAttrV39Kind::Flag { value },
    };
    Ok((source, attr))
}

pub fn link_shorthand_token_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let (source, token) = alt((
        shorthand_token_escaped_pipe_v39,
        shorthand_token_escaped_backslash_v39,
        shorthand_token_escaped_backtick_v39,
        shorthand_token_escaped_colon_v39,
        link_shorthand_token_word_part_v39,
        shorthand_token_single_backslash_v39,
        shorthand_token_single_backtick_v39,
    ))
    .context("")
    .parse(source)?;
    Ok((source, token))
}

pub fn link_shorthand_token_word_part_v39(
    source: &str,
) -> IResult<&str, SpanShorthandTokenV39, ErrorTree<&str>> {
    let (source, text) = is_not("\\]|").context("").parse(source)?;
    let token = SpanShorthandTokenV39 {
        source_text: text.to_string(),
        parsed_text: text.to_string(),
        kind: SpanShorthandTokenV39Kind::WordPart,
    };
    Ok((source, token))
}

pub fn link_shorthand_key_value_attr_v39(
    source: &str,
) -> IResult<&str, SpanAttrV39, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("|").context("").parse(source)?;
    // TODO: allow for spaces here
    let (source, key) = is_not(" :|]").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, tokens) = many1(link_shorthand_token_v39).context("").parse(source)?;
    let value = tokens
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let source_text = initial_source.replace(source, "").to_string();
    let attr = SpanAttrV39 {
        source_text,
        kind: SpanAttrV39Kind::KeyValue {
            key: key.to_string(),
            value,
        },
    };
    Ok((source, attr))
}
