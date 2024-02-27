// pub mod mocks;
pub mod new;
pub mod parse;

use crate::child::Child;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub id: String,
    pub ast: Vec<Child>,
    pub source: String,
    pub source_path: PathBuf,
}

impl Page {
    // pub fn id(&self) -> Option<String> {
    //     self.ast.iter().find_map(|child| {
    //         if let Child::Section(section) = child {
    //             let section_type = &section.r#type;
    //             if section_type == "metadata" {
    //                 // dbg!("---------------", &section.key_value_attributes, "--------------");
    //                 section
    //                     .key_value_attributes
    //                     .get("id")
    //                     .map(|value| value.to_string())
    //             } else {
    //                 None
    //             }
    //         } else {
    //             None
    //         }
    //     })
    // }
}

#[cfg(test)]
mod page_functions {
    // use super::*;
    // use pretty_assertions::assert_eq;

    // #[test]
    // fn test_id() {
    //     let page = Page::s1_index();
    //     let left = Some("id_index".to_string());
    //     let right = page.id();
    //     assert_eq!(left, right);
    // }
}
