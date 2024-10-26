use nom::IResult;
use nom_supreme::error::ErrorTree;
use serde::Deserialize;
use serde::Serialize;

pub mod basic;

use crate::section_v42::basic::*;
use crate::site_config::SiteConfigV42;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SectionV42 {
    Basic(BasicSectionV42),
}

pub fn start_or_full_section_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    nest_level: usize,
    list_level: usize,
    checklist_level: usize,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
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

    Ok((source, SectionV42::Basic(BasicSectionV42 {})))
}
