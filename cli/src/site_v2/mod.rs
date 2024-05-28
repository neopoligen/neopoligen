pub mod object;

use crate::image::Image;
use crate::{page_v2::PageV2, site_config::SiteConfig};
use minijinja::value::Value;
use minijinja::Error;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct SiteV2 {
    pub config: SiteConfig,
    pub images: BTreeMap<String, SiteImage>,
    pub pages: BTreeMap<String, PageV2>,
}

#[derive(Debug, Serialize)]
pub struct SiteImage {
    width: u32,
    height: u32,
    key: String,
    extension: String,
    versions: Vec<(u32, u32)>,
}

impl SiteV2 {
    pub fn new(
        config: &SiteConfig,
        source_pages: &BTreeMap<PathBuf, PageV2>,
        source_images: &BTreeMap<PathBuf, Image>,
    ) -> SiteV2 {
        let mut images = BTreeMap::new();
        for (_source_path, image) in source_images.iter() {
            images.insert(
                image.key().expect("key"),
                SiteImage {
                    width: image.width.expect("width"),
                    height: image.height.expect("height"),
                    extension: image.extension().expect("extension"),
                    key: image.key().expect("key"),
                    versions: image.versions.clone(),
                },
            );
        }

        let mut pages: BTreeMap<String, PageV2> = BTreeMap::new();
        source_pages.iter().for_each(|p| {
            if let Some(id) = p.1.id() {
                pages.insert(id, p.1.clone());
            }
        });
        SiteV2 {
            config: config.clone(),
            images,
            pages,
        }
    }
}

impl SiteV2 {
    pub fn base_url(&self) -> Result<Value, Error> {
        Ok(Value::from(self.config.base_url()))
    }

    pub fn config(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.config))
    }

    pub fn get_image(&self, args: &[Value]) -> Result<Value, Error> {
        let key = args[0].to_string();
        if let Some(image) = self.images.get(&key) {
            Ok(Value::from_serialize(image))
        } else {
            // TODO: Figure out how to pass back an error here
            Ok(Value::from(""))
        }
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

    pub fn page_og_image(&self, args: &[Value]) -> Result<Value, Error> {
        match &self.pages.get(&args[0].to_string()) {
            Some(page) => {
                if let Some(og_image) = page.og_image() {
                    Ok(Value::from(og_image))
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
        Ok(Value::from_serialize(&self.config.theme))
    }
}
