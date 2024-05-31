use crate::ast_v39::ast;
use crate::section_v39::SectionV39;
use crate::site_config::SiteConfig;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct PageV39 {
    pub ast: Option<Vec<SectionV39>>,
    pub config: SiteConfig,
    pub fs_modified: Option<SystemTime>,
    pub output_content: Option<String>,
    pub source_content: Option<String>,
}

impl PageV39 {
    pub fn new_from_fs(
        source_path: PathBuf,
        config: SiteConfig,
        source_content: String,
    ) -> Result<PageV39> {
        let fs_modified = fs::metadata(source_path)?.modified()?;
        Ok(PageV39 {
            ast: None,
            config,
            fs_modified: Some(fs_modified),
            output_content: None,
            source_content: Some(source_content),
        })
    }
}

impl PageV39 {
    pub fn generate_ast(&'_ mut self) -> Result<()> {
        if let Ok(a) = ast(
            &self.source_content.as_ref().unwrap(),
            &self.config.sections,
            &self.config.spans,
        ) {
            self.ast = Some(a);
        }
        Ok(())
    }
}
