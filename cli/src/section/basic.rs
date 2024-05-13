use crate::block::*;
use crate::section::*;
use crate::sections::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;

pub fn basic_section_end<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
    key: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, r#type) = tag(key).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, raw_attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| block_of_end_content(src, spans))
        .context("")
        .parse(source)?;
    let mut attrs: BTreeMap<String, String> = BTreeMap::new();
    let mut flags: Vec<String> = vec![];
    raw_attrs.iter().for_each(|attr| match attr {
        SectionAttr::KeyValue { key, value } => {
            if attrs.contains_key(key) {
                let to_update = attrs.get_mut(key).unwrap();
                to_update.push_str(" ");
                to_update.push_str(value);
            } else {
                attrs.insert(key.to_string(), value.to_string());
            }
        }
        SectionAttr::Flag { key } => flags.push(key.to_string()),
    });

    Ok((
        source,
        Section::Basic {
            attrs,
            bounds: "end".to_string(),
            children,
            flags,
            r#type: r#type.to_string(),
        },
    ))
}

pub fn basic_section_full<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.basic))
        .context("")
        .parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, raw_attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| block_of_anything(src, &spans))
        .context("")
        .parse(source)?;
    let mut attrs: BTreeMap<String, String> = BTreeMap::new();
    let mut flags: Vec<String> = vec![];
    raw_attrs.iter().for_each(|attr| match attr {
        SectionAttr::KeyValue { key, value } => {
            if attrs.contains_key(key) {
                let to_update = attrs.get_mut(key).unwrap();
                to_update.push_str(" ");
                to_update.push_str(value);
            } else {
                attrs.insert(key.to_string(), value.to_string());
            }
        }
        SectionAttr::Flag { key } => flags.push(key.to_string()),
    });
    Ok((
        source,
        Section::Basic {
            attrs,
            bounds: "full".to_string(),
            children,
            flags,
            r#type: r#type.to_string(),
        },
    ))
}

pub fn basic_section_start<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.basic))
        .context("")
        .parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, raw_attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, mut children) = many0(alt((
        |src| block_of_anything(src, &spans),
        |src| start_or_full_section(src, &sections, &spans),
    )))
    .context("")
    .parse(source)?;
    let (source, end_section) = basic_section_end(source, &spans, r#type)?;
    children.push(end_section);
    let mut attrs: BTreeMap<String, String> = BTreeMap::new();
    let mut flags: Vec<String> = vec![];
    raw_attrs.iter().for_each(|attr| match attr {
        SectionAttr::KeyValue { key, value } => {
            if attrs.contains_key(key) {
                let to_update = attrs.get_mut(key).unwrap();
                to_update.push_str(" ");
                to_update.push_str(value);
            } else {
                attrs.insert(key.to_string(), value.to_string());
            }
        }
        SectionAttr::Flag { key } => flags.push(key.to_string()),
    });

    Ok((
        source,
        Section::Basic {
            attrs,
            bounds: "start".to_string(),
            children,
            flags,
            r#type: r#type.to_string(),
        },
    ))
}
