use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::space0_line_ending_or_eof;
use ::toml::Value as TomlValue;
use minijinja::Value;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::fail;
use nom::combinator::rest;
use nom::sequence::pair;
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
pub struct TomlSectionV42 {
    pub attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42>,
    pub bound: Bound,
    pub data: Option<Value>,
    pub end_section: Option<Box<SectionV42>>,
    pub error: Option<String>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn toml_section_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let (source, result) = alt((
        |src| toml_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| toml_section_start_v42(src, &site_config, parent.clone(), debug),
    ))
    .context("")
    .parse(source)?;
    Ok((source, result))
}

pub fn toml_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "TOML_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, kind) =
        site_config
            .section_types
            .get("toml")
            .unwrap()
            .iter()
            .fold(fail(source), |acc, item| match acc {
                Ok(v) => Ok(v),
                Err(_) => terminated(tag(item.as_str()), space0_line_ending_or_eof)
                    .context("")
                    .parse(source),
            })?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, raw_text) = alt((take_until("-- "), rest)).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    if let Ok(toml_value) = raw_text.parse::<TomlValue>() {
        if let Ok(data) = toml_value.try_into::<Value>() {
            let section = SectionV42::Toml(TomlSectionV42 {
                attrs,
                bound: Bound::Full,
                data: Some(data),
                end_section: None,
                error: None,
                flags,
                kind: kind.to_string(),
                template_list: make_template_list("toml", kind, "default"),
            });
            Ok((source, section))
        } else {
            let section = SectionV42::Toml(TomlSectionV42 {
                attrs,
                bound: Bound::Full,
                data: None,
                end_section: None,
                error: Some("Could not parse toml".to_string()),
                flags,
                kind: kind.to_string(),
                template_list: make_template_list("toml", kind, "default"),
            });
            Ok((source, section))
        }
    } else {
        let section = SectionV42::Toml(TomlSectionV42 {
            attrs,
            bound: Bound::Full,
            data: None,
            end_section: None,
            error: Some("Could not parse toml".to_string()),
            flags,
            kind: kind.to_string(),
            template_list: make_template_list("toml", kind, "default"),
        });
        Ok((source, section))
    }
}

pub fn toml_section_start_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "TOML_SECTION_START";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
    let (source, kind) =
        site_config
            .section_types
            .get("toml")
            .unwrap()
            .iter()
            .fold(fail(source), |acc, item| match acc {
                Ok(v) => Ok(v),
                Err(_) => terminated(tag(item.as_str()), tuple((tag("/"), space0, line_ending)))
                    .context("")
                    .parse(source),
            })?;
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, raw_text) = take_until(format!("-- /{}", kind).as_str())
        .context("")
        .parse(source)?;
    let (source, end_section) =
        (|src| end_section_basic_v42(src, site_config, parent.clone(), kind, debug))
            .context("")
            .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let mut error = None;
    let data = Value::from_serialize(vec!["asdf"]);

    // if json_text.trim() != "" {
    //     data = match serde_json5::from_str::<Value>(json_text) {
    //         Ok(d) => d,
    //         Err(e) => {
    //             error = Some(e.to_string());
    //             serde_json5::from_str::<Value>(r#"{}"#).unwrap()
    //         }
    //     };
    // } else {
    //     data = serde_json5::from_str::<Value>(r#"{}"#).unwrap()
    // }

    Ok((
        source,
        SectionV42::Toml(TomlSectionV42 {
            attrs,
            flags,
            bound: Bound::Start,
            data: None,
            end_section: Some(Box::new(end_section)),
            error,
            kind: kind.to_string(),
            template_list: make_template_list("toml", kind, "default"),
        }),
    ))
}

