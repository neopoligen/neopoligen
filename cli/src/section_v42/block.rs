use super::helpers::make_template_list::*;
use crate::helpers::flatten_spans_v42;
use crate::helpers::set_from_vec;
use crate::section_v42::SectionV42;
use crate::site_config::SiteConfigV42;
use crate::span_v42::SpanV42;
use crate::spans_v42::generic_block_content::spans_of_generic_block_content;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BlockSectionV42 {
    // Reminder: there won't be children for a block, but having
    // the field keeps the API consistent which is most important
    pub children: Vec<SectionV42>,
    pub kind: String,
    pub spans: Vec<SpanV42>,
    pub text: String,
    pub template_list: BTreeSet<String>,
}

pub fn generic_block_section_v42<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _nest_level: usize,
    _list_level: usize,
    _checklist_level: usize,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let (source, spans) = spans_of_generic_block_content.context("").parse(source)?;
    let text = flatten_spans_v42(&spans);
    Ok((
        source,
        SectionV42::Block(BlockSectionV42 {
            children: vec![],
            kind: "generic".to_string(),
            spans,
            text,
            template_list: make_template_list("block", "generic", "full", "default"),
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
        let template_list = set_from_vec(vec!["sections/unknown/generic/full/default.neoj"]);
        let left = (
            "",
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "generic".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    output: "this is some text".to_string(),
                })],
                text: "this is some text".to_string(),
                template_list,
            }),
        );
        let right = generic_block_section_v42(&source, &site_config, 0, 0, 0).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_multiple_lines() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("002-multiple-lines.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .trim()
            .to_string();
        let template_list = set_from_vec(vec!["sections/block/generic/full/default.neoj"]);
        let left = (
            "",
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "generic".to_string(),
                spans: vec![
                    SpanV42::PlainText(PlainTextSpan {
                        output: "multiple".to_string(),
                    }),
                    SpanV42::PlainText(PlainTextSpan {
                        output: " ".to_string(),
                    }),
                    SpanV42::PlainText(PlainTextSpan {
                        output: "lines".to_string(),
                    }),
                ],
                text: "multiple lines".to_string(),
                template_list,
            }),
        );
        let right = generic_block_section_v42(&source, &site_config, 0, 0, 0).unwrap();
        assert_eq!(left, right);
    }

    //
}
