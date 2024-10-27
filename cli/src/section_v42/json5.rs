use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use minijinja::Value;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::combinator::rest;
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
    _nest_level: usize,
    _list_level: usize,
    _checklist_level: usize,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let _dbg_header = "BASIC_SECTION_START";
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, kind) = alt((tag("pagedata"), tag("json5-test"), tag("dev")))
        .context("")
        .parse(source)?;
    let (source, _) = line_ending.context("").parse(source)?;
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    let (source, json_text) = alt((take_until("-- "), rest)).context("").parse(source)?;
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

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        bound::Bound,
        span_v42::{plaintext::PlainTextSpan, SpanV42},
    };
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLE_DIR: Dir = include_dir!("src/section_v42/samples/json5/valid");

    #[test]
    fn solo_pagedata_with_extra_data() {
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
                    output: "abcd1234".to_string(),
                })],
            },
        );
        attrs.insert(
            "status".to_string(),
            SectionKeyValueAttrPayloadV42 {
                text: "draft".to_string(),
                spans: vec![SpanV42::PlainText(PlainTextSpan {
                    output: "draft".to_string(),
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
        let right = json5_section_full_v42(&source, &site_config, 0, 0, 0).unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // fn solo_basic_data_test() {
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let source = SAMPLE_DIR
    //         .get_file("002-basic-data.neo")
    //         .unwrap()
    //         .contents_utf8()
    //         .unwrap()
    //         .to_string();
    //     let left = (
    //         "",
    //         SectionV42::Json5(Json5SectionV42 {
    //             attrs: vec![],
    //             flags: vec![],
    //             kind: "json5-test".to_string(),
    //             data: serde_json5::from_str::<Value>(r#"{"alfa": "bravo"}"#).unwrap(),
    //             error: None,
    //             children: vec![],
    //         }),
    //     );
    //     let right = json5_section_full_v42(&source, &site_config, 0, 0, 0).unwrap();
    //     assert_eq!(left, right);
    // }

    //
}
