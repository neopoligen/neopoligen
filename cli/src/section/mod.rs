pub mod basic;
pub mod block;
pub mod mocks;
pub mod raw;
pub mod yaml;

use crate::section::basic::*;
use crate::section::yaml::*;
use crate::section_attr::SectionAttr;
use crate::section_attr::SectionAttrKind;
use crate::site_config::ConfigSections;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};

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
    Raw {
        children: Vec<Section>,
        text: Option<String>,
    },
    Yaml {},
}

impl Section {
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
}

pub fn start_or_full_section<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, section) = alt((
        |src| basic_section_full(src, &sections),
        |src| yaml_section_full(src, &sections),
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

pub fn tag_finder<'a>(
    source: &'a str,
    section: &Vec<String>,
) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, result) = section
        .iter()
        .fold(initial_error(), |acc, item| match acc {
            Ok(v) => Ok(v),
            _ => tag(item.as_str()).context("").parse(source),
        })?;
    Ok((source, result))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn get_attr_is_none_if_it_does_not_exist() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = section.get_attr("key_that_does_not_exist");
        let right = None;
        assert_eq!(left, right);
    }

    #[test]
    fn get_attr_that_does_exist() {
        let section = Section::mock2_div_with_title_and_template_attrs();
        let left = section.get_attr("template");
        let right = Some("template-from-attr".to_string());
        assert_eq!(left, right);
    }

    #[test]
    fn get_attr_combined_attrs_with_the_same_key() {
        let section = Section::mock3_image_with_flag_and_multiple_attrs_with_same_key();
        let left = section.get_attr("alt");
        let right = Some("alfa bravo charlie delta".to_string());
        assert_eq!(left, right);
    }
}
