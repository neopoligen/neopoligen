pub mod basic;
pub mod block;
pub mod checklist;
pub mod checklist_item;
pub mod css;
pub mod csv;
pub mod end;
pub mod helpers;
pub mod html;
pub mod json5;
pub mod list;
pub mod list_item;
pub mod olist;
pub mod olist_item;
pub mod parsing_error;
pub mod raw;
pub mod script;

use crate::section_v42::basic::*;
use crate::section_v42::block::*;
use crate::section_v42::checklist::*;
use crate::section_v42::checklist_item::*;
use crate::section_v42::css::*;
use crate::section_v42::csv::*;
use crate::section_v42::end::*;
use crate::section_v42::html::*;
use crate::section_v42::json5::*;
use crate::section_v42::list::*;
use crate::section_v42::list_item::*;
use crate::section_v42::olist::*;
use crate::section_v42::olist_item::*;
use crate::section_v42::parsing_error::*;
use crate::section_v42::raw::*;
use crate::section_v42::script::*;
use crate::site_config::SiteConfigV42;
use minijinja::value::Object;
use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionParent {
    Basic,
    Checklist,
    ChecklistItem,
    List,
    ListItem,
    None,
    Olist,
    OlistItem,
    Page,
    SubChecklist,
    SubList,
    SubOlist,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionV42 {
    Basic(BasicSectionV42),
    Block(BlockSectionV42),
    Checklist(ChecklistSectionV42),
    ChecklistItem(ChecklistItemSectionV42),
    Css(CssSectionV42),
    Csv(CsvSectionV42),
    End(EndSectionV42),
    Html(HtmlSectionV42),
    Json5(Json5SectionV42),
    List(ListSectionV42),
    ListItem(ListItemSectionV42),
    Olist(OlistSectionV42),
    OlistItem(OlistItemSectionV42),
    ParsingError(ParsingErrorSectionV42),
    Raw(RawSectionV42),
    Script(ScriptSectionV42),
}

impl Object for SectionV42 {}

pub fn start_or_full_section_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let (source, section) = alt((
        //TODO: Move sections to single calls here for easier testing
        |src| checklist_section_v42(src, &site_config, parent.clone(), debug),
        |src| css_section_start_v42(src, &site_config, parent.clone(), debug),
        |src| css_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| csv_section_v42(src, &site_config, parent.clone(), debug),
        |src| html_section_start_v42(src, &site_config, parent.clone(), debug),
        |src| html_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| json5_section_start_v42(src, &site_config, parent.clone(), debug),
        |src| json5_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| list_section_start_v42(src, &site_config, parent.clone(), debug),
        |src| list_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| olist_section_v42(src, &site_config, parent.clone(), debug),
        |src| raw_section_start_v42(src, &site_config, parent.clone(), debug),
        |src| raw_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| script_section_start_v42(src, &site_config, parent.clone(), debug),
        |src| script_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| basic_section_v42(src, &site_config, parent.clone(), debug),
    ))
    .context("")
    .parse(source)?;
    Ok((source, section))
}
