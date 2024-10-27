pub mod basic;
pub mod block;
pub mod json5;

use crate::section_v42::basic::*;
use crate::section_v42::block::*;
use crate::section_v42::json5::*;
use crate::site_config::SiteConfigV42;
use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionV42 {
    Basic(BasicSectionV42),
    Block(BlockSectionV42),
    Json5(Json5SectionV42),
}

pub fn start_or_full_section_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    nest_level: usize,
    list_level: usize,
    checklist_level: usize,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let (source, section) = alt((
        |src| basic_section_full_v42(src, &site_config, nest_level, list_level, checklist_level),
        |src| json5_section_full_v42(src, &site_config, nest_level, list_level, checklist_level),
    ))
    .context("")
    .parse(source)?;

    // dbg!(&source);

    //let (source, section) = alt((
    //    |src| {
    //        basic_section_start(
    //            src,
    //            &sections,
    //            nest_level,
    //            list_level,
    //            checklist_level,
    //            debug,
    //        )
    //    },
    //    |src| {
    //        basic_section_full(
    //            src,
    //            &sections,
    //            nest_level,
    //            list_level,
    //            checklist_level,
    //            debug,
    //        )
    //    },
    //    // TODO: Checklist start
    //    //|src| checklist_section_start(src, &sections, nest_level, list_level),
    //    |src| {
    //        checklist_section_full(
    //            src,
    //            &sections,
    //            nest_level,
    //            list_level,
    //            checklist_level,
    //            debug,
    //        )
    //    },
    //    // TODO: Add json5 start
    //    |src| {
    //        json5_section_full(
    //            src,
    //            &sections,
    //            nest_level,
    //            list_level,
    //            checklist_level,
    //            debug,
    //        )
    //    },
    //    |src| {
    //        list_section_start(
    //            src,
    //            &sections,
    //            nest_level,
    //            list_level,
    //            checklist_level,
    //            debug,
    //        )
    //    },
    //    |src| {
    //        list_section_full(
    //            src,
    //            &sections,
    //            nest_level,
    //            list_level,
    //            checklist_level,
    //            debug,
    //        )
    //    },
    //    |src| {
    //        raw_section_start(
    //            src,
    //            &sections,
    //            nest_level,
    //            list_level,
    //            checklist_level,
    //            debug,
    //        )
    //    },
    //    |src| {
    //        raw_section_full(
    //            src,
    //            &sections,
    //            nest_level,
    //            list_level,
    //            checklist_level,
    //            debug,
    //        )
    //    },
    //    // Reminder: do unknown last since it slurps everything
    //    |src| {
    //        unknown_section_start(
    //            src,
    //            &sections,
    //            nest_level,
    //            list_level,
    //            checklist_level,
    //            debug,
    //        )
    //    },
    //    |src| {
    //        unknown_section_full(
    //            src,
    //            &sections,
    //            nest_level,
    //            list_level,
    //            checklist_level,
    //            debug,
    //        )
    //    },
    //))
    //.context("")
    //.parse(source)?;

    Ok((source, section))
}
