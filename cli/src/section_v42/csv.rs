use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::end::end_section_basic_v42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::space0_line_ending_or_eof;
use ::csv::ReaderBuilder;
use minijinja::Value;
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
use serde_json::json;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CsvSectionV42 {
    pub attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42>,
    pub bound: Bound,
    pub data: Value,
    pub end_section: Option<Box<SectionV42>>,
    pub error: Option<String>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn csv_section_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let (source, result) = alt((
        |src| csv_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| csv_section_start_v42(src, &site_config, parent.clone(), debug),
    ))
    .context("")
    .parse(source)?;
    Ok((source, result))
}

pub fn csv_section_full_v42<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "CSV_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, kind) = alt((
        terminated(tag("csv"), space0_line_ending_or_eof),
        terminated(tag("csv-table"), space0_line_ending_or_eof),
    ))
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, csv_text) = alt((take_until("-- "), rest)).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    // TODO: Handle Errors here
    let error = None;
    let data;
    if csv_text.trim() != "" {
        data = parse_csv(csv_text)
    } else {
        data = serde_json5::from_str::<Value>(r#"[]"#).unwrap()
    }
    let section = SectionV42::Csv(CsvSectionV42 {
        attrs,
        bound: Bound::Full,
        data,
        end_section: None,
        error,
        flags,
        kind: kind.to_string(),
        template_list: make_template_list("csv", kind, "default"),
    });
    Ok((source, section))
}

pub fn csv_section_start_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "CSV_SECTION_START";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, kind) = preceded(
        tuple((tag("--"), space1)),
        alt((
            terminated(tag("csv"), tuple((tag("/"), space0, line_ending))),
            terminated(tag("csv-table"), tuple((tag("/"), space0, line_ending))),
        )),
    )
    .context("")
    .parse(source)?;
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, csv_text) = take_until(format!("-- /{}", kind).as_str())
        .context("")
        .parse(source)?;
    let (source, end_section) =
        (|src| end_section_basic_v42(src, site_config, parent.clone(), kind, debug))
            .context("")
            .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    // TODO: Handle Errors here
    let error = None;
    let data;
    if csv_text.trim() != "" {
        data = parse_csv(csv_text)
    } else {
        data = serde_json5::from_str::<Value>(r#"[]"#).unwrap()
    }
    Ok((
        source,
        SectionV42::Csv(CsvSectionV42 {
            attrs,
            flags,
            bound: Bound::Start,
            data,
            end_section: Some(Box::new(end_section)),
            error,
            kind: kind.to_string(),
            template_list: make_template_list("csv", kind, "default"),
        }),
    ))
}

fn parse_csv(source: &str) -> Value {
    let mut items: Vec<Vec<String>> = vec![];
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_reader(source.as_bytes());
    for result in rdr.deserialize() {
        match result {
            Ok(record) => items.push(record),
            Err(_) => (),
        }
    }
    Value::from_serialize(json!(items))
}

#[cfg(test)]
mod test {
    use super::*;
    use include_dir::{include_dir, Dir};
    use lazy_static::lazy_static;
    use minijinja::Value;
    use pretty_assertions::assert_eq;
    use serde_json::Value as SerdeValue;
    use serde_json5;
    use std::collections::BTreeMap;

    const INPUT_DIR: Dir = include_dir!("src/section_v42/test_files/csv/valid/input");

    lazy_static! {
        static ref INPUT_FILES: BTreeMap<String, String> = {
            let glob = "**/*.neo";
            let mut btm = BTreeMap::new();
            for entry in INPUT_DIR.find(glob).unwrap() {
                let data = INPUT_DIR.get_file(entry.path());
                btm.insert(
                    entry
                        .path()
                        .display()
                        .to_string()
                        .trim_end_matches(".neo")
                        .to_string(),
                    data.expect(
                        format!(
                            "Could not open built-in include: {}",
                            entry.path().display().to_string()
                        )
                        .as_str(),
                    )
                    .contents_utf8()
                    .unwrap()
                    .to_string(),
                );
            }
            btm
        };
    }

    const OUTPUT_DIR: Dir = include_dir!("src/section_v42/test_files/csv/valid/output");

    lazy_static! {
        static ref OUTPUT_FILES: BTreeMap<String, String> = {
            let glob = "**/*.json5";
            let mut btm = BTreeMap::new();
            for entry in OUTPUT_DIR.find(glob).unwrap() {
                let data = OUTPUT_DIR.get_file(entry.path());
                btm.insert(
                    entry
                        .path()
                        .display()
                        .to_string()
                        .trim_end_matches(".json5")
                        .to_string(),
                    data.expect(
                        format!(
                            "Could not open built-in include: {}",
                            entry.path().display().to_string()
                        )
                        .as_str(),
                    )
                    .contents_utf8()
                    .unwrap()
                    .to_string(),
                );
            }
            btm
        };
    }

    #[test]
    fn csv_section_tests() {
        for (id, source) in INPUT_FILES.iter() {
            match OUTPUT_FILES.get(id) {
                Some(output) => {
                    match serde_json5::from_str::<SerdeValue>(output) {
                        Ok(json5) => {
                            let site_config = SiteConfigV42::mock1_basic();
                            let parent = SectionParent::Page;
                            let debug = false;
                            let left = Value::from_serialize(json5);
                            let right = Value::from_serialize(
                                csv_section_v42(source, &site_config, parent, debug)
                                    .unwrap()
                                    .1,
                            );
                            assert_eq!(left, right);
                        }
                        Err(e) => {
                            dbg!(e);
                            assert!(false)
                        }
                    };
                }
                None => {
                    assert!(false);
                }
            }
        }
    }

    //
}
