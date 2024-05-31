pub mod mocks;
pub mod object;

use crate::ast::ast;
use crate::page_filters::*;
use crate::section::Section;
use crate::site_config::SiteConfig;
use crate::span::Span;
use anyhow::Result;
use chrono::DateTime;
use chrono::FixedOffset;
use minijinja::value::Value;
use minijinja::Error;
use minijinja::ErrorKind;
use nom::combinator::value;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PageV2 {
    pub ast: Vec<Section>,
    //pub cached_hash: Option<String>,
    pub config: SiteConfig,
    pub output: Option<String>,
    pub source_content: Option<String>,
    pub source_path: Option<PathBuf>,
}

impl PageV2 {
    pub fn new_from_filesystem(
        source_path: PathBuf,
        config: SiteConfig,
        source_content: String,
    ) -> PageV2 {
        PageV2 {
            ast: vec![],
            config,
            output: None,
            source_content: Some(source_content),
            source_path: Some(source_path),
        }
    }
}

impl PageV2 {
    pub fn all_sections(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.ast))
    }

    pub fn all_sections_except(&self, args: &[Value]) -> Result<Value, Error> {
        let skips: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
        Ok(Value::from_serialize(
            &self
                .ast
                .iter()
                .filter(|section| match section {
                    Section::Basic { r#type, .. } => !skips.contains(r#type),
                    Section::Block { r#type, .. } => !skips.contains(r#type),
                    Section::Checklist { r#type, .. } => !skips.contains(r#type),
                    Section::ChecklistItem { r#type, .. } => !skips.contains(r#type),
                    Section::Comment { r#type, .. } => !skips.contains(r#type),
                    Section::Generic { r#type, .. } => !skips.contains(r#type),
                    Section::Json { r#type, .. } => !skips.contains(r#type),
                    Section::List { r#type, .. } => !skips.contains(r#type),
                    Section::ListItem { r#type, .. } => !skips.contains(r#type),
                    Section::Raw { r#type, .. } => !skips.contains(r#type),
                    Section::TagFinderInit => false,
                    Section::Yaml { r#type, .. } => !skips.contains(r#type),
                })
                .collect::<Vec<_>>(),
        ))
    }

    pub fn date(&self) -> Result<DateTime<FixedOffset>> {
        if let Some(dt) = self.get_metadata_attr("updated") {
            Ok(DateTime::parse_from_rfc3339(&dt)?)
        } else if let Some(dt) = self.get_metadata_attr("created") {
            Ok(DateTime::parse_from_rfc3339(&dt)?)
        } else {
            Err(std::fmt::Error.into())
        }
    }

    pub fn feed_date(&self) -> Option<String> {
        if let Some(dt) = self.get_metadata_attr("updated") {
            if let Some(parsed) = DateTime::parse_from_rfc3339(&dt).ok() {
                Some(parsed.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
            } else {
                None
            }
        } else if let Some(dt) = self.get_metadata_attr("created") {
            if let Some(parsed) = DateTime::parse_from_rfc3339(&dt).ok() {
                Some(parsed.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn format_created_date(&self, fmt: &str) -> Option<String> {
        if let Some(dt) = self.get_metadata_attr("created") {
            if let Some(parsed) = DateTime::parse_from_rfc3339(&dt).ok() {
                Some(parsed.format(&fmt).to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn format_date(&self, fmt: &str) -> Option<String> {
        if let Some(dt) = self.get_metadata_attr("updated") {
            if let Some(parsed) = DateTime::parse_from_rfc3339(&dt).ok() {
                Some(parsed.format(&fmt).to_string())
            } else {
                None
            }
        } else if let Some(dt) = self.get_metadata_attr("created") {
            if let Some(parsed) = DateTime::parse_from_rfc3339(&dt).ok() {
                Some(parsed.format(&fmt).to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn format_updated_date(&self, fmt: &str) -> Option<String> {
        if let Some(dt) = self.get_metadata_attr("updated") {
            if let Some(parsed) = DateTime::parse_from_rfc3339(&dt).ok() {
                Some(parsed.format(&fmt).to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn generate_ast(&mut self, config: &SiteConfig) {
        match ast(
            &self.source_content.clone().unwrap(),
            &config.sections.clone(),
            &config.spans,
        ) {
            Ok(ast) => self.ast = ast,
            Err(_) => {}
        }
    }

    pub fn get_metadata_attr(&self, key: &str) -> Option<String> {
        self.ast.iter().find_map(|sec_enum| {
            if let Section::Yaml { r#type, attrs, .. } = sec_enum {
                if r#type == "metadata" {
                    attrs.iter().find_map(|attr| {
                        if attr.0 == key {
                            Some(attr.1.trim().to_string())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn hash(&self) -> Option<String> {
        if let Some(content) = &self.source_content {
            Some(sha256::digest(content))
        } else {
            None
        }
    }

    pub fn href(&self) -> Option<String> {
        if let (Some(rel_path), Some(title)) = (self.rel_file_path(), self.title_for_url()) {
            let p = PathBuf::from("/").join(rel_path.parent().unwrap());
            Some(format!("{}/?{}", p.display(), title))
        } else {
            None
        }
    }

    pub fn id_v2(&self) -> Result<Value, Error> {
        if let Some(id) = self.get_metadata_attr("id") {
            Ok(Value::from(id))
        } else {
            Err(Error::new(ErrorKind::CannotUnpack, "could not get page id"))
        }
    }

    // DEPRECATED: TODO: switch to .id_v2()
    pub fn id(&self) -> Option<String> {
        self.ast.iter().find_map(|sec_enum| {
            if let Section::Yaml { r#type, attrs, .. } = sec_enum {
                if r#type == "metadata" {
                    attrs.iter().find_map(|attr| {
                        if attr.0 == "id" {
                            Some(attr.1.trim().to_string())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn passes(&self, or_set: &PageFilterOrSet) -> bool {
        if or_set
            .and_groups
            .iter()
            .filter(|ag| {
                let mut hit_include = 0;
                let mut hit_exclude = 0;
                for item in ag.filters.iter() {
                    match &item.kind {
                        PageFilterV2Kind::Status { value_to_match } => {
                            if let Ok(target) = self.status_v2() {
                                if value_to_match == target.as_str().unwrap() {
                                    if item.exclude_request == true {
                                        hit_exclude += 1;
                                    } else {
                                        hit_include += 1;
                                    }
                                }
                            }
                        }
                        PageFilterV2Kind::Type { value_to_match } => {}
                    }
                }
                if hit_exclude > 0 {
                    false
                } else if hit_include == 0 {
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<_>>()
            .len()
            > 0
        {
            true
        } else {
            false
        }

        //if let Some(_) = or_set.and_groups.iter().find(|ag| {
        //let mut found_include = false;
        //let mut found_exclude = false;

        // for filter in ag.filters.iter() {
        //     match filter {
        //         PageFilter::Status { exclude, value } => {
        //             if *exclude == true {
        //                 if *value == self.status().unwrap() {
        //                     found_exclude = true;
        //                 } else {
        //                     found_include = true;
        //                 }
        //             } else {
        //                 if *value == self.status().unwrap() {
        //                     found_include = true;
        //                 }
        //             }
        //         }
        //         PageFilter::Type { exclude, value } => {
        //             if let Ok(t) = self.type_v2() {
        //                 let t = t.to_string();
        //                 if *exclude == true {
        //                     if *value == t {
        //                         found_exclude = true;
        //                     } else {
        //                         found_include = true;
        //                     }
        //                 } else {
        //                     if *value == t {
        //                         found_include = true;
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        // if found_include && !found_exclude {
        //     true
        // } else {
        //     false
        // }

        //false

        ////
        //}) {
        //true
        //} else {
        //false
        //}

        //
    }

    pub fn og_image(&self) -> Option<String> {
        if let Some(id) = &self.id() {
            Some(format!("{}/og-images/{}.jpg", self.config.base_url(), id))
        } else {
            None
        }
    }

    pub fn only_sections(&self, args: &[Value]) -> Result<Value, Error> {
        let outputs: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
        Ok(Value::from_serialize(
            &self
                .ast
                .iter()
                .filter(|section| match section {
                    Section::Basic { r#type, .. } => outputs.contains(r#type),
                    Section::Block { r#type, .. } => outputs.contains(r#type),
                    Section::Checklist { r#type, .. } => outputs.contains(r#type),
                    Section::ChecklistItem { r#type, .. } => outputs.contains(r#type),
                    Section::Comment { r#type, .. } => outputs.contains(r#type),
                    Section::Generic { r#type, .. } => outputs.contains(r#type),
                    Section::Json { r#type, .. } => outputs.contains(r#type),
                    Section::List { r#type, .. } => outputs.contains(r#type),
                    Section::ListItem { r#type, .. } => outputs.contains(r#type),
                    Section::Raw { r#type, .. } => outputs.contains(r#type),
                    Section::TagFinderInit => false,
                    Section::Yaml { r#type, .. } => outputs.contains(r#type),
                })
                .collect::<Vec<_>>(),
        ))
    }

    pub fn permalink(&self) -> Option<String> {
        if let Some(href) = &self.href() {
            Some(format!("{}{}", self.config.base_url(), href))
        } else {
            None
        }
    }

    pub fn plain_text_from_spans(&self, spans: &Vec<Span>) -> Option<String> {
        let strings = spans
            .iter()
            .filter_map(|s| match s {
                Span::WordPart { text, .. } => Some(text.to_string()),
                Span::Space { .. } => Some(" ".to_string()),
                Span::KnownSpan { spans, .. } => self.plain_text_from_spans(&spans),
                _ => None,
            })
            .collect::<Vec<String>>();
        if strings.len() > 0 {
            Some(strings.join(""))
        } else {
            None
        }
    }

    pub fn rel_file_path(&self) -> Option<PathBuf> {
        match self.ast.iter().find_map(|sec_enum| {
            if let Section::Yaml { r#type, attrs, .. } = sec_enum {
                if r#type == "metadata" {
                    attrs.iter().find_map(|attr| {
                        if attr.0 == "path" {
                            let path = PathBuf::from(attr.1.trim())
                                .join("index.html")
                                .strip_prefix("/")
                                .unwrap()
                                .to_path_buf();
                            Some(path)
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }) {
            Some(path) => Some(path),
            None => {
                if let Some(id) = self.id() {
                    Some(
                        PathBuf::from(self.config.default_language.clone())
                            .join(id)
                            .join("index.html"),
                    )
                } else {
                    None
                }
            }
        }
    }

    pub fn status(&self) -> Option<String> {
        if let Some(t) = self.get_metadata_attr("status") {
            Some(t)
        } else {
            Some("published".to_string())
        }
    }

    pub fn status_v2(&self) -> Result<Value, Error> {
        if let Some(t) = self.get_metadata_attr("status") {
            Ok(Value::from(t))
        } else {
            Ok(Value::from("published"))
        }
    }

    pub fn title_as_plain_text(&self) -> Option<String> {
        if let Some(title) = self.title_from_metadata() {
            Some(title)
        } else if let Some(title) = self.title_from_title_section() {
            Some(title)
        } else if let Some(title) = self.title_from_any_section() {
            Some(title)
        } else if let Some(title) = self.title_from_first_few_words() {
            Some(title)
        } else {
            self.id()
        }
    }

    pub fn title_for_url(&self) -> Option<String> {
        if let Some(original) = self.title_as_plain_text() {
            let re1 = Regex::new(r"\W").unwrap();
            let re2 = Regex::new(r"-+").unwrap();
            let re3 = Regex::new(r"^-").unwrap();
            let re4 = Regex::new(r"-$").unwrap();
            let mut updated = original.to_lowercase();
            updated = re1.replace_all(&updated, "-").to_string();
            updated = re2.replace_all(&updated, "-").to_string();
            updated = re3.replace_all(&updated, "").to_string();
            updated = re4.replace_all(&updated, "").to_string();
            Some(updated.to_string())
        } else {
            None
        }
    }

    pub fn title_from_any_section(&self) -> Option<String> {
        self.ast.iter().find_map(|child| {
            match child {
                Section::Basic { attrs, .. } => attrs.get("title"),
                _ => None,
            }
            .cloned()
        })
    }

    pub fn title_from_first_few_words(&self) -> Option<String> {
        self.ast.iter().find_map(|sec_enum| match sec_enum {
            Section::Basic { children, .. } => {
                if children.len() > 0 {
                    if let Section::Block { spans, .. } = &children[0] {
                        if let Some(full_block) = self.plain_text_from_spans(&spans) {
                            let words = full_block.split(" ").take(9).collect::<Vec<&str>>();
                            Some(words.join(" "))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    fn title_from_metadata(&self) -> Option<String> {
        self.ast.iter().find_map(|sec_enum| {
            if let Section::Yaml { r#type, attrs, .. } = sec_enum {
                if r#type == "metadata" {
                    attrs.iter().find_map(|attr| {
                        if attr.0 == "title" {
                            Some(attr.1.trim().to_string())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn title_from_title_section(&self) -> Option<String> {
        self.ast.iter().find_map(|sec_enum| match sec_enum {
            Section::Basic {
                r#type, children, ..
            } => {
                if *r#type == String::from("title") {
                    if children.len() > 0 {
                        if let Section::Block { spans, .. } = &children[0] {
                            self.plain_text_from_spans(&spans)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    pub fn type_v2(&self) -> Result<Value, Error> {
        if let Some(t) = self.get_metadata_attr("type") {
            Ok(Value::from(t))
        } else {
            Ok(Value::from("post".to_string()))
        }
    }

    pub fn r#type(&self) -> Option<String> {
        if let Some(t) = self.get_metadata_attr("type") {
            Some(t)
        } else {
            Some("post".to_string())
        }
    }

    pub fn uuid(&self) -> Option<String> {
        if let Some(id) = self.id() {
            Some(Uuid::new_v5(&Uuid::NAMESPACE_DNS, id.as_bytes()).to_string())
        } else {
            None
        }
    }

    //
}

// fn get_datetime(source: &str) -> Result<NaiveDateTime, chrono::format::ParseError> {
//     let date = NaiveDate::parse_and_remainder(source, "%Y-%m-%d")?;
//     if let Ok(time) = NaiveTime::parse_from_str(date.1, " %H:%M:%S") {
//         Ok(date.0.and_time(time))
//     } else {
//         Ok(date.0.and_hms_opt(0, 0, 0).unwrap())
//     }
// }
