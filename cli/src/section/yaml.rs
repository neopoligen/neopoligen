use crate::block::*;
use crate::section::*;
use crate::sections::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn yaml_section_end<'a>(
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
            attrs.insert(key.to_string(), value.to_string());
            ()
        }
        SectionAttr::Flag { key } => flags.push(key.to_string()),
    });
    Ok((
        source,
        Section::Yaml {
            attrs,
            bounds: "end".to_string(),
            children,
            data: None,
            flags,
            r#type: r#type.to_string(),
        },
    ))
}

pub fn yaml_section_full<'a>(
    source: &'a str,
    sections: &'a Sections,
    _spans: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    // dbg!(&source);
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.yaml))
        .context("")
        .parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    // dbg!(&source);
    let (source, raw_attrs) = many0(section_attr).context("").parse(source)?;
    // dbg!(&source);
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    // dbg!(&source);
    // dbg!("a-----------");
    let (source, _) = multispace0
        .context("")
        .parse(source)?;
    // dbg!("-----------");
    // dbg!(&source);
    let (source, text) = alt((take_until("\n--"), rest, eof)).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let mut attrs: BTreeMap<String, String> = BTreeMap::new();
    let mut flags: Vec<String> = vec![];
    raw_attrs.iter().for_each(|attr| match attr {
        SectionAttr::KeyValue { key, value } => {
            attrs.insert(key.to_string(), value.to_string());
            ()
        }
        SectionAttr::Flag { key } => flags.push(key.to_string()),
    });
    Ok((
        source,
        Section::Yaml {
            attrs,
            bounds: "full".to_string(),
            children: vec![],
            data: Some(text.trim_end().to_string()),
            flags,
            r#type: r#type.to_string(),
        },
    ))
}

pub fn yaml_section_start<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.yaml))
        .context("")
        .parse(source)?;
    let end_key = format!("-- /{}", r#type);
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, raw_attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = many0(empty_until_newline_or_eof)
        .context("")
        .parse(source)?;
    let (source, text) = take_until(end_key.as_str()).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, end_section) = yaml_section_end(source, spans, r#type)?;
    let mut attrs: BTreeMap<String, String> = BTreeMap::new();
    let mut flags: Vec<String> = vec![];
    raw_attrs.iter().for_each(|attr| match attr {
        SectionAttr::KeyValue { key, value } => {
            attrs.insert(key.to_string(), value.to_string());
            ()
        }
        SectionAttr::Flag { key } => flags.push(key.to_string()),
    });
    Ok((
        source,
        Section::Yaml {
            attrs,
            bounds: "start".to_string(),
            children: vec![end_section],
            data: Some(text.trim_end().to_string()),
            flags,
            r#type: r#type.to_string(),
        },
    ))
}
