pub mod basic;
pub mod block;
pub mod end;
pub mod helpers;
pub mod json5;
pub mod parsing_error;
pub mod raw;

use crate::section_v42::basic::*;
use crate::section_v42::block::*;
use crate::section_v42::end::*;
use crate::section_v42::json5::*;
use crate::section_v42::parsing_error::*;
use crate::section_v42::raw::*;
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
    List,
    Page,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionV42 {
    Basic(BasicSectionV42),
    Block(BlockSectionV42),
    End(EndSectionV42),
    Json5(Json5SectionV42),
    ParsingError(ParsingErrorSectionV42),
    Raw(RawSectionV42),
}

impl Object for SectionV42 {
    // fn call_method(
    //     self: &Arc<SectionV42>,
    //     _state: &minijinja::State,
    //     name: &str,
    //     _args: &[Value],
    // ) -> Result<Value, Error> {
    //     match name {
    //         _ => Ok(Value::from("")),
    //     }
    // }
}

pub fn start_or_full_section_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let (source, section) = alt((
        // |src| checklist_section_full_v42(src, &site_config, SectionParent::Page, debug),
        // |src| checklist_section_start_v42(src, &site_config, SectionParent::Page, debug),
        // |src| css_section_full_v42(src, &site_config, SectionParent::Page, debug),
        // |src| css_section_start_v42(src, &site_config, SectionParent::Page, debug),
        // |src| html_section_full_v42(src, &site_config, SectionParent::Page, debug),
        // |src| html_section_start_v42(src, &site_config, SectionParent::Page, debug),
        |src| json5_section_start_v42(src, &site_config, SectionParent::Page, debug),
        |src| json5_section_full_v42(src, &site_config, SectionParent::Page, debug),
        // |src| list_section_full_v42(src, &site_config, SectionParent::Page, debug),
        // |src| list_section_start_v42(src, &site_config, SectionParent::Page, debug),
        // |src| ordered_list_section_full_v42(src, &site_config, SectionParent::Page, debug),
        // |src| ordered_list_section_start_v42(src, &site_config, SectionParent::Page, debug),
        |src| raw_section_full_v42(src, &site_config, SectionParent::Page, debug),
        |src| raw_section_start_v42(src, &site_config, SectionParent::Page, debug),
        // |src| script_section_full_v42(src, &site_config, SectionParent::Page, debug),
        // |src| script_section_start_v42(src, &site_config, SectionParent::Page, debug),
        |src| basic_section_start_v42(src, &site_config, SectionParent::Page, debug),
        |src| basic_section_full_v42(src, &site_config, SectionParent::Page, debug),
    ))
    .context("")
    .parse(source)?;
    Ok((source, section))
}
