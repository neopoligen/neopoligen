use super::helpers::make_template_list::make_template_list;
use super::SectionParent;
use crate::helpers::dump_parser_if_debug;
use crate::section_v42::SectionV42;
use crate::site_config::SiteConfigV42;
use crate::span_v42::SpanV42;
//use crate::spans_v42::generic_block_content::spans_of_generic_block_content;
use crate::spans_v42::generic_block_content_v42::generic_block_content_v42;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
//use nom::character::complete::space1;
use nom::combinator::not;
// use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BlockSectionV42 {
    // Reminder: there won't be children for a block, but having
    // the field keeps the API consistent which is most important
    // because it makes looping easier
    pub children: Vec<SectionV42>,
    pub kind: String,
    pub spans: Vec<SpanV42>,
    pub template_list: Vec<String>,
}

pub fn list_item_paragraph_block<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "LIST_ITEM_PARAGRAPH_BLOCK";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, spans) = generic_block_content_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        SectionV42::Block(BlockSectionV42 {
            children: vec![],
            kind: "block-list-item-paragraph".to_string(),
            spans,
            template_list: make_template_list(
                "list-item-paragraph",
                "list-item-paragraph",
                "default",
            ),
        }),
    ))
}

pub fn paragraph_block_section_v42<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "PARAGRAPH_BLOCK_SECTION";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, spans) = generic_block_content_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        SectionV42::Block(BlockSectionV42 {
            children: vec![],
            kind: "block-paragraph".to_string(),
            spans,
            template_list: make_template_list("block-paragraph", "block-paragraph", "default"),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span_v42::plaintext::PlainTextSpan;
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLE_DIR: Dir = include_dir!("src/section_v42/samples/block/valid");

    #[test]
    fn single_block_test() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("001-single-block.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .trim()
            .to_string();
        let left = (
            "",
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "block-paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "this is some text".to_string(),
                })],
                template_list: make_template_list("block-paragraph", "block-paragraph", "default"),
            }),
        );
        let right =
            paragraph_block_section_v42(&source, &site_config, SectionParent::Page, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn multiple_lines() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("002-multiple-lines.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .trim()
            .to_string();
        let left = (
            "",
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "block-paragraph".to_string(),
                spans: vec![
                    SpanV42::PlainText(PlainTextSpan {
                        text: "multiple".to_string(),
                    }),
                    SpanV42::PlainText(PlainTextSpan {
                        text: " ".to_string(),
                    }),
                    SpanV42::PlainText(PlainTextSpan {
                        text: "lines".to_string(),
                    }),
                ],
                template_list: make_template_list("block-paragraph", "block-paragraph", "default"),
            }),
        );
        let right =
            paragraph_block_section_v42(&source, &site_config, SectionParent::Page, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn list_item_single_line() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("003-list-item-single.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .trim()
            .to_string();
        let left = (
            "",
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "block-list-item-paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "alfa bravo".to_string(),
                })],
                template_list: make_template_list(
                    "list-item-paragraph",
                    "list-item-paragraph",
                    "default",
                ),
            }),
        );
        let right =
            list_item_paragraph_block(&source, &site_config, SectionParent::Page, false).unwrap();
        assert_eq!(left, right);
    }

    //
}
