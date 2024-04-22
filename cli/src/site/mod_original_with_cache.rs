pub mod new;
pub mod object;

use crate::cache_object::CacheObject;
use crate::child::Child;
use crate::collection::{Collection, CollectionItem};
use crate::config::Config;
use crate::image::Image;
use crate::mp3::Mp3;
use crate::page::Page;
use minijinja::Value;
use serde::Serialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use tracing::{event, instrument, Level};

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub cache: Mutex<BTreeMap<String, CacheObject>>,
    pub config: Config,
    pub pages: BTreeMap<String, Page>,
    pub invalid_pages: BTreeMap<PathBuf, String>,
    pub templates: BTreeMap<String, String>,
    pub images: Vec<Image>,
    pub mp3s: Vec<Mp3>,
}

impl Site {
    #[instrument(skip(self))]
    pub fn collection_from_files_and_folders(&self, args: &[Value]) -> Collection {
        let now = Instant::now();
        let id = args[0].to_string();
        match args[1].try_iter() {
            Ok(value_patterns) => {
                let patterns = value_patterns
                    .filter_map(|value_pattern| match value_pattern.try_iter() {
                        Ok(parts) => Some({
                            let response = parts
                                .filter_map(|p| Some(p.to_string()))
                                .collect::<Vec<String>>();
                            response
                        }),
                        Err(_e) => None,
                    })
                    .collect::<Vec<_>>();
                let mut c = Collection::new_from_files_and_folders(&self.pages, patterns);
                c.set_active_item(&id);
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                c
            }
            Err(e) => {
                println!("{}", e);
                Collection::empty()
            }
        }
    }

