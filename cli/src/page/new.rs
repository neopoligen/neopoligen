use crate::ast::*;
use crate::child::Child;
use crate::config::Config;
use crate::page::*;
use std::path::PathBuf;

impl Page {
    pub fn new(source_path: PathBuf, source: String, config: &Config) -> Option<Page> {
        match ast(source.trim_start(), config) {
            Ok((remainder, ast)) => {
                if remainder == "" {
                    match ast.iter().find_map(|child| {
                        if let Child::Section(section) = child {
                            let section_type = &section.r#type;
                            if section_type == "metadata" {
                                section
                                    .key_value_attributes
                                    .get("id")
                                    .map(|value| value.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }) {
                        Some(id) => Some(Page {
                            ast,
                            id,
                            source,
                            source_path,
                            title: Some("Home Page".to_string()),
                        }),
                        None => None,
                    }
                } else {
                    None
                }
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
