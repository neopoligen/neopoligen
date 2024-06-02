use crate::span_attr_v39::SpanAttrV39Kind;
use crate::span_token_v39::SpanTokenV39;
use crate::span_token_v39::SpanTokenV39Kind;
use crate::span_v39::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::many0;
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

pub fn code_shorthand_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, _) = tag("``").context("").parse(source)?;
    let (source, text) = is_not("`|").context("").parse(source)?;
    let (source, attrs) = many0(code_shorthand_attr_v39).context("").parse(source)?;
    let (source, _) = tag("``").context("").parse(source)?;
    Ok((
        source,
        SpanV39 {
            kind: SpanV39Kind::CodeShorthand {
                attrs,
                source_text: "``code``".to_string(),
                parsed_text: text.to_string(),
            },
        },
    ))
}

pub fn code_shorthand_attr_v39(source: &str) -> IResult<&str, SpanAttrV39, ErrorTree<&str>> {
    let (source, attr) = alt((code_shorthand_flag_attr_v39,))
        .context("")
        .parse(source)?;
    Ok((source, attr))
}

pub fn code_shorthand_flag_attr_v39(source: &str) -> IResult<&str, SpanAttrV39, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, text) = is_not("`|").context("").parse(source)?;
    let attr = SpanAttrV39 {
        kind: SpanAttrV39Kind::Flag {
            source_text: format!("|{}", text),
            key: text.to_string(),
        },
    };
    Ok((source, attr))
}

pub fn code_shorthand_flag_attr_v39_dev(
    source: &str,
) -> IResult<&str, SpanAttrV39, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (_source, _text) = is_not("`|").context("").parse(source)?;
    let attr = SpanAttrV39 {
        kind: SpanAttrV39Kind::Flag {
            source_text: format!("|rust\\|here"),
            key: format!("rust|here"),
        },
    };
    Ok(("``", attr))
}

pub fn code_shorthand_token_v39(source: &str) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let (source, token) = alt((code_shorthand_token_word_part_v39,))
        .context("")
        .parse(source)?;
    Ok((source, token))
}

pub fn code_shorthand_token_word_part_v39(
    source: &str,
) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let (source, text) = is_not("\\`|").context("").parse(source)?;
    let token = SpanTokenV39 {
        kind: SpanTokenV39Kind::WordPart {
            source_text: text.to_string(),
            parsed_text: text.to_string(),
        },
    };
    Ok(("``", token))
}

pub fn code_shorthand_token_escaped_backtick_v39(
    source: &str,
) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let (source, the_escape) = tag("\\").context("").parse(source)?;
    let (source, text) = tag("`").context("").parse(source)?;
    let token = SpanTokenV39 {
        kind: SpanTokenV39Kind::EscapedBacktick {
            source_text: format!("{}{}", the_escape, text),
            parsed_text: format!("{}", text),
        },
    };
    Ok((source, token))
}

pub fn code_shorthand_token_escaped_pipe_v39(
    source: &str,
) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let (source, the_escape) = tag("\\").context("").parse(source)?;
    let (source, text) = tag("|").context("").parse(source)?;
    let token = SpanTokenV39 {
        kind: SpanTokenV39Kind::EscapedPipe {
            source_text: format!("{}{}", the_escape, text),
            parsed_text: format!("{}", text),
        },
    };
    Ok((source, token))
}

pub fn code_shorthand_token_escaped_slash_v39(
    source: &str,
) -> IResult<&str, SpanTokenV39, ErrorTree<&str>> {
    let (source, the_escape) = tag("\\").context("").parse(source)?;
    let (source, text) = tag("\\").context("").parse(source)?;
    let token = SpanTokenV39 {
        kind: SpanTokenV39Kind::EscapedSlash {
            source_text: format!("{}{}", the_escape, text),
            parsed_text: format!("{}", text),
        },
    };
    Ok((source, token))
}

// pub fn code_shorthand_key_value_attr_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
//     Ok((source, "".to_string()))
// }
