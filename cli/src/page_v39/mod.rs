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

// #[derive(Clone, Debug)]
// pub struct PageV39Error {
//     kind: PageV39ErrorKind,
//     details: Option<String>,
// }

// #[derive(Clone, Debug)]
// pub enum PageV39ErrorKind {
//     ParserError {},
// }

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

    pub fn get_metadata_attr(&self, target: &str) -> Result<String> {
        let ast = self.ast.clone().ok_or(minijinja::Error::new(
            minijinja::ErrorKind::CannotUnpack,
            "can't get ast".to_string(),
        ))?;
        let response = ast
            .iter()
            .find_map(|section| match &section.kind {
                SectionV39Kind::Yaml {} => section.attrs.iter().find_map(|attr| match &attr.kind {
                    SectionAttrV39Kind::KeyValue { key, value } => {
                        if key == target {
                            Some(value)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }),
                _ => None,
            })
            .ok_or(minijinja::Error::new(
                minijinja::ErrorKind::CannotUnpack,
                format!("could not get metadata field: {}", target),
            ))?;
        Ok(response.to_string())
    }

    pub fn id(&self) -> Result<String> {
        let id = self.get_metadata_attr("id")?;
        Ok(id)
    }

    pub fn rel_output_path(&self) -> Result<PathBuf> {
        let lang = self.config.default_language()?;
        let id = self.id()?;
        Ok(PathBuf::from(format!("{}/{}/index.html", lang, id)))
    }

    pub fn rel_source_path(&self) -> Result<PathBuf> {
        let response = &self.source_path.clone().unwrap();
        let response = response.strip_prefix(self.config.content_dir())?;
        Ok(response.to_path_buf())
    }
}
