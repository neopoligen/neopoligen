pub mod basic;
pub mod block;
pub mod checklist;
pub mod checklist_item;
pub mod json5;
pub mod list;
pub mod list_item;
pub mod mocks;
pub mod raw;
pub mod unknown;

use crate::section::basic::*;
use crate::section::checklist::*;
use crate::section::json5::*;
use crate::section::list::*;
use crate::section::raw::*;
use crate::section::unknown::*;
use crate::section_attr::SectionAttr;
use crate::section_attr::SectionAttrKind;
use crate::site_config::ConfigSections;
use crate::span::*;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Section {
    pub attrs: Vec<SectionAttr>,
    pub bounds: SectionBounds,
    pub kind: SectionKind,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionBounds {
    Full,
    Start,
    End,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionKind {
    Basic {
        children: Vec<Section>,
    },
    Block {
        spans: Vec<Span>,
    },
    Checklist {
        children: Vec<Section>,
    },
    ChecklistItem {
        checked: bool,
        checked_string: Option<String>,
        children: Vec<Section>,
    },
    Json5 {
        data: Value,
        children: Vec<Section>,
    },
    List {
        children: Vec<Section>,
    },
    ListItem {
        children: Vec<Section>,
    },
    Raw {
        children: Vec<Section>,
        text: Option<String>,
    },
    Unknown {
        children: Vec<Section>,
    },
}

impl Section {
    // DEPRECATED: I think this isn't needed and can be
    // removed when PayloadSection is working and pulling
    // data for itself
    pub fn get_attr(&self, target: &str) -> Option<String> {
        let attrs = self
            .attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SectionAttrKind::KeyValue { key, value } => {
                    if key.as_str() == target {
                        Some(value.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<String>>();
        if attrs.len() > 0 {
            Some(attrs.join(" "))
        } else {
            None
        }
    }

    pub fn output_kind(&self) -> Result<Value, Error> {
        match &self.kind {
            SectionKind::Unknown { .. } => Ok(Value::from("unknown")),
            SectionKind::List { .. } => Ok(Value::from("list")),
            SectionKind::ListItem { .. } => Ok(Value::from("list-item")),
            SectionKind::Checklist { .. } => Ok(Value::from("checklist")),
            SectionKind::ChecklistItem { .. } => Ok(Value::from("checklist-item")),
            SectionKind::Basic { .. } => Ok(Value::from("basic")),
            SectionKind::Block { .. } => Ok(Value::from("block")),
            SectionKind::Json5 { .. } => Ok(Value::from("json5")),
            SectionKind::Raw { .. } => Ok(Value::from("raw")),
        }
    }
}

pub fn start_or_full_section<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    nest_level: usize,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, section) = alt((
        |src| basic_section_start(src, &sections, nest_level),
        |src| basic_section_full(src, &sections, nest_level),
        // TODO: Checklist start
        //|src| checklist_section_start(src, &sections, nest_level),
        |src| checklist_section_full(src, &sections, nest_level),
        // TODO: Add json5 start
        |src| json5_section_full(src, &sections, nest_level),
        |src| list_section_start(src, &sections, nest_level),
        |src| list_section_full(src, &sections, nest_level),
        |src| raw_section_start(src, &sections, nest_level),
        |src| raw_section_full(src, &sections, nest_level),
        // Reminder: do unknown last since it slurps everything
        |src| unknown_section_start(src, &sections, nest_level),
        |src| unknown_section_full(src, &sections, nest_level),
    ))
    .context("")
    .parse(source)?;
    Ok((source, section))
}

pub fn initial_error<'a>() -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    // the purpose of this function is just to put an
    // error in the accumulator. There's a way to do that
    // with just making an error, but I haven't solved all
    // the parts to that yet.
    let (_, _) = tag("non-matching text")
        .parse("this will never match so it throws an intentional error")?;
    Ok(("", ""))
}

pub fn tag_finder_start<'a>(
    source: &'a str,
    section: &Vec<String>,
) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, result) = section
        .iter()
        .fold(initial_error(), |acc, item| match acc {
            Ok(v) => Ok(v),
            _ => tuple((tag(item.as_str()), tag("/"), space0, line_ending))
                .map(|x| x.0)
                .context("")
                .parse(source),
        })?;
    Ok((source, result))
}

pub fn tag_finder_full<'a>(
    source: &'a str,
    section: &Vec<String>,
) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, result) = section
        .iter()
        .fold(initial_error(), |acc, item| match acc {
            Ok(v) => Ok(v),
            _ => tuple((tag(item.as_str()), space0, line_ending))
                .map(|x| x.0)
                .context("")
                .parse(source),
        })?;
    Ok((source, result))
}

// pub fn tag_finder_end<'a>(
//     source: &'a str,
//     section: &Vec<String>,
// ) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
//     let (source, result) = section
//         .iter()
//         .fold(initial_error(), |acc, item| match acc {
//             Ok(v) => Ok(v),
//             _ => tuple((tag(item.as_str()), tag("/"), space0, line_ending))
//                 .map(|x| x.0)
//                 .context("")
//                 .parse(source),
//         })?;
//     Ok((source, result))
// }

impl Object for Section {
    fn call_method(
        self: &Arc<Section>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "kind" => self.output_kind(),
            "type" => Ok(Value::from(&self.r#type)),
            _ => Ok(Value::from("")),
        }
    }
}

#[cfg(test)]
mod test {
    //    use super::*;
    //   use crate::site_config::SiteConfig;
    // use pretty_assertions::assert_eq;
    //

    // // DEPRECATED: TODO: All attr processing will be done
    // // in PayloadSection
    // #[test]
    // fn get_attr_is_none_if_it_does_not_exist() {
    //     let section = Section::mock1_basic_title_section_no_attrs();
    //     let left = section.get_attr("key_that_does_not_exist");
    //     let right = None;
    //     assert_eq!(left, right);
    // }

    // // DEPRECATED: TODO: All attr processing will be done
    // // in PayloadSection
    // #[test]
    // #[ignore]
    // fn get_attr_that_does_exist() {
    //     let section = Section::mock2_div_with_title_and_template_attrs();
    //     let left = section.get_attr("template");
    //     let right = Some("template-from-attr".to_string());
    //     assert_eq!(left, right);
    // }

    // // DEPRECATED: TODO: All attr processing will be done
    // // in PayloadSection
    // #[test]
    // #[ignore]
    // fn get_attr_combined_attrs_with_the_same_key() {
    //     let section = Section::mock3_image_with_flag_and_multiple_attrs_with_same_key();
    //     let left = section.get_attr("alt");
    //     let right = Some("alfa bravo charlie delta".to_string());
    //     assert_eq!(left, right);
    // }

    // // DEPRECATED: TODO: All attr processing will be done
    // // in PayloadSection
    // #[test]
    // #[ignore]
    // fn misc_test() {
    //     // let source = include_str!("test_files/integration-1.neo");
    //     let source = include_str!("test_files/to-speed-check.neo");
    //     let config = SiteConfig::mock1_basic();
    //     let left = "";
    //     let right = many1(|src| start_or_full_section(src, &config.sections))(source)
    //         .unwrap()
    //         .0;
    //     dbg!(&right);
    //     assert_eq!(left, right);
    // }

    //
}
