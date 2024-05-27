pub mod mocks;

use crate::ast::ast;
use crate::section::Section;
use crate::site_config::SiteConfig;
use crate::span::Span;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PageV2 {
    pub ast: Vec<Section>,
    //pub cached_hash: Option<String>,
    pub config: SiteConfig,
    pub source_path: Option<PathBuf>,
    pub source_content: Option<String>,
    pub output: Option<String>,
}

impl PageV2 {
    pub fn new_from_filesystem(
        source_path: PathBuf,
        config: SiteConfig,
        source_content: String,
    ) -> PageV2 {
        PageV2 {
            ast: vec![],
            //cached_hash: None,
            config,
            output: None,
            source_path: Some(source_path),
            source_content: Some(source_content),
        }
    }
}

impl PageV2 {
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

    pub fn og_image(&self) -> Option<String> {
        if let Some(id) = &self.id() {
            Some(format!("{}/og-images/{}.jpg", self.config.base_url(), id))
        } else {
            None
        }
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

    //
}
