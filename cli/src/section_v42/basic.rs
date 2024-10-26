// use crate::section_attr_v42::*;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BasicSectionV42 {}

pub fn basic_section_full_v42<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _nest_level: usize,
    _list_level: usize,
    _checklist_level: usize,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let _dbg_header = "BASIC_SECTION_START";
    let (source, _) = tag("-- ").context("").parse(source)?;
    //let (source, kind) = tag("pagedata").context("").parse(source)?;
    let section = SectionV42::Basic(BasicSectionV42 {});
    Ok((source, section))
}
