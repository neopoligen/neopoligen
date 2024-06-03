pub mod mocks;
pub mod object;

use crate::helpers::clean_for_url;
use crate::image::Image;
use crate::page_filters::*;
use crate::{page_v2::PageV2, site_config::SiteConfig};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use minijinja::value::Value;
use minijinja::Error;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct SiteV2 {
    pub build_time: DateTime<Utc>,
    pub config: SiteConfig,
    pub images: BTreeMap<String, SiteImage>,
    pub pages: BTreeMap<String, PageV2>,
    pub mp3s: BTreeMap<String, SiteMp3>,
}

#[derive(Debug, Serialize)]
pub struct SiteImage {
    alt_text: Option<String>,
    alt_text_extended: Option<String>,
    width: u32,
    height: u32,
    key: String,
    extension: String,
    versions: Vec<(u32, u32)>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SiteMp3 {
    pub key: String,
    pub extension: String,
    // TODO: Add useful metadata here
}

impl SiteV2 {
    pub fn new(
        config: &SiteConfig,
        source_pages: &BTreeMap<PathBuf, PageV2>,
        source_images: &BTreeMap<PathBuf, Image>,
        source_mp3s: &BTreeMap<String, SiteMp3>,
    ) -> SiteV2 {
        let mut images = BTreeMap::new();
        for (_source_path, image) in source_images.iter() {
            // This check makes sure thee image has data
            if let Some(_width) = image.width {
                images.insert(
                    image.key().expect("key"),
                    SiteImage {
                        alt_text: image.alt_text.clone(),
                        alt_text_extended: image.alt_text_extended.clone(),
                        extension: image.extension().expect("extension"),
                        height: image.height.expect("height"),
                        key: image.key().expect("key"),
                        versions: image.versions.clone(),
                        width: image.width.expect("width"),
                    },
                );
            }
        }
        let mut pages: BTreeMap<String, PageV2> = BTreeMap::new();
        source_pages.iter().for_each(|p| {
            if let Some(id) = p.1.id() {
                pages.insert(id, p.1.clone());
            }
        });
        SiteV2 {
            build_time: Utc::now(),
            config: config.clone(),
            images,
            mp3s: source_mp3s.clone(),
            pages,
        }
    }
}
impl SiteV2 {
    pub fn base_url(&self) -> Result<Value, Error> {
        Ok(Value::from(self.config.base_url()))
    }

    pub fn build_time(&self) -> Result<Value, Error> {
        Ok(Value::from(
            self.build_time
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        ))
    }

    pub fn collection_by_date(&self, args: &[Value]) -> Result<Value, Error> {
        if let Some(or_filters) = PageFilterOrSet::parse(&args) {
            Ok(Value::from_serialize(
                self.pages
                    .iter()
                    .filter(|p| p.1.date().is_ok())
                    .filter(|p| p.1.passes(&or_filters))
                    .sorted_by(|a, b| Ord::cmp(&b.1.date().unwrap(), &a.1.date().unwrap()))
                    .map(|p| p.1.id().unwrap())
                    .collect::<Vec<String>>(),
            ))
        } else {
            Err(std::fmt::Error.into())
        }

        // DEPRECATED: Remove when collecction_by_date is working
        // let mut or_filters = PageFilterOrSet::new();
        // or_filters.and_groups = args
        //     .iter()
        //     .filter_map(|ag| {
        //         if let Ok(and_iter) = ag.try_iter() {
        //             Some(PageFilterAndGroup {
        //                 filters: and_iter
        //                     .into_iter()
        //                     .filter_map(|filter_string| {
        //                         if let Some(text) = filter_string.as_str() {
        //                             PageFilter::parse(text)
        //                         } else {
        //                             None
        //                         }
        //                     })
        //                     .collect::<Vec<PageFilter>>(),
        //             })
        //         } else {
        //             None
        //         }
        //     })
        //     .collect();
        //dbg!(or_filters);
        // Ok(Value::from_serialize(
        //     self.pages
        //         .iter()
        //         .filter(|p| {
        //             let mut include = false;
        //             if let (Some(id), Some(status)) = (p.1.id(), p.1.status()) {
        //                 include = true
        //             }
        //             include
        //         })
        //         .sorted_by(|a, b| Ord::cmp(&b.1.date(), &a.1.date()))
        //         .map(|p| p.1.id().unwrap())
        //         .collect::<Vec<String>>(),
        // ))
    }

    pub fn config(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.config))
    }

    pub fn get_image(&self, args: &[Value]) -> Result<Value, Error> {
        let key = args[0].to_string();
        let key = clean_for_url(&key).unwrap();
        if let Some(image) = self.images.get(&key) {
            Ok(Value::from_serialize(image))
        } else {
            // TODO: Figure out how to pass back an error here
            Ok(Value::from(""))
        }
    }

    pub fn get_mp3(&self, args: &[Value]) -> Result<Value, Error> {
        let key = args[0].to_string();
        let key = clean_for_url(&key).unwrap();
        if let Some(mp3) = self.mp3s.get(&key) {
            Ok(Value::from_serialize(mp3))
        } else {
            // TODO: Figure out how to pass back an error here
            Ok(Value::from(""))
        }
    }

    pub fn page_date_for_feed(&self, args: &[Value]) -> Result<Value, Error> {
        let page_id = args[0].to_string();
        if let Some(page) = &self.pages.get(&page_id) {
            if let Ok(dt) = page.date() {
                Ok(Value::from(
                    dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                ))
            } else {
                Err(std::fmt::Error.into())
            }
        } else {
            Err(std::fmt::Error.into())
        }
    }

    pub fn page_format_date(&self, args: &[Value]) -> Result<Value, Error> {
        let page_id = args[0].to_string();
        let fmt = args[1].to_string();
        if let Some(page) = &self.pages.get(&page_id) {
            Ok(page.format_date(&fmt).into())
        } else {
            Err(std::fmt::Error.into())
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

    // DEPRECATED: TODO: replace .page_sections() with page.sections()
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

    pub fn page_uuid(&self, args: &[Value]) -> Result<Value, Error> {
        if let Some(page_id) = args[0].as_str() {
            if let Some(page) = &self.pages.get(page_id) {
                if let Some(uuid) = page.uuid() {
                    Ok(Value::from(uuid))
                } else {
                    Err(std::fmt::Error.into())
                }
            } else {
                Err(std::fmt::Error.into())
            }
        } else {
            Err(std::fmt::Error.into())
        }
    }

    // // TODO: Change to collection_*
    // pub fn get_pages_by_date(&self, _args: &[Value]) -> Result<Value, Error> {
    //     let pages = self
    //         .pages
    //         .iter()
    //         .filter_map(|p| {
    //             if let (Some(id), Some(date)) = (p.1.id(), p.1.date()) {
    //                 Some((id, date))
    //             } else {
    //                 None
    //             }
    //         })
    //         .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
    //         .map(|i| i.0)
    //         .collect::<Vec<String>>();
    //     Ok(Value::from_serialize(pages))
    // }

    pub fn theme(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.config.theme))
    }

    pub fn uuid(&self) -> Result<Value, Error> {
        Ok(Value::from(
            Uuid::new_v5(&Uuid::NAMESPACE_DNS, self.config.base_url().as_bytes()).to_string(),
        ))
    }
}
