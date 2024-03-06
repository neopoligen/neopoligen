pub mod new;
pub mod object;

use crate::cache_object::CacheObject;
use crate::child::Child;
use crate::collection::Collection;
use crate::config::Config;
use crate::page::Page;
use minijinja::Value;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Mutex;
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
}

impl Site {
    pub fn collection_from_files_and_folders(&self, args: &[Value]) -> Collection {
        let id = args[0].to_string();
        match args[1].try_iter() {
            Ok(value_patterns) => {
                let patterns = value_patterns
                    .filter_map(|value_pattern| match value_pattern.try_iter() {
                        Ok(parts) => Some(
                            parts
                                .filter_map(|p| Some(p.to_string()))
                                .collect::<Vec<String>>(),
                        ),
                        Err(_e) => None,
                    })
                    .collect::<Vec<_>>();
                let mut c = Collection::new_from_files_and_folders(&self.pages, patterns);
                c.set_active_item(&id);
                c
            }
            Err(_e) => Collection::empty(),
        }
    }

    #[instrument(skip(self))]
    pub fn get_cache(&self, key: &str) -> Option<CacheObject> {
        let binding = self.cache.lock().unwrap();
        match binding.get(key) {
            Some(obj) => Some(obj.clone()),
            None => None,
        }
    }

    pub fn ilink(&self, args: &[Value]) -> Option<String> {
        let current_id = args[0].to_string();
        let target_id = args[1].to_string();
        let text = args[2].to_string();
        if current_id == target_id {
            Some(text)
        } else {
            match self.pages.get(&target_id) {
                Some(page) => Some(format!(
                    r#"<a href="{}">{}</a>"#,
                    page.href.clone().unwrap(),
                    text
                )),
                None => None,
            }
        }
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
        let mut binding = self.cache.lock().unwrap();
        binding.insert(key, obj)
    }

    #[instrument(skip(self))]
    pub fn log_from_template(&self, args: &[Value]) -> String {
        event!(Level::INFO, "{}", args[0].to_string());
        "".to_string()
    }

    pub fn link_or_title(&self, args: &[Value]) -> Option<String> {
        let current_page_id = args[0].to_string();
        let target_page_id = args[1].to_string();
        if current_page_id == target_page_id {
            match self.pages.get(&target_page_id) {
                Some(_) => Some(format!(
                    r#"{}"#,
                    self.page_title(&[Value::from(target_page_id.clone())])
                        .unwrap(),
                )),
                None => None,
            }
        } else {
            match self.pages.get(&target_page_id) {
                Some(_) => Some(format!(
                    r#"<a href="{}">{}</a>"#,
                    self.page_href(&[Value::from(target_page_id.clone())])
                        .unwrap(),
                    self.page_title(&[Value::from(target_page_id.clone())])
                        .unwrap(),
                )),
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

    pub fn page_folders(&self, args: &[Value]) -> Vec<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => page
                .source_path
                .strip_prefix(&self.config.folders.content_root.clone())
                .unwrap()
                .parent()
                .unwrap()
                .components()
                .map(|c| c.as_os_str().to_string_lossy().to_string().to_lowercase())
                .collect(),
            None => vec![],
        }
    }

    pub fn page_href(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => {
                if let Some(response) = page.ast.iter().find_map(|child| {
                    if let Child::Section(section) = child {
                        if &section.r#type == "metadata" {
                            section.key_value_attributes.iter().find_map(|attr| {
                                if attr.0 == "path" {
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
    pub fn page_href_title(&self, id: &str) -> Option<String> {
        match self.page_title(&[Value::from(id)]) {
            Some(title) => Some(
                urlencoding::encode(&title.to_lowercase().replace(" ", "-").to_string())
                    .into_owned(),
            ),
            None => None,
        }
    }

    pub fn page_html_link(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => page.html_link.clone(),
            None => None,
        }
    }

    pub fn page_ids(&self) -> Vec<String> {
        self.pages.iter().map(|page| page.0.to_string()).collect()
    }

    // #[instrument(skip(self))]
    pub fn page_main_body(&self, args: &[Value]) -> Value {
        // event!(Level::INFO, "running page_main_body");
        if let Some(page) = self.pages.get(&args[0].to_string()) {
            // event!(Level::INFO, "{}", page.source_path.display());
            Value::from_serializable(
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
            )
        } else {
            Value::from_serializable::<Vec<Child>>(&vec![])
        }
    }

    pub fn page_output_path(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => match page.ast.iter().find_map(|child| {
                if let Child::Section(section) = child {
                    if &section.r#type == "metadata" {
                        section.key_value_attributes.iter().find_map(|attr| {
                            if attr.0 == "path" {
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
                    let mut output_path = self.config.folders.output_root.clone();
                    output_path.push(override_path.unwrap().strip_prefix("/").unwrap());
                    output_path.push("index.html");
                    Some(output_path.display().to_string())
                }
                None => Some(format!(
                    "{}/{}/{}/index.html",
                    self.config.folders.output_root.display(),
                    self.config.default_language,
                    &id,
                )),
            },
            None => None,
        }
        // match self.pages.get(&id) {
        //     Some(_) => Some(format!(
        //         "{}/{}/{}/index.html",
        //         self.config.folders.output_root.display(),
        //         self.config.default_language,
        //         &id,
        //     )),
        //     None => None,
        // }
    }

    pub fn page_path_parts(&self, args: &[Value]) -> Vec<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => {
                // dbg!(&page.source_path);
                // dbg!(&self.config.folders.content_root.clone());
                page.source_path
                    .strip_prefix(&self.config.folders.content_root.clone())
                    .unwrap()
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string().to_lowercase())
                    .collect()
            }
            None => vec![],
        }
    }

    pub fn page_place_section(&self, args: &[Value]) -> Value {
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

    pub fn page_source_path(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => Some(page.source_path.display().to_string()),
            None => None,
        }
    }

    pub fn page_status(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => match page.ast.iter().find_map(|child| {
                if let Child::Section(section) = child {
                    if &section.r#type == "metadata" {
                        section.key_value_attributes.iter().find_map(|attr| {
                            if attr.0 == "status" {
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
                Some(type_from_metadata) => type_from_metadata,
                None => Some("published".to_string()),
            },
            None => None,
        }
    }

    pub fn page_template(&self, args: &[Value]) -> Option<String> {
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
    pub fn page_menu_title(&self, args: &[Value]) -> Option<String> {
        self.page_title(args)
    }

    pub fn page_title(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => Some(page.title.clone().unwrap()),
            None => None,
        }

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

    pub fn page_type(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => match page.ast.iter().find_map(|child| {
                if let Child::Section(section) = child {
                    if &section.r#type == "metadata" {
                        section.key_value_attributes.iter().find_map(|attr| {
                            if attr.0 == "type" {
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
                Some(type_from_metadata) => type_from_metadata,
                None => Some("post".to_string()),
            },
            None => None,
        }
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

    pub fn show(&self, args: &[Value]) -> Option<String> {
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
