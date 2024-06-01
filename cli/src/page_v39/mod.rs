pub mod mocks;
pub mod object;

use crate::ast_v39::parse;
use crate::neo_error::NeoError;
use crate::section_attr_v39::SectionAttrV39Kind;
use crate::section_v39::{SectionV39, SectionV39Kind};
use crate::site_config::SiteConfig;
use anyhow::Result;
use minijinja::Value;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone, Debug, Serialize)]
pub struct PageV39 {
    pub ast: Option<Vec<SectionV39>>,
    pub config: SiteConfig,
    pub errors: Vec<NeoError>,
    pub fs_modified: Option<SystemTime>,
    pub output_content: Option<String>,
    pub source_content: Option<String>,
    pub source_path: Option<PathBuf>,
}

impl PageV39 {
    pub fn new_from_fs(
        source_path: PathBuf,
        config: SiteConfig,
        source_content: String,
    ) -> Result<PageV39> {
        let fs_modified = fs::metadata(&source_path)?.modified()?;
        Ok(PageV39 {
            ast: None,
            config,
            errors: vec![],
            fs_modified: Some(fs_modified),
            output_content: None,
            source_content: Some(source_content),
            source_path: Some(source_path),
        })
    }
}

impl PageV39 {
    pub fn all_sections(&self) -> Result<Value, minijinja::Error> {
        Ok(Value::make_object_iterable(
            self.ast.clone().unwrap(),
            |sections| Box::new(sections.iter().cloned().map(Value::from_object)),
        ))
    }

    pub fn generate_ast(&mut self) -> Result<()> {
        match parse(
            &self.source_content.as_ref().unwrap(),
            &self.config.sections,
            &self.config.spans,
        ) {
            Ok(sections) => self.ast = Some(sections),
            Err(e) => self.errors.push(e),
        }
        Ok(())
    }

    pub fn get_metadata_attr(&self, target: &str) -> Option<String> {
        // Reminder: This only gets the first instance of the target attr
        // TODO: update so multiple instances of the same attr key
        // are joined like they are in section
        if let Some(ast) = &self.ast {
            ast.iter().find_map(|section| match &section.kind {
                SectionV39Kind::Yaml {} => section.attrs.iter().find_map(|attr| match &attr.kind {
                    SectionAttrV39Kind::KeyValue { key, value } => {
                        if key == target {
                            Some(value.to_string())
                        } else {
                            None
                        }
                    }
                    _ => None,
                }),
                _ => None,
            })
        } else {
            None
        }
    }

    pub fn id(&self) -> Option<String> {
        self.get_metadata_attr("id")
    }

    pub fn rel_output_path(&self) -> Option<PathBuf> {
        if let Some(path_override) = self.get_metadata_attr("path") {
            Some(PathBuf::from(format!("{}/index.html", path_override)))
        } else {
            if let (Ok(lang), Some(id)) = (self.config.default_language(), self.id()) {
                Some(PathBuf::from(format!("{}/{}/index.html", lang, id)))
            } else {
                None
            }
        }
    }

    pub fn rel_output_path_scrubber(&self, source: &str) -> Option<PathBuf> {
        if source == "/" {
            Some(PathBuf::from("index.html"))
        } else {
            None
        }

        // if let Some(path_override) = self.get_metadata_attr("path") {
        //     Some(PathBuf::from(format!("{}/index.html", path_override)))
        // } else {
        //     if let (Ok(lang), Some(id)) = (self.config.default_language(), self.id()) {
        //         Some(PathBuf::from(format!("{}/{}/index.html", lang, id)))
        //     } else {
        //         None
        //     }
        // }
    }

    pub fn rel_source_path(&self) -> Option<PathBuf> {
        let source_path = &self.source_path.clone().unwrap();
        if let Ok(rel_source_path) = source_path.strip_prefix(self.config.content_dir()) {
            Some(rel_source_path.to_path_buf())
        } else {
            None
        }
    }

    pub fn r#type(&self) -> Option<String> {
        if let Some(t) = self.get_metadata_attr("type") {
            Some(t)
        } else {
            Some("post".to_string())
        }
    }

    pub fn status(&self) -> Option<String> {
        if let Some(t) = self.get_metadata_attr("status") {
            Some(t)
        } else {
            Some("published".to_string())
        }
    }
}
