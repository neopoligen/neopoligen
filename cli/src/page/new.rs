use crate::ast::*;
use crate::child::Child;
use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn new(source_path: PathBuf, source: String, config: &Config) -> Option<Page> {
        println!("Parsing: {}", source_path.display());

        match ast(source.trim_start(), config) {
            Ok((remainder, _ast)) => {
                if remainder != "" {
                    dbg!("failed to fully parse");
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
