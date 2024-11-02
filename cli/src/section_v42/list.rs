use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::list_item::*;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
// use crate::span_v42::space0_line_ending_or_eof::space0_line_ending_or_eof;
// use nom::bytes::complete::is_not;
// use nom::sequence::terminated;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ListSectionV42 {
    pub attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42>,
    pub bound: Bound,
    pub children: Vec<SectionV42>,
    pub end_section: Option<Box<SectionV42>>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn list_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "LIST_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    // TODO: Get the list types dynamically
    let (source, kind) = tag("list").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let (source, children) =
        many0(|src| list_item_section_full_v42(src, site_config, parent.clone(), debug))
            .context("")
            .parse(source)?;
    let section = SectionV42::List(ListSectionV42 {
        attrs,
        bound: Bound::Full,
        children,
        end_section: None,
        kind: kind.to_string(),
        flags,
        template_list: make_template_list("list", kind, "default"),
    });
    Ok((source, section))
}

// pub fn list_section_start_v42<'a>(
//     source: &'a str,
//     site_config: &'a SiteConfigV42,
//     parent: SectionParent,
//     debug: bool,
// ) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
//     let dbg_header = "LIST_SECTION_START";
//     dump_parser_if_debug(debug, dbg_header, source, 1);
//     let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
//     dump_parser_if_debug(debug, dbg_header, source, 2);
//     // TODO: Get the list types dynamically
//     let (source, kind) = tag("list").context("").parse(source)?;
//     dump_parser_if_debug(debug, dbg_header, source, 3);
//     let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
//     dump_parser_if_debug(debug, dbg_header, source, 4);
//     let (source, _) = multispace0.context("").parse(source)?;
//     dump_parser_if_debug(debug, dbg_header, source, 5);
//     // TODO: Handle if you're in a list or a checklist here.
//     let (source, children) = many0(alt((
//         |src| paragraph_block_section_v42(src, site_config, SectionParent::List, debug),
//         |src| start_or_full_section_v42(src, site_config, SectionParent::List, debug),
//     )))
//     .context("")
//     .parse(source)?;
//     dump_parser_if_debug(debug, dbg_header, source, 6);
//     let (source, end_section) =
//         (|src| end_section_basic_v42(src, site_config, parent.clone(), kind, debug))
//             .context("")
//             .parse(source)?;
//     dump_parser_if_debug(debug, dbg_header, source, 7);
//     let section = SectionV42::List(ListSectionV42 {
//         attrs,
//         bound: Bound::Start,
//         children,
//         end_section: Some(Box::new(end_section)),
//         kind: kind.to_string(),
//         flags,
//         template_list: make_template_list("list", kind, "default"),
//     });
//     Ok((source, section))
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::span_v42::plaintext::PlainTextSpan;
    use crate::{bound::Bound, span_v42::SpanV42};
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLE_DIR: Dir = include_dir!("src/section_v42/samples/list");

    #[test]
    fn solo_basic_full() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("001-basic-full.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();
        let attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
        let template_list = vec![
            "sections/list/default.neoj".to_string(),
            "sections/generic-list/default.neoj".to_string(),
        ];
        let left = (
            "-- p\n\nasdf\n",
            SectionV42::List(ListSectionV42 {
                attrs,
                bound: Bound::Full,
                children: vec![
                    SectionV42::ListItem(ListItemSectionV42 {
                        bound: Bound::Full,
                        children: vec![SectionV42::Block(BlockSectionV42 {
                            children: vec![],
                            kind: "block-list-item-paragraph".to_string(),
                            spans: vec![SpanV42::PlainText(PlainTextSpan {
                                text: "alfa".to_string(),
                            })],
                            template_list: vec![
                                "sections/block-list-item-paragraph/default.neoj".to_string()
                            ],
                        })],
                        kind: "list-item".to_string(),
                        template_list: vec![
                            "sections/list-item/default.neoj".to_string(),
                            "sections/generic-list-item/default.neoj".to_string(),
                        ],
                    }),
                    SectionV42::ListItem(ListItemSectionV42 {
                        bound: Bound::Full,
                        children: vec![SectionV42::Block(BlockSectionV42 {
                            children: vec![],
                            kind: "block-list-item-paragraph".to_string(),
                            spans: vec![SpanV42::PlainText(PlainTextSpan {
                                text: "bravo".to_string(),
                            })],
                            template_list: vec![
                                "sections/block-list-item-paragraph/default.neoj".to_string()
                            ],
                        })],
                        kind: "list-item".to_string(),
                        template_list: vec![
                            "sections/list-item/default.neoj".to_string(),
                            "sections/generic-list-item/default.neoj".to_string(),
                        ],
                    }),
                ],
                end_section: None,
                flags: vec![],
                kind: "list".to_string(),
                template_list,
            }),
        );
        let right =
            list_section_full_v42(&source, &site_config, SectionParent::Page, false).unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // #[ignore]
    // fn basic_start_end() {
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let source = SAMPLE_DIR
    //         .get_file("002-start-end.neo")
    //         .unwrap()
    //         .contents_utf8()
    //         .unwrap()
    //         .to_string();
    //     let attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
    //     let template_list = vec![
    //         "sections/list/default.neoj".to_string(),
    //         "sections/generic-list/default.neoj".to_string(),
    //     ];
    //     let left = (
    //         "",
    //         SectionV42::List(ListSectionV42 {
    //             attrs,
    //             bound: Bound::Start,
    //             children: vec![],
    //             end_section: Some(Box::new(SectionV42::End(EndSectionV42 {
    //                 attrs: BTreeMap::new(),
    //                 flags: vec![],
    //                 bound: Bound::End,
    //                 children: vec![SectionV42::Block(BlockSectionV42 {
    //                     children: vec![],
    //                     kind: "block-paragraph".to_string(),
    //                     spans: vec![SpanV42::PlainText(PlainTextSpan {
    //                         text: "Post List".to_string(),
    //                     })],
    //                     template_list: vec!["sections/block-paragraph/default.neoj".to_string()],
    //                 })],
    //                 kind: "list-end".to_string(),
    //                 template_list: vec![
    //                     "sections/list-end/default.neoj".to_string(),
    //                     "sections/generic-end/default.neoj".to_string(),
    //                 ],
    //             }))),
    //             flags: vec![],
    //             kind: "list".to_string(),
    //             template_list,
    //         }),
    //     );
    //     let right =
    //         list_section_start_v42(&source, &site_config, SectionParent::Page, false).unwrap();
    //     assert_eq!(left, right);
    // }

    //
}
