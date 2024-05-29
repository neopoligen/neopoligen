pub mod mocks;
pub mod object;

use crate::helpers::clean_for_url;
use crate::image::Image;
use crate::{page_v2::PageV2, site_config::SiteConfig};
use itertools::Itertools;
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
            config: config.clone(),
            images,
            mp3s: source_mp3s.clone(),
            pages,
        }
    }
}

#[derive(Debug)]
pub struct PageFilterOrSet {
    and_groups: Vec<PageFilterAndGroup>,
}

impl PageFilterOrSet {
    pub fn new() -> PageFilterOrSet {
        PageFilterOrSet { and_groups: vec![] }
    }
}

#[derive(Debug)]
pub struct PageFilterAndGroup {
    pub filters: Vec<PageFilter>,
}

#[derive(Debug)]
pub struct PageFilter {
    exclude: bool,
    r#type: PageFilterType,
    value: String,
}

#[derive(Debug)]
pub enum PageFilterType {
    RootFolder,
    Folder,
    Status,
    Tag,
}

impl SiteV2 {
    pub fn base_url(&self) -> Result<Value, Error> {
        Ok(Value::from(self.config.base_url()))
    }

    pub fn collection_by_date(&self, args: &[Value]) -> Result<Value, Error> {
        let mut or_filters = PageFilterOrSet::new();
        if let Ok(raw_or_groups) = args[0].try_iter() {
            or_filters.and_groups = raw_or_groups
                .into_iter()
                .filter_map(|ag| {
                    if let Ok(and_iter) = ag.try_iter() {
                        Some(PageFilterAndGroup {
                            filters: and_iter
                                .into_iter()
                                .filter_map(|filter_string| {
                                    let fs = filter_string.to_string();
                                    if let Some(parts) = fs.split_once(":") {
                                        dbg!(&parts.0);
                                        match parts.0 {
                                            "status" => Some(PageFilter {
                                                exclude: false,
                                                r#type: PageFilterType::Status,
                                                value: parts.1.to_string(),
                                            }),
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<PageFilter>>(),
                        })
                    } else {
                        None
                    }
                })
                .collect();
        }

        dbg!(or_filters);

        // let mut or_filters = PageFilterOrSet::new();
        // or_filters.and_groups = filters_arg
        //     .into_iter()
        //     .filter_map(|_f| {
        //         None
        //         // dbg!(&f);
        //         // PageFilter {
        //         //     r#type: PageFilterType::Status,
        //         //     value: "some_value".to_string(),
        //         //     exclude: false,
        //         // }
        //     })
        //     .collect::<Vec<PageFilterAndGroup>>();
        let v = vec![
            "delta123".to_string(),
            "alfa1234".to_string(),
            "hotel123".to_string(),
            "foxtrot1".to_string(),
            "golf1234".to_string(),
            "echo1234".to_string(),
            "bravo123".to_string(),
        ];
        Ok(Value::from_serialize(v))
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
}
