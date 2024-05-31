use crate::block::*;
use crate::section_attr_v39::*;
use crate::section_v39::block::*;
use crate::section_v39::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;

pub fn yaml_section_full_v39<'a>(
    source: &'a str,
    _sections: &'a Sections,
    _spans: &'a Vec<String>,
) -> IResult<&'a str, SectionV39, ErrorTree<&'a str>> {
    Ok((
        source,
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Full,
            kind: SectionV39Kind::Yaml {},
            r#type: "metadata".to_string(),
        },
    ))
}
