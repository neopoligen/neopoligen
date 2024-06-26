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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PagePayload {
    pub id: Option<String>,
    pub status: Option<String>,
    pub r#type: Option<String>,
}

impl PagePayload {
    pub fn new() -> PagePayload {
        PagePayload {
            id: None,
            status: None,
            r#type: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PageV39 {
    pub ast: Option<Vec<SectionV39>>,
    pub config: SiteConfig,
    pub errors: Vec<NeoError>,
    pub fs_modified: Option<SystemTime>,
    pub output_content: Option<String>,
    pub source_content: Option<String>,
    pub source_path: Option<PathBuf>,
    // DEPRECATE: status in favor of payload
    pub status: Option<String>,
    // DEPRECATE: template_list in favor of payload
    pub template_list: Vec<String>,
    // DEPRECATE: type in favor of payload
    pub r#type: Option<String>,
    pub payload: Option<PagePayload>,
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
            status: None,
            template_list: vec![],
            r#type: None,
            payload: None,
        })
    }

    pub fn new_from_string(config: SiteConfig, source_content: String) -> Result<PageV39> {
        let mut page = PageV39 {
            ast: None,
            config,
            errors: vec![],
            fs_modified: None,
            output_content: None,
            source_content: Some(source_content),
            source_path: None,
            status: None,
            template_list: vec![],
            r#type: None,
            payload: None,
        };
        let _ = page.generate_ast();
        let _ = page.generate_payload();
        Ok(page)
    }
}

impl PageV39 {
    // DEPRECATED PROBABLY IN FAVOR OF KEYS
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
        //// update details in sections
        //self.ast.unwrap().iter().for_each(|section| {
        //    match &section.kind {
        //        SectionV39Kind::Basic { children } => {},
        //        SectionV39Kind::Block { spans } => {},
        //        SectionV39Kind::Raw { children , text } = {},
        //        SectionV39Kind::Yaml {  } => {}
        //    }
        //    //section.
        //});
        // // Prep all the necessary fields
        // self.prep_type();
        // self.prep_status();
        // self.prep_template_list();
        Ok(())
    }

    pub fn generate_payload(&mut self) -> Result<()> {
        let mut p = PagePayload::new();
        p.id = self.get_metadata_attr("id");
        p.status = self.get_status();
        self.payload = Some(p);
        Ok(())
    }

    // DEPRECATED PROBABLY IN FAVOR OF KEYS
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

    pub fn get_status(&self) -> Option<String> {
        if let Some(status) = self.get_metadata_attr("status") {
            Some(status)
        } else {
            Some("published".to_string())
        }
    }

    // DEPRECATED PROBABLY IN FAVOR OF KEYS
    pub fn id(&self) -> Option<String> {
        self.get_metadata_attr("id")
    }

    // DEPRECATED in favor of get_status
    pub fn prep_status(&mut self) {
        if let Some(status) = self.get_metadata_attr("status") {
            self.r#status = Some(status);
        } else {
            self.status = Some("published".to_string());
        }
    }

    pub fn prep_template_list(&mut self) {
        if let (Some(_t), Some(_status)) = (&self.r#type, &self.status) {
            self.template_list
                .push(format!("pages/post/published.neoj"));

            // TODO: Add these when you make sure that they
            // don't already match
            // self.template_list = vec![
            //     format!("pages/{}/{}.neoj", t, status),
            //     format!("pages/{}/published.neoj", t),
            //     format!("pages/post/{}.neoj", status),
            //     format!("pages/post/published.neoj"),
            // ];
        }
    }

    pub fn prep_type(&mut self) {
        if let Some(t) = self.get_metadata_attr("type") {
            self.r#type = Some(t);
        } else {
            self.r#type = Some("post".to_string());
        }
    }

    // DEPRECATED PROBABLY IN FAVOR OF KEYS
    pub fn rel_output_path(&self) -> Option<PathBuf> {
        // TODO: Reminder: Put output path check in the builder
        // to prevent files from being written outside the
        // output root.
        if let Some(override_path) = self.get_metadata_attr("path") {
            self.rel_output_path_scrubber(&override_path)
        } else {
            if let (Ok(lang), Some(id)) = (self.config.default_language(), self.id()) {
                Some(PathBuf::from(format!("{}/{}/index.html", lang, id)))
            } else {
                None
            }
        }
    }

    pub fn rel_output_path_scrubber(&self, source: &str) -> Option<PathBuf> {
        if let Some(scrubbed_string) = source.strip_prefix("/") {
            if scrubbed_string == "" {
                Some(PathBuf::from("index.html"))
            } else {
                let p = PathBuf::from(scrubbed_string);
                if let Some(_) = p.extension() {
                    Some(p)
                } else {
                    Some(p.join("index.html"))
                }
            }
        } else {
            None
        }
    }

    // DEPRECATED PROBABLY IN FAVOR OF KEYS
    pub fn rel_source_path(&self) -> Option<PathBuf> {
        let source_path = &self.source_path.clone().unwrap();
        if let Ok(rel_source_path) = source_path.strip_prefix(self.config.content_dir()) {
            Some(rel_source_path.to_path_buf())
        } else {
            None
        }
    }

    // DEPRECATED PROBABLY IN FAVOR OF KEYS
    pub fn r#type(&self) -> Option<String> {
        if let Some(t) = self.get_metadata_attr("type") {
            Some(t)
        } else {
            Some("post".to_string())
        }
    }

    // DEPRECATED PROBABLY IN FAVOR OF KEYS
    pub fn status(&self) -> Option<String> {
        if let Some(t) = self.get_metadata_attr("status") {
            Some(t)
        } else {
            Some("published".to_string())
        }
    }

    //
}
