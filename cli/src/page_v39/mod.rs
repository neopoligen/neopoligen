use crate::section_v39::SectionV39;
use crate::site_config::SiteConfig;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct PageV39 {
    pub ast: Option<Vec<SectionV39>>,
    pub fs_modified: Option<SystemTime>,
}

impl PageV39 {
    pub fn new_from_fs(
        source_path: PathBuf,
        _config: SiteConfig,
        _content: String,
    ) -> Result<PageV39> {
        let fs_modified = fs::metadata(source_path)?.modified()?;
        Ok(PageV39 {
            ast: None,
            fs_modified: Some(fs_modified),
        })
    }
}

impl PageV39 {
    pub fn generate_ast(&self) -> Result<()> {
        Ok(())
    }
}
