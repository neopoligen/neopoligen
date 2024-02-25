use crate::ast::*;
use crate::config::Config;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn new(source_path: PathBuf, source: String, config: &Config) -> Option<Page> {
        if let Ok((_, ast)) = ast(source.trim_start(), config) {
            Some(Page {
                ast,
                source,
                source_path,
            })
        } else {
            None
        }
    }
}
