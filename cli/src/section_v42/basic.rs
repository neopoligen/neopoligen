use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::*;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BasicSectionV42 {
    pub attrs: BTreeMap<String, Vec<SectionKeyValueAttrPayloadV42>>,
    pub bound: Bound,
    pub children: Vec<SectionV42>,
    pub end_section: Option<Box<SectionV42>>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn basic_section_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let (source, result) = alt((
        |src| basic_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| basic_section_start_v42(src, &site_config, parent.clone(), debug),
    ))
    .context("")
    .parse(source)?;
    Ok((source, result))
}

pub fn basic_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "BASIC_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, kind) = terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let (source, children) =
        many0(|src| paragraph_block_section_v42(src, site_config, SectionParent::Basic, debug))
            .context("")
            .parse(source)?;
    let template_list = make_template_list_v42("basic", kind, &attrs);
    let section = SectionV42::Basic(BasicSectionV42 {
        attrs,
        bound: Bound::Full,
        children,
        end_section: None,
        kind: kind.to_string(),
        flags,
        template_list,
        // template_list: vec![
        //     "sections/p/default.neoj".to_string(),
        //     "sections/basic/default.neoj".to_string(),
        // ],
    });
    Ok((source, section))
}

pub fn basic_section_start_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let dbg_header = "BASIC_SECTION_START";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, kind) = terminated(
        is_not(" /\t\r\n"),
        pair(tag("/"), space0_line_ending_or_eof),
    )
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let (source, children) = many0(alt((
        |src| paragraph_block_section_v42(src, site_config, SectionParent::Basic, debug),
        |src| start_or_full_section_v42(src, site_config, SectionParent::Basic, debug),
    )))
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 6);
    let (source, end_section) =
        (|src| end_section_basic_v42(src, site_config, parent.clone(), kind, debug))
            .context("")
            .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 7);
    let template_list = make_template_list_v42("basic", kind, &attrs);
    let section = SectionV42::Basic(BasicSectionV42 {
        attrs,
        bound: Bound::Start,
        children,
        end_section: Some(Box::new(end_section)),
        kind: kind.to_string(),
        flags,
        template_list,
        // template_list: vec!["sections/basic/default.neoj".to_string()],
    });
    Ok((source, section))
}
