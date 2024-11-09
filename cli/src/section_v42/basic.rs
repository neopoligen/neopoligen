use super::helpers::make_template_list::*;
use crate::bound::Bound;
use crate::helpers::dump_parser_if_debug;
use crate::section_attr_v42::section_attr_v42;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use crate::span_v42::space0_line_ending_or_eof::*;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BasicSectionV42 {
    pub attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42>,
    pub bound: Bound,
    pub children: Vec<SectionV42>,
    pub end_section: Option<Box<SectionV42>>,
    pub flags: Vec<String>,
    pub kind: String,
    pub template_list: Vec<String>,
}

pub fn basic_section_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let (source, result) = alt((
        |src| basic_section_full_v42(src, &site_config, parent.clone(), debug),
        |src| basic_section_start_v42(src, &site_config, parent.clone(), debug),
    ))
    .context("")
    .parse(source)?;
    Ok((source, result))
}

pub fn basic_section_full_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "BASIC_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, kind) = terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let (source, children) = match parent {
        SectionParent::List => {
            many0(|src| list_item_paragraph_block(src, site_config, parent.clone(), debug))
                .context("")
                .parse(source)?
        }
        SectionParent::ListItem => {
            many0(|src| list_item_paragraph_block(src, site_config, parent.clone(), debug))
                .context("")
                .parse(source)?
        }
        _ => many0(|src| paragraph_block_section_v42(src, site_config, parent.clone(), debug))
            .context("")
            .parse(source)?,
    };
    let section = SectionV42::Basic(BasicSectionV42 {
        attrs,
        bound: Bound::Full,
        children,
        end_section: None,
        kind: kind.to_string(),
        flags,
        template_list: make_template_list("basic", kind, "default"),
    });
    Ok((source, section))
}

pub fn basic_section_start_v42<'a>(
    source: &'a str,
    site_config: &'a SiteConfigV42,
    parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    let dbg_header = "BASIC_SECTION_START";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, kind) = terminated(
        is_not(" /\t\r\n"),
        pair(tag("/"), space0_line_ending_or_eof),
    )
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, (attrs, flags)) = section_attr_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    // TODO: Handle if you're in a list or a checklist here.
    let (source, children) = match parent {
        SectionParent::ListItem => many0(alt((
            |src| list_item_paragraph_block(src, site_config, parent.clone(), debug),
            |src| start_or_full_section_v42(src, site_config, parent.clone(), debug),
        )))
        .context("")
        .parse(source)?,
        SectionParent::List => many0(alt((
            |src| list_item_paragraph_block(src, site_config, parent.clone(), debug),
            |src| start_or_full_section_v42(src, site_config, parent.clone(), debug),
        )))
        .context("")
        .parse(source)?,
        _ => many0(alt((
            |src| paragraph_block_section_v42(src, site_config, parent.clone(), debug),
            |src| start_or_full_section_v42(src, site_config, parent.clone(), debug),
        )))
        .context("")
        .parse(source)?,
    };
    // let (source, children) = many0(alt((
    //     |src| paragraph_block_section_v42(src, site_config, SectionParent::Basic, debug),
    //     |src| start_or_full_section_v42(src, site_config, SectionParent::Basic, debug),
    // )))
    // .context("")
    // .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 6);
    let (source, end_section) =
        (|src| end_section_basic_v42(src, site_config, parent.clone(), kind, debug))
            .context("")
            .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 7);
    let section = SectionV42::Basic(BasicSectionV42 {
        attrs,
        bound: Bound::Start,
        children,
        end_section: Some(Box::new(end_section)),
        kind: kind.to_string(),
        flags,
        template_list: make_template_list("basic", kind, "default"),
    });
    Ok((source, section))
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

    const INPUT_DIR: Dir = include_dir!("src/section_v42/test_files/basic/valid/input");

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

    const OUTPUT_DIR: Dir = include_dir!("src/section_v42/test_files/basic/valid/output");

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
    fn solo_basic_section_tests() {
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
                                basic_section_v42(source.trim(), &site_config, parent, debug)
                                    .unwrap()
                                    .1,
                            );
                            assert_eq!(left, right, "Error in: {}", id);
                        }
                        Err(e) => {
                            dbg!(e);
                            assert!(false, "Error in: {}", id);
                        }
                    };
                }
                None => {
                    assert!(false, "Error in: {}", id);
                }
            }
        }
    }

    //
}
