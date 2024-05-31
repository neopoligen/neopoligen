#![allow(unused_imports)]

use crate::block::*;
use crate::block_v39::block_of_anything_v39;
use crate::section_attr_v39::*;
use crate::section_v39::*;
// use crate::sections::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;

pub fn basic_section_full<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, SectionV39<'a>, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.basic))
        .context("")
        .parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, attrs) = many0(section_attr_v39).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| block_of_anything_v39(src, &spans))
        .context("")
        .parse(source)?;

    let section = SectionV39 {
        attrs,
        bounds: SectionV39Bounds::Full,
        kind: SectionV39Kind::Basic {
            children,
            r#type: r#type.to_string(),
        },
    };
    Ok((source, section))

    // let mut attrs: BTreeMap<String, String> = BTreeMap::new();
    // let mut flags: Vec<String> = vec![];
    // raw_attrs.iter().for_each(|attr| match attr {
    //     SectionAttr::KeyValue { key, value } => {
    //         if attrs.contains_key(key) {
    //             let to_update = attrs.get_mut(key).unwrap();
    //             to_update.push_str(" ");
    //             to_update.push_str(value);
    //         } else {
    //             attrs.insert(key.to_string(), value.to_string());
    //         }
    //     }
    //     SectionAttr::Flag { key } => flags.push(key.to_string()),
    // });
    // Ok((
    //     source,
    //     Section::Basic {
    //         attrs,
    //         attr_list: raw_attrs,
    //         bounds: "full".to_string(),
    //         children,
    //         flags,
    //         r#type: r#type.to_string(),
    //     },
    // ))

    // let (source, _) = tag("-- ").context("").parse(source)?;
    // let (source, r#type) = (|src| tag_finder(src, &sections.basic))
    //     .context("")
    //     .parse(source)?;
    // let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    // let (source, raw_attrs) = many0(section_attr).context("").parse(source)?;
    // let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    // let (source, _) = multispace0.context("").parse(source)?;
    // let (source, children) = many0(|src| block_of_anything(src, &spans))
    //     .context("")
    //     .parse(source)?;
    // let mut attrs: BTreeMap<String, String> = BTreeMap::new();
    // let mut flags: Vec<String> = vec![];
    // raw_attrs.iter().for_each(|attr| match attr {
    //     SectionAttr::KeyValue { key, value } => {
    //         if attrs.contains_key(key) {
    //             let to_update = attrs.get_mut(key).unwrap();
    //             to_update.push_str(" ");
    //             to_update.push_str(value);
    //         } else {
    //             attrs.insert(key.to_string(), value.to_string());
    //         }
    //     }
    //     SectionAttr::Flag { key } => flags.push(key.to_string()),
    // });
    // Ok((
    //     source,
    //     Section::Basic {
    //         attrs,
    //         attr_list: raw_attrs,
    //         bounds: "full".to_string(),
    //         children,
    //         flags,
    //         r#type: r#type.to_string(),
    //     },
    // ))
}
