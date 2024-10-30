use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
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
    pub bound: Bound,
    pub children: Vec<SectionV42>,
    pub end_section: Option<Box<SectionV42>>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn unknown_section_start_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let dbg_header = "UNKNOWN_SECTION_START";
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
    // TODO: Handle if you're in a list or a checklist here.
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
    let section = SectionV42::Unknown(UnknownSectionV42 {
        attrs,
        bound: Bound::Start,
        children,
        end_section: Some(Box::new(end_section)),
        kind: kind.to_string(),
        flags,
        template_list: make_template_list("unknown", "generic", "start", "default"),
    });
    Ok((source, section))
}

pub fn unknown_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "UNKNOWN_SECTION_FULL";
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
        many0(|src| paragraph_block_section_v42(src, site_config, parent.clone(), debug))
            .context("")
            .parse(source)?;
    let section = SectionV42::Unknown(UnknownSectionV42 {
        attrs,
        bound: Bound::Full,
        children,
        end_section: None,
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
    use rstest::rstest;

    const VALID_SAMPLES: Dir = include_dir!("src/section_v42/samples/unknown/valid");

    #[rstest]
    #[case("003-check-preview.neo", "")]
    fn solo_integration_run_for_unknown_sections(#[case] file: &str, #[case] remainder: &str) {
        let debug = false;
        let site_config = SiteConfigV42::mock1_basic();
        let source = get_include_str_file(VALID_SAMPLES, file);
        let result =
            start_or_full_section_v42(&source, &site_config, SectionParent::Page, debug).unwrap();
        if result.clone().0 == remainder {
            assert!(true);
        } else {
            dbg!(result);
            assert!(false);
        }
    }

    #[test]
    fn integration_001_basic_full() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = get_include_str_file(VALID_SAMPLES, "001-basic-full.neo");
        let attrs = BTreeMap::new();
        let bound = Bound::Full;
        let block_templates = vec!["sections/block/paragraph/full/default.neoj".to_string()];
        let children = vec![
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "Content".to_string(),
                })],
                text: "Content".to_string(),
                template_list: block_templates.clone(),
            }),
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "Blocks".to_string(),
                })],
                text: "Blocks".to_string(),
                template_list: block_templates.clone(),
            }),
        ];
        let end_section = None;
        let kind = "unknown-test-section".to_string();
        let flags = vec![];
        let template_list = vec!["sections/unknown/generic/full/default.neoj".to_string()];
        let left = (
            "",
            SectionV42::Unknown(UnknownSectionV42 {
                attrs,
                bound,
                children,
                end_section,
                kind,
                flags,
                template_list,
            }),
        );
        let right =
            unknown_section_full_v42(&source, &site_config, SectionParent::Page, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn integration_002_basic_start_end() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = get_include_str_file(VALID_SAMPLES, "002-basic-start-end.neo");
        let attrs = BTreeMap::new();
        let bound = Bound::Start;
        let block_templates = vec!["sections/block/paragraph/full/default.neoj".to_string()];
        let children = vec![
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "Delta".to_string(),
                })],
                text: "Delta".to_string(),
                template_list: block_templates.clone(),
            }),
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "Tango".to_string(),
                })],
                text: "Tango".to_string(),
                template_list: block_templates.clone(),
            }),
        ];
        let end_section = Some(Box::new(SectionV42::End(EndSectionV42 {
            attrs: BTreeMap::new(),
            bound: Bound::End,
            children: vec![],
            flags: vec![],
            kind: "unknown-test-section".to_string(),
            template_list: vec![
                "sections/ends/unknown-test-section/end/default.neoj".to_string(),
                "sections/ends/generic/end/default.neoj".to_string(),
            ],
        })));
        let kind = "unknown-test-section".to_string();
        let flags = vec![];
        let template_list = vec!["sections/unknown/generic/start/default.neoj".to_string()];
        let left = (
            "",
            SectionV42::Unknown(UnknownSectionV42 {
                attrs,
                bound,
                children,
                end_section,
                kind,
                flags,
                template_list,
            }),
        );
        let right =
            unknown_section_start_v42(&source, &site_config, SectionParent::Page, false).unwrap();
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
    //                     text: "testValue".to_string(),
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
