pub mod object;

use crate::{page_v2::PageV2, site_config::SiteConfig};
use minijinja::value::Value;
use minijinja::Error;
// use rusqlite::config;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct SiteV2 {
    pub config: SiteConfig,
    pub pages: BTreeMap<String, PageV2>,
}

impl SiteV2 {
    pub fn new(config: &SiteConfig, source_pages: &BTreeMap<PathBuf, PageV2>) -> SiteV2 {
        let mut pages: BTreeMap<String, PageV2> = BTreeMap::new();
        source_pages.iter().for_each(|p| {
            if let Some(id) = p.1.id() {
                pages.insert(id, p.1.clone());
            }
        });
        SiteV2 {
            config: config.clone(),
            pages,
        }
    }
}

impl SiteV2 {
    pub fn base_url(&self) -> Result<Value, Error> {
        if let Some(url) = &self.config.base_url {
            Ok(Value::from(url))
        } else {
            // TODO: Make this an Error
            Ok(Value::from(""))
        }
    }

    pub fn config(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.config))
    }

    pub fn page_href(&self, args: &[Value]) -> Result<Value, Error> {
        match &self.pages.get(&args[0].to_string()) {
            Some(page) => {
                if let Some(href) = page.href() {
                    Ok(Value::from(href))
                } else {
                    Ok(Value::from(""))
                }
            }
            None => Ok(Value::from("")),
        }
    }

    pub fn page_permalink(&self, args: &[Value]) -> Result<Value, Error> {
        match &self.pages.get(&args[0].to_string()) {
            Some(page) => {
                if let Some(permalink) = page.permalink() {
                    Ok(Value::from(permalink))
                } else {
                    Ok(Value::from(""))
                }
            }
            None => Ok(Value::from("")),
        }
    }

    pub fn page_sections(&self, args: &[Value]) -> Result<Value, Error> {
        match &self.pages.get(&args[0].to_string()) {
            Some(page) => Ok(Value::from_serialize(&page.ast)),
            None => Ok(Value::from("")),
        }
    }

    pub fn page_title_as_plain_text(&self, args: &[Value]) -> Result<Value, Error> {
        match &self.pages.get(&args[0].to_string()) {
            Some(page) => Ok(Value::from_serialize(&page.title_as_plain_text())),
            None => Ok(Value::from("")),
        }
    }

    pub fn theme(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.config.theme_options))
    }
}
