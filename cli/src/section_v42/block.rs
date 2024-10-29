use crate::helpers::dump_parser_if_debug;
use crate::helpers::flatten_spans_v42;
use crate::section_v42::SectionV42;
use crate::site_config::SiteConfigV42;
use crate::span_v42::SpanV42;
use crate::spans_v42::generic_block_content::spans_of_generic_block_content;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

use super::SectionParent;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BlockSectionV42 {
    // Reminder: there won't be children for a block, but having
    // the field keeps the API consistent which is most important
    pub children: Vec<SectionV42>,
    pub kind: String,
    pub spans: Vec<SpanV42>,
    pub text: String,
    pub template_list: Vec<String>,
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
    let (source, spans) = spans_of_generic_block_content.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, _) = multispace0.context("").parse(source)?;
    let text = flatten_spans_v42(&spans);
    Ok((
        source,
        SectionV42::Block(BlockSectionV42 {
            children: vec![],
            kind: "paragraph".to_string(),
            spans,
            text,
            template_list: vec!["sections/block/paragraph/full/default.neoj".to_string()],
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
        let template_list = vec!["sections/block/paragraph/full/default.neoj".to_string()];
        let left = (
            "",
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "this is some text".to_string(),
                })],
                text: "this is some text".to_string(),
                template_list,
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
        let template_list = vec!["sections/block/paragraph/full/default.neoj".to_string()];
        let left = (
            "",
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "paragraph".to_string(),
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
                text: "multiple lines".to_string(),
                template_list,
            }),
        );
        let right =
            paragraph_block_section_v42(&source, &site_config, SectionParent::Page, false).unwrap();
        assert_eq!(left, right);
    }

    //
}
