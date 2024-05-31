use crate::section_v39::SectionV39;
use crate::section_v39::SectionV39Bounds;
use crate::section_v39::SectionV39Kind;
use crate::span_v39::*;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockV39 {}

pub fn block_of_anything_v39<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
) -> IResult<&'a str, SectionV39, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, spans) = many0(|src| span_v39(src, spans))
        .context("")
        .parse(source)?;
    // let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Full,
            kind: SectionV39Kind::Block { spans },
            r#type: "block".to_string(),
        },
    ))
}

// pub fn block_of_end_content_v39<'a>(
//     source: &'a str,
//     spans: &'a Vec<String>,
// ) -> IResult<&'a str, SectionV39, ErrorTree<&'a str>> {
//     let (source, _) = not(eof).context("").parse(source)?;
//     let (source, _) = not(tag("-")).context("").parse(source)?;
//     let (source, _) = not(tag("[")).context("").parse(source)?;
//     let (source, spans) = many0(|src| span_finder(src, spans))
//         .context("")
//         .parse(source)?;
//     let (source, _) = multispace0.context("").parse(source)?;
//     Ok((source, SectionV39 {}))
// }
