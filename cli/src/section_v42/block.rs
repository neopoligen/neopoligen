use crate::helpers::flatten_spans_v42;
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BlockSectionV42 {
    pub kind: String,
    pub spans: Vec<SpanV42>,
    pub text: String,
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
            kind: "generic".to_string(),
            spans,
            text,
        }),
    ))
}

#[cfg(test)]
mod test {
    use crate::span_v42::plaintext::PlainTextSpan;

    use super::*;
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLE_DIR: Dir = include_dir!("src/section_v42/samples/block/valid");

    #[test]
    fn solo_single_block_test() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("001-single-block.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();
        let left = (
            "",
            SectionV42::Block(BlockSectionV42 {
                kind: "generic".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    output: "this is some text".to_string(),
                })],
                text: "this is some text".to_string(),
            }),
        );
        let right = generic_block_section_v42(&source, &site_config, 0, 0, 0).unwrap();
        assert_eq!(left, right);
    }

    //
}
