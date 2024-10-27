use super::make_template_list::make_template_list;
use crate::bound::Bound;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::space0_line_ending_or_eof;
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
pub struct UnknownSectionV42 {
    pub attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42>,
    pub bounds: Bound,
    pub children: Vec<SectionV42>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn unknown_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    nest_level: usize,
    list_level: usize,
    checklist_level: usize,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let _dbg_header = "UNKNOWN_SECTION_FULL";
    let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
    let (source, kind) = terminated(is_not(" \t\r\n"), space0_line_ending_or_eof)
        .context("")
        .parse(source)?;
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| {
        generic_block_section_v42(src, site_config, nest_level, list_level, checklist_level)
    })
    .context("")
    .parse(source)?;
    let section = SectionV42::Unknown(UnknownSectionV42 {
        attrs,
        bounds: Bound::Full,
        children,
        kind: kind.to_string(),
        flags,
        template_list: make_template_list("unknown", "generic", "full", "default"),
    });
    Ok((source, section))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        helpers::get_include_str_file,
        span_v42::{plaintext::PlainTextSpan, SpanV42},
    };
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLES: Dir = include_dir!("src/section_v42/samples/unknown/valid");

    #[test]
    fn solo_integration_001() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = get_include_str_file(SAMPLES, "001-basic.neo");
        let attrs = BTreeMap::new();
        let bounds = Bound::Full;
        let children = vec![
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "generic".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    output: "Content".to_string(),
                })],
                text: "Content".to_string(),
                template_list: vec!["sections/block/generic/full/default.neoj".to_string()],
            }),
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "generic".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    output: "Blocks".to_string(),
                })],
                text: "Blocks".to_string(),
                template_list: vec!["sections/block/generic/full/default.neoj".to_string()],
            }),
        ];
        let kind = "unknown-test-section".to_string();
        let flags = vec![];
        let template_list = vec!["sections/unknown/generic/full/default.neoj".to_string()];
        let left = (
            "",
            SectionV42::Unknown(UnknownSectionV42 {
                attrs,
                bounds,
                children,
                kind,
                flags,
                template_list,
            }),
        );
        let right = unknown_section_full_v42(&source, &site_config, 0, 0, 0).unwrap();
        assert_eq!(left, right);
    }

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