    #[instrument(skip(self))]
    pub fn collection_from_tags(&self, args: &[Value]) -> Collection {
        let now = Instant::now();
        let id = args[0].to_string();
        match args[1].try_iter() {
            Ok(tags) => {
                let tag_set = tags.map(|t| t.to_string()).collect::<Vec<String>>();
                let tag_key = tag_set.join("");
                // event!(Level::DEBUG, "tag_key: {}", tag_key);
                match self.get_cache(&tag_key) {
                    Some(c_obj) => match c_obj {
                        CacheObject::Collection(mut c) => {
                            // event!(Level::DEBUG, "cache_hit: {}", tag_key);
                            c.set_active_item(&id);
                            event!(Level::DEBUG, "||{:?}||", now.elapsed());
                            c
                        }
                        _ => {
                            // event!(Level::DEBUG, "cache_miss: {}", tag_key);
                            let mut c = Collection::new_from_tags(&self.pages, tag_set);
                            c.set_active_item(&id);
                            self.set_cache(tag_key, CacheObject::Collection(c.clone()));
                            event!(Level::DEBUG, "||{:?}||", now.elapsed());
                            c
                        }
                    },
                    None => {
                        // event!(Level::DEBUG, "cache_miss: {}", tag_key);
                        let mut c = Collection::new_from_tags(&self.pages, tag_set);
                        c.set_active_item(&id);
                        self.set_cache(tag_key, CacheObject::Collection(c.clone()));
                        event!(Level::DEBUG, "||{:?}||", now.elapsed());
                        c
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
                Collection::empty()
            }
        }
    }

    #[instrument(skip(self))]
    pub fn does_template_exist(&self, args: &[Value]) -> String {
        let now = Instant::now();
        let path = args[0].to_string();
        let response = match self.templates.get(&path) {
            Some(_) => "yes".to_string(),
            None => "no".to_string(),
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response
    }

    #[instrument(skip(self))]
    pub fn error_from_template(&self, args: &[Value]) -> String {
        let now = Instant::now();
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        "".to_string()
    }

    #[instrument(skip(self))]
    pub fn highlight_code(&self, args: &[Value]) -> String {
        let now = Instant::now();
        let code = args[0].to_string();
        let lang = args[1].to_string();
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let syntax = syntax_set
            .find_syntax_by_token(&lang)
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
        let mut html_generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
        for line in LinesWithEndings::from(code.trim()) {
            let _ = html_generator.parse_html_for_line_which_includes_newline(line);
        }
        let initial_html = html_generator.finalize();
        let output_html: Vec<_> = initial_html
            .lines()
            .map(|line| format!(r#"<span class="numberedLine">{}</span>"#, line))
            .collect();
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        format!("{}", output_html.join("\n"))
    }

    #[instrument(skip(self))]
    pub fn page_head(&self, args: &[Value]) -> Vec<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        let response = match self.pages.get(&id) {
            Some(page) => page.head.clone(),
            None => vec![],
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response
    }

    #[instrument(skip(self))]
    pub fn page_scripts(&self, args: &[Value]) -> Vec<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        let response = match self.pages.get(&id) {
            Some(page) => page.scripts.clone(),
            None => vec![],
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response
    }

    #[instrument(skip(self))]
    pub fn page_stylesheets(&self, args: &[Value]) -> Vec<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        let response = match self.pages.get(&id) {
            Some(page) => page.stylesheets.clone(),
            None => vec![],
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response
    }

    #[instrument(skip(self))]
    pub fn get_cache(&self, key: &str) -> Option<CacheObject> {
        let now = Instant::now();
        let binding = self.cache.lock().unwrap();
        let response = match binding.get(key) {
            Some(obj) => Some(obj.clone()),
            None => None,
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response
    }

    #[instrument(skip(self))]
    pub fn get_subtree(&self, args: &[Value]) -> Vec<CollectionItem> {
        let now = Instant::now();
        let original_json = json!(args[1]);
        let original_collection: Collection = serde_json::from_value(original_json).unwrap();
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        original_collection.get_subtree(&args[0].to_string())
    }

    #[instrument(skip(self))]
    pub fn ilink(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let current_id = args[0].to_string();
        let target_id = args[1].to_string();
        let text = args[2].to_string();
        if current_id == target_id {
            event!(Level::DEBUG, "||{:?}||", now.elapsed());
            Some(text)
        } else {
            match self.pages.get(&target_id) {
                Some(page) => {
                    event!(Level::DEBUG, "||{:?}||", now.elapsed());
                    Some(format!(
                        r#"<a href="{}">{}</a>"#,
                        page.href.clone().unwrap(),
                        text
                    ))
                }
                None => None,
            }
        }
    }

    #[instrument(skip(self))]
    pub fn image(&self, args: &[Value]) -> Option<Image> {
        let now = Instant::now();
        let target_name = args[0].to_string();
        self.images.iter().find_map(|image| {
            if &target_name == &image.file_stem {
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                Some(image.clone())
            } else if &target_name == &image.file_name {
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                Some(image.clone())
            } else {
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                None
            }
        })
    }

    #[instrument(skip(self))]
    pub fn mp3(&self, args: &[Value]) -> Option<Mp3> {
        let now = Instant::now();
        let target_name = args[0].to_string();
        self.mp3s.iter().find_map(|mp3| {
            if &target_name == &mp3.file_stem {
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                Some(mp3.clone())
            } else if &target_name == &mp3.file_name {
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                Some(mp3.clone())
            } else {
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                None
            }
        })
    }

    // pub fn tlink(&self, args: &[Value]) -> Option<String> {
    //     let current_id = args[0].to_string();
    //     let target_id = args[1].to_string();
    //     if current_id == target_id {
    //         Some(text)
    //     } else {
    //         match self.pages.get(&target_id) {
    //             Some(page) => page.html_link.clone(),
    //             None => None,
    //         }
    //     }
    // }

    #[instrument(skip(self))]
    pub fn set_cache(&self, key: String, obj: CacheObject) -> Option<CacheObject> {
        let now = Instant::now();
        let mut binding = self.cache.lock().unwrap();
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        binding.insert(key, obj)
    }

    #[instrument(skip(self))]
    pub fn log_from_template(&self, args: &[Value]) -> String {
        let now = Instant::now();
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        "".to_string()
    }

    #[instrument(skip(self))]
    pub fn link_or_title(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let current_page_id = args[0].to_string();
        let target_page_id = args[1].to_string();
        if current_page_id == target_page_id {
            match self.pages.get(&target_page_id) {
                Some(_) => {
                    event!(Level::DEBUG, "||{:?}||", now.elapsed());
                    Some(format!(
                        r#"{}"#,
                        self.page_title(&[Value::from(target_page_id.clone())])
                            .unwrap(),
                    ))
                }
                None => None,
            }
        } else {
            match self.pages.get(&target_page_id) {
                Some(_) => {
                    event!(Level::DEBUG, "||{:?}||", now.elapsed());
                    Some(format!(
                        r#"<a href="{}">{}</a>"#,
                        self.page_href(&[Value::from(target_page_id.clone())])
                            .unwrap(),
                        self.page_title(&[Value::from(target_page_id.clone())])
                            .unwrap(),
                    ))
                }
                None => None,
            }
        }
    }

    // pub fn nav_link_title_link(&self, args: &[Value]) -> Option<String> {
    //     Some(format!(
    //         r#"<a href="{}">{}</a>"#,
    //         self.page_href(args).unwrap(),
    //         self.page_title(args).unwrap()
    //     ))
    // }

    #[instrument(skip(self))]
    pub fn page_ast(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => {
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                Some(serde_json::to_string::<Vec<Child>>(&page.ast).unwrap())
            }
            None => None,
        }
    }

    #[instrument(skip(self))]
    pub fn page_ast_pretty(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => {
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                Some(serde_json::to_string_pretty::<Vec<Child>>(&page.ast).unwrap())
            }
            None => None,
        }
    }

    #[instrument(skip(self))]
    pub fn page_folders(&self, args: &[Value]) -> Vec<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => {
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                page.source_path
                    .strip_prefix(&self.config.folders.content_root.clone())
                    .unwrap()
                    .parent()
                    .unwrap()
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string().to_lowercase())
                    .collect()
            }
            None => vec![],
        }
    }

    #[instrument(skip(self))]
    pub fn page_href(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => {
                if let Some(response) = page.ast.iter().find_map(|child| {
                    if let Child::Section(section) = child {
                        if &section.r#type == "metadata" {
                            section.key_value_attributes.iter().find_map(|attr| {
                                if attr.0 == "path" {
                                    event!(Level::DEBUG, "||{:?}||", now.elapsed());
                                    Some(Some(attr.1.to_string()))
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
                    response
                } else {
                    event!(Level::DEBUG, "||{:?}||", now.elapsed());
                    Some(format!(
                        "/{}/{}/?{}",
                        self.config.default_language,
                        id,
                        self.page_href_title(&id).unwrap()
                    ))
                }
            }
            None => None,
        }
    }

    // TODO: Forward to page
    #[instrument(skip(self))]
    pub fn page_href_title(&self, id: &str) -> Option<String> {
        let now = Instant::now();
        match self.page_title(&[Value::from(id)]) {
            Some(title) => {
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                Some(
                    urlencoding::encode(&title.to_lowercase().replace(" ", "-").to_string())
                        .into_owned(),
                )
            }
            None => None,
        }
    }

    #[instrument(skip(self))]
    pub fn page_html_link(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        let response = match self.pages.get(&id) {
            Some(page) => page.html_link.clone(),
            None => None,
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response
    }

    #[instrument(skip(self))]
    pub fn page_ids(&self) -> Vec<String> {
        let now = Instant::now();
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        self.pages.iter().map(|page| page.0.to_string()).collect()
    }

    #[instrument(skip(self))]
    pub fn page_main_body(&self, args: &[Value]) -> Value {
        let now = Instant::now();
        // event!(Level::DEBUG, "running page_main_body");
        if let Some(page) = self.pages.get(&args[0].to_string()) {
            // event!(Level::DEBUG, "{}", page.source_path.display());
            let response = Value::from_serializable(
                &page
                    .ast
                    .clone()
                    .into_iter()
                    .filter_map(|child| {
                        if let Child::Section(sec) = &child {
                            if self.config.main_body_section_excludes.contains(&sec.r#type) {
                                None
                            } else {
                                Some(child)
                            }
                        } else if let Child::List(sec) = &child {
                            if self.config.main_body_section_excludes.contains(&sec.r#type) {
                                None
                            } else {
                                Some(child)
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Child>>(),
            );
            event!(Level::DEBUG, "||{:?}||", now.elapsed());
            response
        } else {
            event!(Level::DEBUG, "||{:?}||", now.elapsed());
            Value::from_serializable::<Vec<Child>>(&vec![])
        }
    }

    // Deprecated, should be able to pull directly from the
    // page in the generate_files() approach
    #[instrument(skip(self))]
    pub fn page_build_path(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => match page.ast.iter().find_map(|child| {
                if let Child::Section(section) = child {
                    if &section.r#type == "metadata" {
                        section.key_value_attributes.iter().find_map(|attr| {
                            if attr.0 == "path" {
                                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                                Some(Some(attr.1.to_string()))
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
                Some(override_path) => {
                    let mut build_path = self.config.folders.build_root.clone();
                    build_path.push(override_path.unwrap().strip_prefix("/").unwrap());
                    build_path.push("index.html");
                    event!(Level::DEBUG, "||{:?}||", now.elapsed());
                    Some(build_path.display().to_string())
                }
                None => Some(format!(
                    "{}/{}/{}/index.html",
                    self.config.folders.build_root.display(),
                    self.config.default_language,
                    &id,
                )),
            },
            None => None,
        }
    }

    #[instrument(skip(self))]
    pub fn page_path_parts(&self, args: &[Value]) -> Vec<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => {
                // dbg!(&page.source_path);
                // dbg!(&self.config.folders.content_root.clone());
                let response = page
                    .source_path
                    .strip_prefix(&self.config.folders.content_root.clone())
                    .unwrap()
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string().to_lowercase())
                    .collect();
                event!(Level::DEBUG, "||{:?}||", now.elapsed());
                response
            }
            None => vec![],
        }
    }

    #[instrument(skip(self))]
    pub fn page_place_section(&self, args: &[Value]) -> Value {
        let now = Instant::now();
        let id = args[0].to_string();
        let section_type = args[1].to_string();
        match self.pages.get(&id) {
            Some(page) => page
                .ast
                .iter()
                .filter_map(|child| {
                    if let Child::Section(sec) = &child {
                        if sec.r#type == section_type {
                            Some(Value::from_serializable(child))
                        } else {
                            None
                        }
                    } else if let Child::List(sec) = &child {
                        if sec.r#type == section_type {
                            event!(Level::DEBUG, "||{:?}||", now.elapsed());
                            Some(Value::from_serializable(child))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect(),
            // Value::from_serializable::<Vec<String>>(&vec![]),
            None => Value::from_serializable::<Vec<String>>(&vec![]),
        }
    }

    // Deprecated - should be able to use the page directly, I think
    // in the .generate_files() approach
    #[instrument(skip(self))]
    pub fn page_source(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        let response = match self.pages.get(&id) {
            Some(page) => Some(page.source.clone()),
            None => None,
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response
    }

    #[instrument(skip(self))]
    pub fn page_source_path(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        let response = match self.pages.get(&id) {
            Some(page) => Some(page.source_path.display().to_string()),
            None => None,
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response
    }

    #[instrument(skip(self))]
    pub fn page_template(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        if self.pages.contains_key(&id) {
            let template_searches = vec![
                format!(
                    "pages/{}/{}.jinja",
                    self.page_type(args).unwrap(),
                    self.page_status(args).unwrap(),
                ),
                format!("pages/{}/published.jinja", self.page_type(args).unwrap()),
                format!("pages/post/{}.jinja", self.page_status(args).unwrap()),
                format!("pages/post/published.jinja"),
            ];
            event!(Level::DEBUG, "||{:?}||", now.elapsed());
            template_searches
                .iter()
                .find_map(|t| match self.templates.get(t) {
                    Some(_) => Some(t),
                    None => None,
                })
                .cloned()
        } else {
            None
        }
    }

    // NOTE: This is a stub that just calls .page_title()
    // until the actual functionality is built
    #[instrument(skip(self))]
    pub fn page_menu_title(&self, args: &[Value]) -> Option<String> {
        // let now = Instant::now();
        self.page_title(args)
    }

    #[instrument(skip(self))]
    pub fn page_status(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        let response = match self.pages.get(&id) {
            Some(page) => page.status.clone(),
            None => None,
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response
    }

    #[instrument(skip(self))]
    pub fn page_title(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        let response = match self.pages.get(&id) {
            Some(page) => Some(page.title.clone().unwrap()),
            None => None,
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response

        // let cache_id = format!("page-titles-{}", id);
        // match self.get_cache(&cache_id) {
        //     Some(page_title_cache) => {
        //         if let CacheObject::OptionString(page_title) = page_title_cache {
        //             page_title
        //         } else {
        //             None
        //         }
        //     }
        //     None => {
        //         let title = match self.pages.get(&id) {
        //             Some(page) => {
        //                 if let Some(title) = page_title_from_metadata(&page.ast) {
        //                     Some(title)
        //                 } else if let Some(title) = page_title_from_title_section(&page.ast) {
        //                     Some(title)
        //                 } else if let Some(title) = page_title_from_any_section(&page.ast) {
        //                     Some(title)
        //                 } else if let Some(title) = page_title_from_first_few_words(&page.ast) {
        //                     Some(title)
        //                 } else if let Some(title) = page_title_from_id(&page.ast) {
        //                     Some(title)
        //                 } else {
        //                     Some("no title".to_string())
        //                 }
        //             }
        //             None => Some("(missing page)".to_string()),
        //         };
        //         self.set_cache(cache_id, CacheObject::OptionString(title.clone()));
        //         title
        //     }
        // }
    }

    #[instrument(skip(self))]
    pub fn page_type(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let id = args[0].to_string();
        let response = match self.pages.get(&id) {
            Some(page) => page.r#type.clone(),
            None => None,
        };
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        response
    }

    // fn prep_cache(&self) {
    //     // NOTE: everything relies on the cache being set up. So,
    //     // everything unwraps directly. If something hasn't been
    //     // added yet it'll trigger an intended panic
    //     let mut c = self.cache.lock().unwrap();
    //     c.insert("page-titles".to_string(), BTreeMap::new());
    //     // TODO: DEPRECATE menus and move to nav_items
    //     c.insert("menus".to_string(), BTreeMap::new());
    //     c.insert("nav_items".to_string(), BTreeMap::new());
    // }

    #[instrument(skip(self))]
    pub fn show(&self, args: &[Value]) -> Option<String> {
        let now = Instant::now();
        let content = serde_json::to_string_pretty(
            &serde_json::from_str::<serde_json::Value>(&args[0].to_string()).unwrap(),
        );
        let code_type = "json";
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let syntax = syntax_set.find_syntax_by_token(code_type).unwrap();
        let mut html_generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
        for line in LinesWithEndings::from(&content.unwrap()) {
            let _ = html_generator.parse_html_for_line_which_includes_newline(line);
        }
        let initial_html = html_generator.finalize();
        let output_html: Vec<_> = initial_html
            .lines()
            .map(|line| format!(r#"<span class="linenumber">{}</span>"#, line))
            .collect();
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
        Some(format!(
            r#"<pre class="template_data_object"><code>{}</code></pre>"#,
            output_html.join("\n")
        ))
    }
}

// // MOVED
// fn filter_section(sec: &Section) -> Option<String> {
//     let SectionCategory::StandardSectionFull { containers } = &sec.category else {
//         return None;
//     };
//     let first = containers.first()?;
//     let Child::Block(thing) = first else {
//         return None;
//     };
//     let spans = thing
//         .iter()
//         .flat_map(|span| get_span_words(&span))
//         .collect::<String>();
//     Some(spans)
// }

// // MOVED
// fn get_span_words(span: &Span) -> Vec<String> {
//     match span {
//         Span::Word { text, .. } => {
//             vec![text.to_string()]
//         }
//         Span::Space { .. } => vec![" ".to_string()],
//         Span::StandardSpan { spans, .. } => spans
//             .iter()
//             .map(|span| get_span_words(&span))
//             .collect::<Vec<Vec<String>>>()
//             .concat(),
//         _ => vec!["".to_string()],
//     }
// }

impl Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
