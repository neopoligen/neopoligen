use crate::section_attr_v42::flag::SectionFlagAttrV42;
use crate::section_attr_v42::key_value::SectionKeyValueAttrV42;
use crate::section_attr_v42::section_attr_v42;
use crate::section_attr_v42::SectionAttrV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::rest;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Json5SectionV42 {
    kind: String,
    attrs: Vec<SectionKeyValueAttrV42>,
    flags: Vec<SectionFlagAttrV42>,
}

pub fn json5_section_full_v42<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _nest_level: usize,
    _list_level: usize,
    _checklist_level: usize,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let _dbg_header = "BASIC_SECTION_START";
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, kind) = tag("pagedata").context("").parse(source)?;
    let (source, _) = line_ending.context("").parse(source)?;
    let (source, raw_attrs) = many0(section_attr_v42).context("").parse(source)?;
    let (source, _) = rest.context("").parse(source)?;

    let attrs = raw_attrs
        .iter()
        .filter_map(|attr| match attr {
            SectionAttrV42::KeyValueV42(kv) => Some(kv.clone()),
            _ => None,
        })
        .collect();

    let flags = raw_attrs
        .iter()
        .filter_map(|attr| match attr {
            SectionAttrV42::FlagV42(f) => Some(f.clone()),
            _ => None,
        })
        .collect();

    let section = SectionV42::Json5(Json5SectionV42 {
        attrs,
        flags,
        kind: kind.to_string(),
    });
    Ok((source, section))
}
