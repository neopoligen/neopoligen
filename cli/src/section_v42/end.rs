use crate::bound::Bound;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EndSectionV42 {
    pub attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42>,
    pub bound: Bound,
    pub children: Vec<SectionV42>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn end_section_basic_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    kind: &str,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let _dbg_header = "END_SECTION_BASIC";
    let (source, _) = tuple((
        tag("--"),
        space1,
        tag("/"),
        tag(kind),
        space0_line_ending_or_eof,
    ))
    .context("")
    .parse(source)?;
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;

    // let (source, children) = match parent {
    //     SectionParent::ListItem => many0(alt((
    //         |src| list_item_paragraph_block(src, site_config, parent.clone(), debug),
    //         |src| start_or_full_section_v42(src, site_config, parent.clone(), debug),
    //     )))
    //     .context("")
    //     .parse(source)?,
    //     _ => many0(alt((
    //         |src| paragraph_block_section_v42(src, site_config, parent.clone(), debug),
    //         |src| start_or_full_section_v42(src, site_config, parent.clone(), debug),
    //     )))
    //     .context("")
    //     .parse(source)?,
    // };

    let (source, children) =
        many0(|src| paragraph_block_section_v42(src, site_config, parent.clone(), debug))
            .context("")
            .parse(source)?;

    let section = SectionV42::End(EndSectionV42 {
        attrs,
        bound: Bound::End,
        children,
        kind: format!("{}-end", kind.to_string()),
        flags,
        template_list: vec![
            format!("sections/{}-end/default.neoj", kind),
            "sections/generic-end/default.neoj".to_string(),
        ],
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

    const SAMPLES: Dir = include_dir!("src/section_v42/samples/end/valid");

    #[test]
    fn integration_001_basic_unknown() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = get_include_str_file(SAMPLES, "001-basic-unknown.neo");
        let attrs = BTreeMap::new();
        let bound = Bound::End;
        let block_templates = vec!["sections/block-paragraph/default.neoj".to_string()];
        let children = vec![
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "block-paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "Tango".to_string(),
                })],
                template_list: block_templates.clone(),
            }),
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "block-paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "Foxtrot".to_string(),
                })],
                template_list: block_templates.clone(),
            }),
        ];
        let kind = "unknown-test-section".to_string();
        let flags = vec![];
        let template_list = vec![
            "sections/unknown-test-section-end/default.neoj".to_string(),
            "sections/generic-end/default.neoj".to_string(),
        ];
        let left = (
            "",
            SectionV42::End(EndSectionV42 {
                attrs,
                bound,
                children,
                kind: format!("{}-end", kind.clone()),
                flags,
                template_list,
            }),
        );
        let right = end_section_basic_v42(
            &source,
            &site_config,
            SectionParent::Page,
            kind.as_str(),
            false,
        )
        .unwrap();
        assert_eq!(left, right);
    }

    //
}
