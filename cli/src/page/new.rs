use crate::ast::*;
use crate::child::Child;
use crate::config::Config;
use crate::page::*;
use std::path::PathBuf;

impl Page {
    pub fn parse_page(source_path: PathBuf, source: String, config: &Config) -> ParsedPage {
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
                        Some(_) => ParsedPage::Placeholder,
                        None => ParsedPage::Placeholder,
                    }

                    // let Some(id) = ast.iter().find_map(|child| {
                    //     if let Child::Section(section) = child {
                    //         let section_type = &section.r#type;
                    //         if section_type == "metadata" {
                    //             section
                    //                 .key_value_attributes
                    //                 .get("id")
                    //                 .map(|value| value.to_string())
                    //         } else {
                    //             None
                    //         }
                    //     } else {
                    //         None
                    //     }
                    // }) else {
                    //     return None;
                    // };

                    // ParsedPage::InvalidPage {
                    //     path: source_path.clone(),
                    //     remainder: Some(remainder.to_string()),
                    //     source: source.clone(),
                    //     error: Some("Could not complete parsing".to_string()),
                    // }
                } else {
                    ParsedPage::InvalidPage {
                        path: source_path.clone(),
                        remainder: Some(remainder.to_string()),
                        source: source.clone(),
                        error: Some("Could not complete parsing".to_string()),
                    }

                    // let Some(id) = ast.iter().find_map(|child| {
                    //     if let Child::Section(section) = child {
                    //         let section_type = &section.r#type;
                    //         if section_type == "metadata" {
                    //             section
                    //                 .key_value_attributes
                    //                 .get("id")
                    //                 .map(|value| value.to_string())
                    //         } else {
                    //             None
                    //         }
                    //     } else {
                    //         None
                    //     }
                    // }) else {
                    //     return None;
                    // };

                    // ParsedPage::ValidPage(Page {
                    //     ast,
                    //     id,
                    //     source,
                    //     source_path,
                    // })
                }
            }
            Err(e) => ParsedPage::InvalidPage {
                path: source_path.clone(),
                remainder: None,
                source: source.clone(),
                error: Some(format!("{}", e)),
            },
        }
    }

    pub fn new(source_path: PathBuf, source: String, config: &Config) -> Option<Page> {
        match ast(source.trim_start(), config) {
            Ok((remainder, _ast)) => {
                if remainder != "" {
                    println!(
                        "\n\nERROR: Failed to fully parse: {}",
                        source_path.display()
                    );
                    dbg!(remainder);
                }
            }
            Err(e) => println!("{}", e),
        };

        let Ok((_, ast)) = ast(source.trim_start(), config) else {
            return None;
        };

        let Some(id) = ast.iter().find_map(|child| {
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
        }) else {
            return None;
        };
        Some(Page {
            ast,
            id,
            source,
            source_path,
        })
    }
}
