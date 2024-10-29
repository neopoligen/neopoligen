use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::end::end_section_basic_v42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::space0_line_ending_or_eof;
use minijinja::Value;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::space1;
use nom::combinator::rest;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Json5SectionV42 {
    pub attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42>,
    pub bounds: Bound,
    pub children: Vec<SectionV42>,
    pub data: Value,
    pub error: Option<String>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn json5_section_full_v42<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "JSON_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, kind) = alt((
        tag("pagedata"),
        tag("json5-test"),
        tag("dev"),
        tag("neopoligen-spec-test"),
    ))
    .context("")
    .parse(source)?;
    let (source, _) = space0_line_ending_or_eof.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, json_text) = alt((take_until("-- "), rest)).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let mut error = None;
    let data;
    if json_text.trim() != "" {
        data = match serde_json5::from_str::<Value>(json_text) {
            Ok(d) => d,
            Err(e) => {
                error = Some(e.to_string());
                serde_json5::from_str::<Value>(r#"{}"#).unwrap()
            }
        };
    } else {
        data = serde_json5::from_str::<Value>(r#"{}"#).unwrap()
    }
    let section = SectionV42::Json5(Json5SectionV42 {
        attrs,
        bounds: Bound::Full,
        children: vec![],
        data,
        error,
        flags,
        kind: kind.to_string(),
        template_list: make_template_list("json5", kind, "full", "default"),
    });
    Ok((source, section))
}

pub fn json5_section_start_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "JSON_SECTION_START";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, kind) = preceded(
        tuple((tag("--"), space1)),
        alt((
            tag("pagedata"),
            tag("json5-test"),
            tag("dev"),
            tag("neopoligen-spec-test"),
        )),
    )
    .context("")
    .parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, json_text) = take_until(format!("-- /{}", kind).as_str())
        .context("")
        .parse(source)?;
    let (source, end_section) =
        (|src| end_section_basic_v42(src, site_config, SectionParent::Basic, kind, debug))
            .context("")
            .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let mut error = None;
    let data;
    if json_text.trim() != "" {
        data = match serde_json5::from_str::<Value>(json_text) {
            Ok(d) => d,
            Err(e) => {
                error = Some(e.to_string());
                serde_json5::from_str::<Value>(r#"{}"#).unwrap()
            }
        };
    } else {
        data = serde_json5::from_str::<Value>(r#"{}"#).unwrap()
    }
    Ok((
        source,
        SectionV42::Json5(Json5SectionV42 {
            attrs: BTreeMap::new(),
            flags: vec![],
            bounds: Bound::Start,
            data,
            children: vec![end_section],
            error,
            kind: kind.to_string(),
            template_list: make_template_list("json5", kind, "start", "default"),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        bound::Bound,
        span_v42::{plaintext::PlainTextSpan, SpanV42},
    };
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLE_DIR: Dir = include_dir!("src/section_v42/samples/json5");

    #[test]
    fn pagedata_with_extra_data() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("001-pagedata-without-extra-data.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();
        let mut attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
        attrs.insert(
            "id".to_string(),
            SectionKeyValueAttrPayloadV42 {
                text: "abcd1234".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "abcd1234".to_string(),
                })],
            },
        );
        attrs.insert(
            "status".to_string(),
            SectionKeyValueAttrPayloadV42 {
                text: "draft".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "draft".to_string(),
                })],
            },
        );
        let template_list = vec![
            "sections/json5/pagedata/full/default.neoj".to_string(),
            "sections/json5/generic/full/default.neoj".to_string(),
        ];
        let left = (
            "",
            SectionV42::Json5(Json5SectionV42 {
                attrs,
                bounds: Bound::Full,
                children: vec![],
                data: serde_json5::from_str::<Value>("{}").unwrap(),
                error: None,
                flags: vec![],
                kind: "pagedata".to_string(),
                template_list,
            }),
        );
        let right =
            json5_section_full_v42(&source, &site_config, SectionParent::Page, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_basic_start_end() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("003-start-end.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();
        let attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
        let template_list = vec![
            "sections/json5/pagedata/start/default.neoj".to_string(),
            "sections/json5/generic/start/default.neoj".to_string(),
        ];
        let left = (
            "",
            SectionV42::Json5(Json5SectionV42 {
                attrs,
                bounds: Bound::Start,
                children: vec![SectionV42::End(EndSectionV42 {
                    attrs: BTreeMap::new(),
                    flags: vec![],
                    bounds: Bound::End,
                    children: vec![],
                    kind: "pagedata".to_string(),
                    template_list: vec![
                        "sections/ends/pagedata/end/default.neoj".to_string(),
                        "sections/ends/generic/end/default.neoj".to_string(),
                    ],
                })],
                data: serde_json5::from_str::<Value>(r#"{ "alfa": "bravo"}"#).unwrap(),
                error: None,
                flags: vec![],
                kind: "pagedata".to_string(),
                template_list,
            }),
        );
        let right =
            json5_section_start_v42(&source, &site_config, SectionParent::Page, false).unwrap();
        assert_eq!(left, right);
    }

    //
}
