use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ListItemSectionV42 {
    pub bound: Bound,
    pub children: Vec<SectionV42>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn list_item_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "LISTITEM_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, paragraphs) =
        many1(|src| list_item_paragraph_block(src, site_config, SectionParent::Basic, debug))
            .context("")
            .parse(source)?;
    let section = SectionV42::ListItem(ListItemSectionV42 {
        bound: Bound::Full,
        children: paragraphs,
        kind: "list-item".to_string(),
        template_list: make_template_list("list-item", "list-item", "default"),
    });
    Ok((source, section))
}

// pub fn listitem_section_start_v42<'a>(
//     source: &'a str,
//     site_config: &'a SiteConfigV42,
//     _parent: SectionParent,
//     debug: bool,
// ) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
//     // let debug = true;
//     let dbg_header = "LISTITEM_SECTION_START";
//     dump_parser_if_debug(debug, dbg_header, source, 1);
//     // TODO: Get section kind list dynamically
//     let (source, kind) = preceded(tuple((tag("--"), space1)), alt((tag("listitem"),)))
//         .context("")
//         .parse(source)?;
//     let (source, _) = tuple((tag("/"), space0, line_ending))
//         .context("")
//         .parse(source)?;
//     let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
//     dump_parser_if_debug(debug, dbg_header, source, 2);
//     let (source, text) = take_until(format!("-- /{}", kind).as_str())
//         .context("")
//         .parse(source)?;
//     let (source, end_section) =
//         (|src| end_section_basic_v42(src, site_config, SectionParent::Basic, kind, debug))
//             .context("")
//             .parse(source)?;
//     dump_parser_if_debug(debug, dbg_header, source, 3);
//     Ok((
//         source,
//         SectionV42::ListItem(ListItemSectionV42 {
//             attrs,
//             flags,
//             bound: Bound::Start,
//             end_section: Some(Box::new(end_section)),
//             kind: kind.to_string(),
//             template_list: make_template_list("list-item", kind, "default"),
//             text: text.trim().to_string(),
//         }),
//     ))
// }

#[cfg(test)]
mod test {

    use super::*;
    use crate::span_v42::plaintext::PlainTextSpan;
    use crate::{bound::Bound, span_v42::SpanV42};
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLE_DIR: Dir = include_dir!("src/section_v42/samples/list-item");

    #[test]
    fn basic_full() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("001-basic-full.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();
        let template_list = vec![
            "sections/list-item/default.neoj".to_string(),
            "sections/generic-list-item/default.neoj".to_string(),
        ];
        let left = (
            "",
            SectionV42::ListItem(ListItemSectionV42 {
                bound: Bound::Full,
                children: vec![SectionV42::Block(BlockSectionV42 {
                    children: vec![],
                    kind: "block-list-item-paragraph".to_string(),
                    spans: vec![SpanV42::PlainText(PlainTextSpan {
                        text: "some text".to_string(),
                    })],
                    template_list: vec![
                        "sections/block-list-item-paragraph/default.neoj".to_string()
                    ],
                })],
                kind: "list-item".to_string(),
                template_list,
            }),
        );
        let right =
            list_item_section_full_v42(&source, &site_config, SectionParent::Page, false).unwrap();
        assert_eq!(left, right);
    }

    // // TODO: Update this for a nested test
    // #[test]
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
    //         "sections/list-item/default.neoj".to_string(),
    //         "sections/generic-list-item/default.neoj".to_string(),
    //     ];
    //     let left = (
    //         "",
    //         SectionV42::ListItem(ListItemSectionV42 {
    //             attrs,
    //             bound: Bound::Start,
    //             end_section: Some(Box::new(SectionV42::End(EndSectionV42 {
    //                 attrs: BTreeMap::new(),
    //                 flags: vec![],
    //                 bound: Bound::End,
    //                 children: vec![SectionV42::Block(BlockSectionV42 {
    //                     children: vec![],
    //                     kind: "block-paragraph".to_string(),
    //                     spans: vec![SpanV42::PlainText(PlainTextSpan {
    //                         text: "Post ListItem".to_string(),
    //                     })],
    //                     template_list: vec!["sections/block-paragraph/default.neoj".to_string()],
    //                 })],
    //                 kind: "list-item-end".to_string(),
    //                 template_list: vec![
    //                     "sections/list-item-end/default.neoj".to_string(),
    //                     "sections/generic-end/default.neoj".to_string(),
    //                 ],
    //             }))),
    //             flags: vec![],
    //             kind: "list-item".to_string(),
    //             template_list,
    //             text: "console.log('bravo')".to_string(),
    //         }),
    //     );
    //     let right =
    //         list_item_section_start_v42(&source, &site_config, SectionParent::Page, false).unwrap();
    //     assert_eq!(left, right);
    // }

    //
}
