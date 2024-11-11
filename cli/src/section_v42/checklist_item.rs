use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChecklistItemSectionV42 {
    pub bound: Bound,
    pub checked: bool,
    pub checked_string: Option<String>,
    pub children: Vec<SectionV42>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn checklist_item_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
    kind: &str,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "CHECKLISTITEM_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("[").context("").parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, checked_string_raw) = terminated(opt(is_not("]")), pair(tag("]"), space1))
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, paragraphs) = many1(|src| {
        checklist_item_paragraph_block(src, site_config, SectionParent::ChecklistItem, debug, &kind)
    })
    .context("")
    .parse(source)?;
    let (checked, checked_string) = match checked_string_raw {
        Some(value) => (true, Some(value.to_string())),
        None => (false, None),
    };
    let kind_name = format!("{}-item", kind);
    let section = SectionV42::ChecklistItem(ChecklistItemSectionV42 {
        bound: Bound::Full,
        checked,
        checked_string,
        children: paragraphs,
        kind: kind_name.clone(),
        template_list: make_template_list("checklist-item", &kind_name, "default"),
    });
    Ok((source, section))
}

pub fn checklist_item_section_start_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
    kind: &str,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "CHECKLISTITEM_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("[").context("").parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, checked_string_raw) = terminated(opt(is_not("]")), tag("]"))
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, paragraphs) = many1(alt((
        |src| {
            checklist_item_paragraph_block(
                src,
                site_config,
                SectionParent::ChecklistItem,
                debug,
                &kind,
            )
        },
        |src| start_or_full_section_v42(src, site_config, SectionParent::ChecklistItem, debug),
    )))
    .context("")
    .parse(source)?;
    let (checked, checked_string) = match checked_string_raw {
        Some(value) => (true, Some(value.to_string())),
        None => (false, None),
    };
    let kind_name = format!("{}-item", kind);
    let section = SectionV42::ChecklistItem(ChecklistItemSectionV42 {
        bound: Bound::Full,
        checked,
        checked_string,
        children: paragraphs,
        kind: kind_name.clone(),
        template_list: make_template_list("checklist-item", &kind_name, "default"),
    });
    Ok((source, section))
}
