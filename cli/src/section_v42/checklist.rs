use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::checklist_item::*;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::*;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::fail;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChecklistSectionV42 {
    pub attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42>,
    pub bound: Bound,
    pub children: Vec<SectionV42>,
    pub end_section: Option<Box<SectionV42>>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn checklist_section_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let (source, result) = alt((
        |src| checklist_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| checklist_section_start_v42(src, &site_config, parent.clone(), debug),
    ))
    .context("")
    .parse(source)?;
    Ok((source, result))
}

pub fn checklist_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let next_parent = match &parent {
        SectionParent::Page => SectionParent::Checklist,
        _ => SectionParent::SubChecklist,
    };
    let dbg_header = "CHECKLIST_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, kind) = site_config
        .section_types
        .get("checklist")
        .unwrap()
        .iter()
        .fold(fail(source), |acc, item| match acc {
            Ok(v) => Ok(v),
            Err(_) => tag(item.as_str()).context("").parse(source),
        })?;
    let (source, _) = space0_line_ending_or_eof.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let (source, children) = many0(|src| {
        checklist_item_section_full_v42(src, site_config, next_parent.clone(), debug, &kind)
    })
    .context("")
    .parse(source)?;
    let section = SectionV42::Checklist(ChecklistSectionV42 {
        attrs,
        bound: Bound::Full,
        children,
        end_section: None,
        kind: kind.to_string(),
        flags,
        template_list: make_template_list("checklist", kind, "default"),
    });
    Ok((source, section))
}

pub fn checklist_section_start_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let next_parent = match &parent {
        SectionParent::Page => SectionParent::Checklist,
        _ => SectionParent::SubChecklist,
    };
    let dbg_header = "CHECKLIST_SECTION_START";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, kind) = preceded(
        tuple((tag("--"), space1)),
        alt((terminated(
            tag("checklist"),
            tuple((tag("/"), space0, line_ending)),
        ),)),
    )
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    // TODO: Handle if you're in a checklist or a checklist here.
    let (source, children) = many0(alt((
        |src| checklist_item_section_start_v42(src, site_config, next_parent.clone(), debug, &kind),
        |src| start_or_full_section_v42(src, site_config, next_parent.clone(), debug),
    )))
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let (source, end_section) =
        (|src| end_section_basic_v42(src, site_config, next_parent.clone(), kind, debug))
            .context("")
            .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 6);
    let section = SectionV42::Checklist(ChecklistSectionV42 {
        attrs,
        bound: Bound::Start,
        children,
        end_section: Some(Box::new(end_section)),
        kind: kind.to_string(),
        flags,
        template_list: make_template_list("checklist", kind, "default"),
    });
    Ok((source, section))
}
