use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::rest;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ScriptSectionV42 {
    pub attrs: BTreeMap<String, Vec<SectionKeyValueAttrPayloadV42>>,
    pub body: String,
    pub bound: Bound,
    pub end_section: Option<Box<SectionV42>>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn script_section_full_v42<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "SCRIPT_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    // TODO: Pull in keys dynamically
    let (source, kind) = alt((terminated(tag("script"), space0_line_ending_or_eof),))
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, body) = alt((take_until("-- "), rest)).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let section = SectionV42::Script(ScriptSectionV42 {
        attrs,
        bound: Bound::Full,
        end_section: None,
        flags,
        kind: kind.to_string(),
        template_list: make_template_list("script", kind, "default"),
        body: body.trim().to_string(),
    });
    Ok((source, section))
}

pub fn script_section_start_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "SCRIPT_SECTION_START";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    // TODO: Get section kind list dynamically
    let (source, kind) = preceded(
        tuple((tag("--"), space1)),
        alt((terminated(
            tag("script"),
            tuple((tag("/"), space0, line_ending)),
        ),)),
    )
    .context("")
    .parse(source)?;
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, body) = take_until(format!("-- /{}", kind).as_str())
        .context("")
        .parse(source)?;
    let (source, end_section) =
        (|src| end_section_basic_v42(src, site_config, parent.clone(), kind, debug))
            .context("")
            .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    Ok((
        source,
        SectionV42::Script(ScriptSectionV42 {
            attrs,
            flags,
            bound: Bound::Start,
            end_section: Some(Box::new(end_section)),
            kind: kind.to_string(),
            template_list: make_template_list("script", kind, "default"),
            body: body.trim().to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::span_v42::plaintext::PlainTextSpan;
    use crate::{bound::Bound, span_v42::SpanV42};
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLE_DIR: Dir = include_dir!("src/section_v42/samples/script");

    #[test]
    fn basic_full() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("001-basic-full.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();
        let attrs: BTreeMap<String, Vec<SectionKeyValueAttrPayloadV42>> = BTreeMap::new();
        let left = (
            "",
            SectionV42::Script(ScriptSectionV42 {
                attrs,
                body: "console.log('alfa')".to_string(),
                bound: Bound::Full,
                end_section: None,
                flags: vec![],
                kind: "script".to_string(),
                template_list: make_template_list("script", "script", "default"),
            }),
        );
        let right =
            script_section_full_v42(&source, &site_config, SectionParent::Page, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn basic_start_end() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("002-start-end.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();
        let attrs: BTreeMap<String, Vec<SectionKeyValueAttrPayloadV42>> = BTreeMap::new();
        let left = (
            "",
            SectionV42::Script(ScriptSectionV42 {
                attrs,
                body: "console.log('bravo')".to_string(),
                bound: Bound::Start,
                end_section: Some(Box::new(SectionV42::End(EndSectionV42 {
                    attrs: BTreeMap::new(),
                    flags: vec![],
                    bound: Bound::End,
                    children: vec![SectionV42::Block(BlockSectionV42 {
                        children: vec![],
                        kind: "paragraph".to_string(),
                        spans: vec![SpanV42::PlainText(PlainTextSpan {
                            kind: "plain-text".to_string(),
                            text: "Post Script".to_string(),
                        })],
                        template_list: make_template_list("paragraph", "paragraph", "default"),
                    })],
                    kind: "script-end".to_string(),
                    template_list: make_template_list("end", "script-end", "default"),
                }))),
                flags: vec![],
                kind: "script".to_string(),
                template_list: make_template_list("script", "script", "default"),
            }),
        );
        let right =
            script_section_start_v42(&source.trim(), &site_config, SectionParent::Page, false)
                .unwrap();
        assert_eq!(left, right);
    }

    //
}
