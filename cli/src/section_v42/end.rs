use self::helpers::make_template_list::make_template_list;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::*;
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
    // let debug = true;
    let dbg_header = "END_SECTION_BASIC";
    dump_parser_if_debug(debug, dbg_header, source, 1);
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
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, children) = match parent {
        SectionParent::Checklist => many0(|src| {
            checklist_item_paragraph_block(src, site_config, parent.clone(), debug, &kind)
        })
        .context("")
        .parse(source)?,
        SectionParent::ChecklistItem => (source, vec![]),
        SectionParent::SubChecklist => (source, vec![]),
        //
        SectionParent::List => {
            many0(|src| list_item_paragraph_block(src, site_config, parent.clone(), debug, &kind))
                .context("")
                .parse(source)?
        }
        SectionParent::ListItem => (source, vec![]),
        SectionParent::SubList => (source, vec![]),
        //
        SectionParent::Olist => {
            many0(|src| olist_item_paragraph_block(src, site_config, parent.clone(), debug, &kind))
                .context("")
                .parse(source)?
        }
        SectionParent::OlistItem => (source, vec![]),
        SectionParent::SubOlist => (source, vec![]),
        //
        _ => many0(|src| paragraph_block_section_v42(src, site_config, parent.clone(), debug))
            .context("")
            .parse(source)?,
    };
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let section = SectionV42::End(EndSectionV42 {
        attrs,
        bound: Bound::End,
        children,
        kind: format!("{}-end", kind.to_string()),
        flags,
        template_list: make_template_list("end", format!("{}-end", kind).as_str(), "default"),
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
        let children = vec![
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    kind: "plain-text".to_string(),
                    text: "Tango".to_string(),
                })],
                template_list: make_template_list("paragraph", "paragraph", "default"),
            }),
            SectionV42::Block(BlockSectionV42 {
                children: vec![],
                kind: "paragraph".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    kind: "plain-text".to_string(),
                    text: "Foxtrot".to_string(),
                })],
                template_list: make_template_list("paragraph", "paragraph", "default"),
            }),
        ];
        let kind = "unknown-test-section".to_string();
        let flags = vec![];
        let left = (
            "",
            SectionV42::End(EndSectionV42 {
                attrs,
                bound,
                children,
                kind: format!("{}-end", kind.clone()),
                flags,
                template_list: make_template_list("end", "unknown-test-section-end", "default"),
            }),
        );
        let right = end_section_basic_v42(
            &source.trim(),
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
