use crate::span_attr_v39::SpanAttrV39Kind;
use crate::span_v39::*;
// use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
// use nom::character::complete::line_ending;
// use nom::character::complete::multispace0;
// use nom::character::complete::space0;
// use nom::character::complete::space1;
// use nom::combinator::eof;
// use nom::combinator::not;
// use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
// use serde::Serialize;

pub fn code_shorthand_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, _) = tag("``").context("").parse(source)?;
    let (source, text) = is_not("`|").context("").parse(source)?;
    let (source, _) = tag("``").context("").parse(source)?;
    let attrs = vec![];
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

// pub fn code_shorthand_attr_v39(source: &str) -> IResult<&str, , ErrorTree<&str>> {
//     Ok((source, "".to_string()))
// }

pub fn code_shorthand_flag_attr_v39(source: &str) -> IResult<&str, SpanAttrV39, ErrorTree<&str>> {
    let (source, _) = tag("|rust").context("").parse(source)?;
    let attr = SpanAttrV39 {
        kind: SpanAttrV39Kind::Flag {
            key: "rust".to_string(),
        },
    };
    Ok((source, attr))
}

// pub fn code_shorthand_key_value_attr_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
//     Ok((source, "".to_string()))
// }
