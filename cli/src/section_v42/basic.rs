#![allow(unused_imports)]
use super::make_template_list::make_template_list;
use crate::section_attr_v42;
use crate::section_attr_v42::flag::SectionFlagAttrV42;
use crate::section_attr_v42::key_value::SectionKeyValueAttrV42;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::space0_line_ending_or_eof;
use crate::span_v42::SpanV42;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::combinator::rest;
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
    pub attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42>,
    pub bounds: String,
    pub children: Vec<SectionV42>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn basic_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    nest_level: usize,
    list_level: usize,
    checklist_level: usize,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let _dbg_header = "BASIC_SECTION_START";
    let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
    let (source, kind) = terminated(tag("title"), space0_line_ending_or_eof)
        .context("")
        .parse(source)?;
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| {
        generic_block_section_v42(src, site_config, nest_level, list_level, checklist_level)
    })
    .context("")
    .parse(source)?;
    let section = SectionV42::Basic(BasicSectionV42 {
        attrs,
        bounds: "full".to_string(),
        children,
        kind: kind.to_string(),
        flags,
        template_list: make_template_list("basic", kind, "full", "default"),
    });
    Ok((source, section))
}

#[cfg(test)]
mod test {

    // use crate::span_v42::{plaintext::PlainTextSpan, SpanV42};

    // use super::*;
    // use include_dir::{include_dir, Dir};
    // use pretty_assertions::assert_eq;

    // const SAMPLE_DIR: Dir = include_dir!("src/section_v42/samples/basic/valid");

    // #[test]
    // fn solo_pagedata_with_extra_data() {
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let source = SAMPLE_DIR
    //         .get_file("001-single-block.neo")
    //         .unwrap()
    //         .contents_utf8()
    //         .unwrap()
    //         .to_string();
    //     let left = (
    //         "",
    //         SectionV42::Basic(BasicSectionV42 {
    //             attrs: vec![],
    //             children: vec![],
    //             kind: "title".to_string(),
    //             flags: vec![],
    //         }),
    //     );
    //     let right = basic_section_full_v42(&source, &site_config, 0, 0, 0).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn solo_attrs_and_flags_test() {
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let source = SAMPLE_DIR
    //         .get_file("002-attrs-and-flags.neo")
    //         .unwrap()
    //         .contents_utf8()
    //         .unwrap()
    //         .to_string();
    //     let left = (
    //         "",
    //         SectionV42::Basic(BasicSectionV42 {
    //             attrs: vec![SectionKeyValueAttrV42 {
    //                 key: "test-key".to_string(),
    //                 value: vec![SpanV42::PlainText(PlainTextSpan {
    //                     output: "testValue".to_string(),
    //                 })],
    //             }],
    //             children: vec![],
    //             kind: "title".to_string(),
    //             flags: vec!["test-flag".to_string()],
    //         }),
    //     );
    //     let right = basic_section_full_v42(&source, &site_config, 0, 0, 0).unwrap();
    //     assert_eq!(left, right);
    // }

    //
}
