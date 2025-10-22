use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OlistItemSectionV42 {
    pub bound: Bound,
    pub children: Vec<SectionV42>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn olist_item_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
    kind: &str,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "OLISTITEM_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = alt((tag("- "), terminated(digit1, tag(". "))))
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, paragraphs) = many1(|src| {
        olist_item_paragraph_block(src, site_config, SectionParent::OlistItem, debug, &kind)
    })
    .context("")
    .parse(source)?;
    let kind_tag = format!("{}-item", kind);
    let section = SectionV42::OlistItem(OlistItemSectionV42 {
        bound: Bound::Full,
        children: paragraphs,
        kind: kind_tag.clone(),
        template_list: make_template_list("olist-item", &kind_tag, "default"),
    });
    Ok((source, section))
}

pub fn olist_item_section_start_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
    kind: &str,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "OLISTITEM_SECTION_START";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = alt((tag("- "), terminated(digit1, tag(". "))))
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, paragraphs) = many1(alt((
        |src| olist_item_paragraph_block(src, site_config, SectionParent::OlistItem, debug, kind),
        |src| start_or_full_section_v42(src, site_config, SectionParent::OlistItem, debug),
    )))
    .context("")
    .parse(source)?;
    let kind_tag = format!("{}-item", kind);
    let section = SectionV42::OlistItem(OlistItemSectionV42 {
        bound: Bound::Full,
        children: paragraphs,
        kind: kind_tag.clone(),
        template_list: make_template_list("olist-item", &kind_tag, "default"),
    });
    Ok((source, section))
}
