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

use crate::helpers::flatten_spans;
use crate::section::basic::*;
use crate::section::checklist::*;
use crate::section::json5::*;
use crate::section::list::*;
use crate::section::raw::*;
use crate::section::unknown::*;
use crate::section_attr::SectionAttr;
use crate::section_attr::SectionAttrKind;
// use crate::section_attr::SectionAttrKind;
use crate::site_config::ConfigSections;
use crate::span::*;
use minijinja::value::Object;
use minijinja::Error;
// use minijinja::ErrorKind;
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
    pub attr_list: Vec<String>,
    pub attrs: Vec<SectionAttr>,
    pub bounds: SectionBounds,
    pub flag_list: Vec<String>,
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
    End {
        children: Vec<Section>,
        closing: String,
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
    // TODO: Hoist attrs and flags from the settings
    pub fn attr_flag_list(&self, args: &[Value]) -> Result<Value, Error> {
        let mut class_list = self.classes();
        for i in 0..args.len() {
            class_list.push(args[i].to_string())
        }
        if class_list.len() > 0 {
            Ok(Value::from(format!(r#"class="{}""#, class_list.join(" "))))
        } else {
            Ok(Value::from(""))
        }
    }

    pub fn bounds(&self) -> Result<Value, Error> {
        match self.bounds {
            SectionBounds::End => Ok(Value::from("end")),
            SectionBounds::Full => Ok(Value::from("full")),
            SectionBounds::Start => Ok(Value::from("start")),
        }
    }

    pub fn children(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize::<Vec<Value>>(match &self.kind {
            SectionKind::Basic { children } => children
                .iter()
                .map(|c| Value::from_object(c.clone()))
                .collect(),
            SectionKind::Checklist { children } => children
                .iter()
                .map(|c| Value::from_object(c.clone()))
                .collect(),
            SectionKind::ChecklistItem { children, .. } => children
                .iter()
                .map(|c| Value::from_object(c.clone()))
                .collect(),
            SectionKind::End { children, .. } => children
                .iter()
                .map(|c| Value::from_object(c.clone()))
                .collect(),
            SectionKind::Json5 { children, .. } => children
                .iter()
                .map(|c| Value::from_object(c.clone()))
                .collect(),
            SectionKind::List { children } => children
                .iter()
                .map(|c| Value::from_object(c.clone()))
                .collect(),
            SectionKind::ListItem { children } => children
                .iter()
                .map(|c| Value::from_object(c.clone()))
                .collect(),
            SectionKind::Raw { children, .. } => children
                .iter()
                .map(|c| Value::from_object(c.clone()))
                .collect(),
            SectionKind::Unknown { children } => children
                .iter()
                .map(|c| Value::from_object(c.clone()))
                .collect(),
            _ => vec![],
        }))
    }

    // TOOD: Accept arguments for extra classes
    // to add (and to create the class attr if
    // it doesn't already exist)
    pub fn class_attr(&self) -> Result<Value, Error> {
        if self.classes().len() > 0 {
            Ok(Value::from(format!(
                r#"class="{}""#,
                self.classes().join(" ")
            )))
        } else {
            Ok(Value::from(""))
        }
    }

    pub fn classes(&self) -> Vec<String> {
        self.attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SectionAttrKind::KeyValueSpans { key, spans } => {
                    if key == "class" {
                        Some(flatten_spans(spans))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<String>>()
    }

    // // DEPRECATED: I think this isn't needed and can be
    // // removed when PayloadSection is working and pulling
    // // data for itself. And actually, I don't think it's
    // // needed when the move to V42 is done
    // pub fn get_attr(&self, target: &str) -> Option<String> {
    //     let attrs = self
    //         .attrs
    //         .iter()
    //         .filter_map(|attr| match &attr.kind {
    //             SectionAttrKind::KeyValue { key, value } => {
    //                 if key.as_str() == target {
    //                     Some(value.clone())
    //                 } else {
    //                     None
    //                 }
    //             }
    //             _ => None,
    //         })
    //         .collect::<Vec<String>>();
    //     if attrs.len() > 0 {
    //         Some(attrs.join(" "))
    //     } else {
    //         None
    //     }
    // }

    pub fn kind(&self) -> Result<Value, Error> {
        match &self.kind {
            SectionKind::Basic { .. } => Ok(Value::from("basic")),
            SectionKind::Block { .. } => Ok(Value::from("block")),
            SectionKind::Checklist { .. } => Ok(Value::from("checklist")),
            SectionKind::ChecklistItem { .. } => Ok(Value::from("checklist-item")),
            SectionKind::End { .. } => Ok(Value::from("end")),
            SectionKind::Json5 { .. } => Ok(Value::from("json5")),
            SectionKind::List { .. } => Ok(Value::from("list")),
            SectionKind::ListItem { .. } => Ok(Value::from("list-item")),
            SectionKind::Raw { .. } => Ok(Value::from("raw")),
            SectionKind::Unknown { .. } => Ok(Value::from("unknown")),
        }
    }

    pub fn spans(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize::<Vec<Value>>(match &self.kind {
            SectionKind::Block { spans } => spans
                .iter()
                .map(|s| Value::from_object(s.clone()))
                .collect(),
            _ => vec![],
        }))
    }

    // TODO: cascade back the templates through
    // the various defaults
    pub fn template_list(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(vec![
            format!(
                "sections/{}/{}/{}/{}.neoj",
                &self.kind().unwrap(),
                &self.r#type,
                &self.bounds().unwrap(),
                "default"
            ),
            "sections/unknown/generic/full/default.neoj".to_string(),
        ]))
    }

    pub fn text(&self) -> Result<Value, Error> {
        match &self.kind {
            SectionKind::Raw { text, .. } => Ok(Value::from(text.clone().unwrap())),
            _ => Ok(Value::from("")),
        }
    }

    //
}

pub fn start_or_full_section<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    nest_level: usize,
    list_level: usize,
    checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, section) = alt((
        |src| basic_section_start(src, &sections, nest_level, list_level, checklist_level, debug),
        |src| basic_section_full(src, &sections, nest_level, list_level, checklist_level, debug),
        // TODO: Checklist start
        //|src| checklist_section_start(src, &sections, nest_level, list_level),
        |src| checklist_section_full(src, &sections, nest_level, list_level, checklist_level, debug),
        // TODO: Add json5 start
        |src| json5_section_full(src, &sections, nest_level, list_level, checklist_level, debug),
        |src| list_section_start(src, &sections, nest_level, list_level, checklist_level, debug),
        |src| list_section_full(src, &sections, nest_level, list_level, checklist_level, debug),
        |src| raw_section_start(src, &sections, nest_level, list_level, checklist_level, debug),
        |src| raw_section_full(src, &sections, nest_level, list_level, checklist_level, debug),
        // Reminder: do unknown last since it slurps everything
        |src| unknown_section_start(src, &sections, nest_level, list_level, checklist_level, debug),
        |src| unknown_section_full(src, &sections, nest_level, list_level, checklist_level, debug),
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
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "attr_flag_list" => self.attr_flag_list(args),
            "bounds" => self.bounds(),
            "children" => self.children(),
            "class_attr" => self.class_attr(),
            "kind" => self.kind(),
            "spans" => self.spans(),
            "template_list" => self.template_list(),
            "text" => self.text(),
            "type" => Ok(Value::from(&self.r#type)),
            _ => Ok(Value::from("")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    //   use crate::site_config::SiteConfig;
    use pretty_assertions::assert_eq;
    //
    // TODO:
    // - bounds test

    #[test]
    #[ignore]
    fn attr_flag_list_none() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = section.attr_flag_list(&[]).unwrap();
        let right = Value::from("");
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn attr_flag_list_passing_value() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = section.attr_flag_list(&["ping-test".into()]).unwrap();
        let right = Value::from(r#"class="ping-test""#);
        assert_eq!(left, right);
    }

    // TODO: Add this back in
    #[test]
    #[ignore]
    fn attr_flag_list_from_attribute() {
        let section = Section::mock4_youtube_with_tags_and_classes();
        let left = section.attr_flag_list(&[]).unwrap();
        let right = Value::from(r#"class="alfa-class""#);
        assert_eq!(left, right);
    }

    // TODO: Add this back in
    #[test]
    #[ignore]
    fn attr_flag_list_from_attribute_and_passed_value() {
        let section = Section::mock4_youtube_with_tags_and_classes();
        let left = section.attr_flag_list(&["bravo-class".into()]).unwrap();
        let right = Value::from(r#"class="alfa-class bravo-class""#);
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn attr_flag_list_passing_a_class() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = section
            .attr_flag_list(&["foxtrot-test-class".into()])
            .unwrap();
        let right = Value::from(r#"class="foxtrot-test-class""#);
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn class_attr_none() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = section.class_attr().unwrap();
        let right = Value::from("");
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn class_attr_basic() {
        let section = Section::mock4_youtube_with_tags_and_classes();
        let left = section.class_attr().unwrap();
        let right = Value::from(r#"class="alfa-class""#);
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn classes_none() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = section.classes();
        let right: Vec<String> = vec![];
        assert_eq!(left, right);
    }

    // TODO: Add this back in when hyphens are working
    #[test]
    #[ignore]
    fn classes_basic() {
        let section = Section::mock4_youtube_with_tags_and_classes();
        let left = section.classes();
        let right = vec!["alfa-class".to_string()];
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn template_list_basic_check() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = section.template_list().unwrap();
        let right = Value::from_serialize::<Vec<String>>(vec![
            "sections/basic/title/full/default.neoj".to_string(),
            "sections/unknown/generic/full/default.neoj".to_string(),
        ]);
        assert_eq!(left, right);
    }

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
