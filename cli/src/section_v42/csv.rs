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
    let (source, kind) = alt((tag("csv"),)).context("").parse(source)?;
    let (source, _) = space0_line_ending_or_eof.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, csv_text) = alt((take_until("-- "), rest)).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let error = None;
    let data;
    if csv_text.trim() != "" {
        data = parse_csv(csv_text)

        // let mut rdr = ReaderBuilder::new()
        //     .delimiter(b';')
        //     .from_reader(csv_text.as_bytes());

        //data = serde_json5::from_str::<Value>(r#"[]"#).unwrap()
        // data = match serde_json5::from_str::<Value>(json_text) {
        //     Ok(d) => d,
        //     Err(e) => {
        //         error = Some(e.to_string());
        //         serde_json5::from_str::<Value>(r#"{}"#).unwrap()
        //     }
        // };
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
    let (source, kind) = preceded(tuple((tag("--"), space1)), alt((tag("csv"),)))
        .context("")
        .parse(source)?;
    let (source, _) = tuple((tag("/"), space0, line_ending))
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
    let error = None;
    let data;
    if csv_text.trim() != "" {
        data = parse_csv(csv_text)
        // data = serde_json5::from_str::<Value>(r#"[]"#).unwrap()

        // data = match serde_json5::from_str::<Value>(json_text) {
        //     Ok(d) => d,
        //     Err(e) => {
        //         error = Some(e.to_string());
        //         serde_json5::from_str::<Value>(r#"{}"#).unwrap()
        //     }
        // };
    } else {
        data = serde_json5::from_str::<Value>(r#"[]"#).unwrap()
        //data = serde_json5::from_str::<Value>(r#"{}"#).unwrap()
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
    use crate::section_v42::SectionParent;
    use crate::{
        bound::Bound,
        span_v42::{plaintext::PlainTextSpan, SpanV42},
    };
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLE_DIR: Dir = include_dir!("src/section_v42/samples/csv/valid");

    #[test]
    fn solo_basic_csv() {
        let site_config = SiteConfigV42::mock1_basic();
        let parent = SectionParent::Page;
        let debug = false;
        let source = SAMPLE_DIR
            .get_file("001-basic-csv.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();
        let parsed = csv_section_full_v42(&source, &site_config, parent, debug).unwrap();
        let left = serde_json5::from_str::<serde_json::Value>(
            r#"{
"attrs": {}, "bound": "full", "end_section": null, "error": null, "flags": [], "kind": "csv", "type": "csv",
"template_list": ["sections/csv/default.neoj", "sections/generic-csv/default.neoj",
"built-in/sections/csv/default.neoj", "built-in/sections/generic-csv/default.neoj"
],
"data": [["a", "b", "c"], ["d", "e", "f"]]
        }"#,
        )
        .unwrap();
        let right = json!(parsed.1);
        assert_eq!(left, right);
    }

    // #[test]
    // fn pagedata_with_extra_data() {
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let source = SAMPLE_DIR
    //         .get_file("001-pagedata-without-extra-data.neo")
    //         .unwrap()
    //         .contents_utf8()
    //         .unwrap()
    //         .to_string();
    //     let mut attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
    //     attrs.insert(
    //         "id".to_string(),
    //         SectionKeyValueAttrPayloadV42 {
    //             text: "abcd1234".to_string(),
    //             spans: vec![SpanV42::PlainText(PlainTextSpan {
    //                 text: "abcd1234".to_string(),
    //             })],
    //         },
    //     );
    //     attrs.insert(
    //         "status".to_string(),
    //         SectionKeyValueAttrPayloadV42 {
    //             text: "draft".to_string(),
    //             spans: vec![SpanV42::PlainText(PlainTextSpan {
    //                 text: "draft".to_string(),
    //             })],
    //         },
    //     );
    //     let template_list = vec![
    //         "sections/pagedata/default.neoj".to_string(),
    //         "sections/generic-json5/default.neoj".to_string(),
    //     ];
    //     let left = (
    //         "",
    //         SectionV42::Json5(Json5SectionV42 {
    //             attrs,
    //             bound: Bound::Full,
    //             data: serde_json5::from_str::<Value>("{}").unwrap(),
    //             end_section: None,
    //             error: None,
    //             flags: vec![],
    //             kind: "pagedata".to_string(),
    //             template_list,
    //         }),
    //     );
    //     let right =
    //         json5_section_full_v42(&source, &site_config, SectionParent::Page, false).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn basic_start_end() {
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let source = SAMPLE_DIR
    //         .get_file("003-start-end.neo")
    //         .unwrap()
    //         .contents_utf8()
    //         .unwrap()
    //         .to_string();
    //     let attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
    //     let template_list = vec![
    //         "sections/pagedata/default.neoj".to_string(),
    //         "sections/generic-json5/default.neoj".to_string(),
    //     ];
    //     let left = (
    //         "",
    //         SectionV42::Json5(Json5SectionV42 {
    //             attrs,
    //             bound: Bound::Start,
    //             end_section: Some(Box::new(SectionV42::End(EndSectionV42 {
    //                 attrs: BTreeMap::new(),
    //                 flags: vec![],
    //                 bound: Bound::End,
    //                 children: vec![],
    //                 kind: "pagedata-end".to_string(),
    //                 template_list: vec![
    //                     "sections/pagedata-end/default.neoj".to_string(),
    //                     "sections/generic-end/default.neoj".to_string(),
    //                 ],
    //             }))),
    //             data: serde_json5::from_str::<Value>(r#"{ "alfa": "bravo"}"#).unwrap(),
    //             error: None,
    //             flags: vec![],
    //             kind: "pagedata".to_string(),
    //             template_list,
    //         }),
    //     );
    //     let right =
    //         json5_section_start_v42(&source, &site_config, SectionParent::Page, false).unwrap();
    //     assert_eq!(left, right);
    // }

    //
}
