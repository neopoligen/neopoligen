pub mod builder;
pub mod display;
pub mod filter_pages_alpha;
pub mod load_pages;
pub mod new;
pub mod object;

use crate::child::Child;
use crate::config::Config;
use crate::folder_menu_item::FolderMenuItem;
use crate::page::Page;
use minijinja::Value;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Mutex;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct SiteV2 {
    pub config: Config,
    pub pages: BTreeMap<String, Page>,
    pub page_templates: BTreeMap<String, String>,
    pub holder: Mutex<BTreeMap<String, String>>,
}

impl SiteV2 {
    pub fn domain(&self) -> Option<String> {
        Some(self.config.domain.trim_end_matches('/').to_string())
    }

    pub fn domain_no_https(&self) -> Option<String> {
        Some(
            self.config
                .domain
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .trim_end_matches("/")
                .to_string(),
        )
    }

    pub fn filtered_pages_alpha(&self, args: &[Value]) -> Vec<String> {
        let mut exclude_set: BTreeSet<String> = BTreeSet::new();
        let mut include_set: BTreeSet<BTreeSet<String>> = BTreeSet::new();
        let _ = &args[0].try_iter().unwrap().for_each(|l1| {
            let mut l1_set: BTreeSet<String> = BTreeSet::new();
            l1.try_iter().unwrap().for_each(|l2| {
                l1_set.insert(l2.to_string());
            });
            include_set.insert(l1_set);
        });
        let _ = &args[1].try_iter().unwrap().for_each(|v| {
            exclude_set.insert(v.to_string());
        });
        self.pages
            .iter()
            .filter_map(|p| {
                let page_filters = p.1.filter_set();
                if !page_filters.is_disjoint(&exclude_set) {
                    None
                } else {
                    include_set.iter().find_map(|inc| {
                        if inc.is_subset(&page_filters) {
                            Some(p.1.id().unwrap())
                        } else {
                            None
                        }
                    })
                }
            })
            .collect()
    }

    pub fn folder_for_page(&self, args: &[Value]) -> Option<String> {
        if let Ok(folder_request) = &args[1].clone().try_into() {
            let folder_index = folder_request - 1usize;
            if let Some(page) = self.pages.get(&args[0].to_string()) {
                let folders = page.folders();
                if folders.len() >= folder_index {
                    Some(folders[folder_index].clone())
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

    // pub fn make_folder_menu_data(&self) {
    //     let mut binding = self.holder.lock().unwrap();
    //     binding.insert("initial_menu_data".to_string(), "asdf".to_string());
    // }

    pub fn folder_menu(&self, args: &[Value]) -> Vec<FolderMenuItem> {
        let mut binding = self.holder.lock().unwrap();
        match binding.get("initial_menu_data") {
            Some(data) => serde_json::from_str(&data).unwrap(),
            None => {
                let r = args[1]
                    .try_iter()
                    .unwrap()
                    .filter_map(|folder_vecs| {
                        let folder_pattern: Vec<String> = folder_vecs
                            .try_iter()
                            .unwrap()
                            .map(|f| f.to_string())
                            .collect();
                        self.folder_menu_index_finder(folder_pattern)
                    })
                    .collect();
                binding.insert(
                    "initial_menu_data".to_string(),
                    serde_json::to_string(&r).expect("json serializtion"),
                );
                r
            }
        }
    }

    pub fn folder_menu_legacy(&self, args: &[Value]) -> Vec<FolderMenuItem> {
        let binding = self.holder.lock().unwrap();
        match binding.get("initial_menu_data") {
            Some(_) => println!("got it"),
            None => {}
        }
        let r = args[1]
            .try_iter()
            .unwrap()
            .filter_map(|folder_vecs| {
                let folder_pattern: Vec<String> = folder_vecs
                    .try_iter()
                    .unwrap()
                    .map(|f| f.to_string())
                    .collect();
                self.folder_menu_index_finder(folder_pattern)
            })
            .collect();
        r
    }

    pub fn folder_menu_index_finder(&self, pattern: Vec<String>) -> Option<FolderMenuItem> {
        let mut full_pattern_with_file = pattern.clone();
        full_pattern_with_file.push("_index.neo".to_string());
        self.pages.iter().find_map(|page| {
            if full_pattern_with_file == page.1.path_parts() {
                let mut fmi = FolderMenuItem {
                    page_id: page.1.id().unwrap(),
                    is_current_link: false,
                    title: page.1.full_title(),
                    href: page.1.href(),
                    children: self.folder_menu_child_item_finder(&pattern),
                };
                // TODO: Get sub folders here
                let mut next_folders: Vec<FolderMenuItem> =
                    self.folder_menu_subfolder_finder(&pattern);
                fmi.children.append(&mut next_folders);
                Some(fmi)
            } else {
                None
            }
        })
    }

    pub fn folder_menu_subfolder_finder(&self, pattern: &Vec<String>) -> Vec<FolderMenuItem> {
        let mut next_level_folders: BTreeSet<Vec<String>> = BTreeSet::new();
        self.pages.iter().for_each(|page| {
            if page
                .1
                .folders()
                .iter()
                .take(pattern.len())
                .eq(pattern.clone().iter())
            {
                if page.1.folders().len() == pattern.len() + 1 {
                    next_level_folders.insert(page.1.folders());
                }
            }
        });
        next_level_folders
            .iter()
            .filter_map(|pat| self.folder_menu_index_finder(pat.clone()))
            .collect()
    }

    pub fn folder_menu_child_item_finder(&self, pattern: &Vec<String>) -> Vec<FolderMenuItem> {
        let mut full_pattern_with_file = pattern.clone();
        full_pattern_with_file.push("_index.neo".to_string());
        self.pages
            .iter()
            .filter_map(|page| {
                if &page.1.folders() == pattern && page.1.path_parts() != full_pattern_with_file {
                    let fmi = FolderMenuItem {
                        page_id: page.1.id().unwrap(),
                        is_current_link: false,
                        title: page.1.full_title(),
                        href: page.1.href(),
                        children: vec![],
                    };
                    Some(fmi)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn href_for_page(&self, args: &[Value]) -> Option<String> {
        if let Some(page) = self.pages.get(&args[0].to_string()) {
            page.href()
        } else {
            None
        }
    }

    pub fn image_path_for(&self, args: &[Value]) -> Option<String> {
        let image_root = self.config.folders.site_images_root.clone();
        let image_name = args[0].to_string();
        let files: Vec<_> = WalkDir::new(&image_root)
            .into_iter()
            .filter_map(|v| {
                if let Some(name) = v.as_ref().unwrap().path().file_stem() {
                    let target_name_stem = PathBuf::from(&image_name);
                    match target_name_stem.file_stem() {
                        Some(chopped) => {
                            if name.to_str() == chopped.to_str() {
                                match v.as_ref() {
                                    Ok(p) => {
                                        let dir = p.path().strip_prefix(image_root.clone());
                                        let mut return_path_buf = PathBuf::from("/images");
                                        return_path_buf.push(
                                            dir.expect("to string for image path")
                                                .to_str()
                                                .unwrap(),
                                        );
                                        Some(return_path_buf.display().to_string())
                                    }
                                    Err(_e) => {
                                        println!("ERROR with image path");
                                        None
                                    }
                                }
                            } else {
                                None
                            }
                        }
                        None => None,
                    }
                } else {
                    None
                }
            })
            .collect();
        if files.len() == 1 {
            Some(files[0].clone())
        } else {
            None
        }
    }

    pub fn include_section_from_page(&self, args: &[Value]) -> Vec<Child> {
        if let Some(page) = self.pages.get(&args[1].to_string()) {
            page.section_by_id(args[2].to_string())
        } else {
            vec![]
        }
    }

    pub fn link_or_title(&self, args: &[Value]) -> Option<String> {
        let current_page_id = args[0].to_string();
        let target_page_id = args[1].to_string();
        let mut exclude_tags: BTreeSet<String> = BTreeSet::new();
        let _ = &args[2].try_iter().unwrap().for_each(|t| {
            exclude_tags.insert(t.to_string());
        });
        let title_override = args[3].to_string();
        if let Some(current_page) = &self.pages.get(&current_page_id) {
            if let Some(target_page) = &self.pages.get(&target_page_id) {
                let title = if title_override != "" {
                    title_override
                } else {
                    target_page.full_title().unwrap()
                };
                if target_page.id().unwrap() == current_page.id().unwrap() {
                    target_page.full_title()
                } else if exclude_tags.len() == 0 {
                    Some(format!(
                        r#"<a href="{}">{}</a>"#,
                        target_page.href().unwrap(),
                        title
                    ))
                } else if current_page.filter_set().is_disjoint(&exclude_tags) {
                    Some(format!(
                        r#"<a href="{}">{}</a>"#,
                        target_page.href().unwrap(),
                        title
                    ))
                } else {
                    Some(title)
                }
            } else {
                // TODO: Output error message here
                Some("(page no longer available)".to_string())
            }
        } else {
            // TODO: Output error message here
            Some("(page link error)".to_string())
        }
    }

    // let current_id = args[0].to_string();
    // if let Some(current_page) = &self.pages.get(&current_id.to_string()) {
    //     let mut exclude_tags: BTreeSet<String> = BTreeSet::new();
    //     match args[1].get_attr("exclude_tags") {
    //         Ok(exclude_tags_raw) => {
    //             if !exclude_tags_raw.is_undefined() {
    //                 exclude_tags_raw.try_iter().unwrap().for_each(|exclude| {
    //                     exclude_tags.insert(exclude.to_string());
    //                 });
    //             }
    //         }
    //         Err(_) => {}
    //     };
    //     let return_value = match args[1].get_attr("target_page") {
    //         Ok(target_page_id_value) => {
    //             if !target_page_id_value.is_undefined() {
    //                 let target_page_id = target_page_id_value.to_string();
    //                 if let Some(target_page) = self.pages.get(&target_page_id) {
    //                     let title = match args[1].get_attr("title_override") {
    //                         Ok(title_override_value) => {
    //                             if !title_override_value.is_undefined() {
    //                                 title_override_value.to_string()
    //                             } else {
    //                                 target_page.full_title().unwrap()
    //                             }
    //                         }
    //                         Err(_) => "no_title_available".to_string(),
    //                     };
    //                     if target_page.id().unwrap() == current_id {
    //                         Some(title.to_string())
    //                     } else if exclude_tags.len() == 0 {
    //                         Some(format!(
    //                             r#"<a href="{}">{}</a>"#,
    //                             target_page.href().unwrap(),
    //                             title
    //                         ))
    //                     } else if current_page.filter_set().is_disjoint(&exclude_tags) {
    //                         Some(format!(
    //                             r#"<a href="{}">{}</a>"#,
    //                             target_page.href().unwrap(),
    //                             title
    //                         ))
    //                     } else {
    //                         Some(title.to_string())
    //                     }
    //                 } else {
    //                     Some("page no longer available".to_string())
    //                 }
    //             } else {
    //                 Some("page no longer available".to_string())
    //             }
    //         }
    //         Err(_) => Some("page no longer available".to_string()),
    //     };
    //     return_value
    // } else {
    //     Some("page no longer available".to_string())
    // }

    // pub fn link_or_title(&self, args: &[Value]) -> Option<String> {
    //     let current_id = args[0].to_string();
    //     if let Some(current_page) = &self.pages.get(&current_id.to_string()) {
    //         let mut exclude_tags: BTreeSet<String> = BTreeSet::new();
    //         match args[1].get_attr("exclude_tags") {
    //             Ok(exclude_tags_raw) => {
    //                 if !exclude_tags_raw.is_undefined() {
    //                     exclude_tags_raw.try_iter().unwrap().for_each(|exclude| {
    //                         exclude_tags.insert(exclude.to_string());
    //                     });
    //                 }
    //             }
    //             Err(_) => {}
    //         };
    //         let return_value = match args[1].get_attr("target_page") {
    //             Ok(target_page_id_value) => {
    //                 if !target_page_id_value.is_undefined() {
    //                     let target_page_id = target_page_id_value.to_string();
    //                     if let Some(target_page) = self.pages.get(&target_page_id) {
    //                         let title = match args[1].get_attr("title_override") {
    //                             Ok(title_override_value) => {
    //                                 if !title_override_value.is_undefined() {
    //                                     title_override_value.to_string()
    //                                 } else {
    //                                     target_page.full_title().unwrap()
    //                                 }
    //                             }
    //                             Err(_) => "no_title_available".to_string(),
    //                         };
    //                         if target_page.id().unwrap() == current_id {
    //                             Some(title.to_string())
    //                         } else if exclude_tags.len() == 0 {
    //                             Some(format!(
    //                                 r#"<a href="{}">{}</a>"#,
    //                                 target_page.href().unwrap(),
    //                                 title
    //                             ))
    //                         } else if current_page.filter_set().is_disjoint(&exclude_tags) {
    //                             Some(format!(
    //                                 r#"<a href="{}">{}</a>"#,
    //                                 target_page.href().unwrap(),
    //                                 title
    //                             ))
    //                         } else {
    //                             Some(title.to_string())
    //                         }
    //                     } else {
    //                         Some("page no longer available".to_string())
    //                     }
    //                 } else {
    //                     Some("page no longer available".to_string())
    //                 }
    //             }
    //             Err(_) => Some("page no longer available".to_string()),
    //         };
    //         return_value
    //     } else {
    //         Some("page no longer available".to_string())
    //     }
    // }

    pub fn log_from_template(&self, args: &[Value]) -> String {
        println!("{}", &args[0].to_string());
        // Return an empty string so "none" doesn't
        // show up in the output
        "".to_string()
    }

    pub fn main_body_for_page(&self, args: &[Value]) -> Vec<Child> {
        if let Some(page) = self.pages.get(&args[0].to_string()) {
            page.main_body()
        } else {
            vec![]
        }
    }

    // // This is for making menus by looking for `-- children``
    // // attributes and generating them that way. There aren't
    // // currently tests on it since it took a while to figure out
    // // how to get it to go.
    // // This works, but I'm not really using it for now.
    // // I'm working on the folder based solution which I think
    // // will be less friction to use. Still keeping this
    // // around though
    // pub fn page_id_menu(&self, args: &[Value]) -> Vec<MenuItem> {
    //     let z = args[1]
    //         .try_iter()
    //         .unwrap()
    //         // .filter_map(|page_id| self.page_menu_item(page_id.as_str().unwrap()))
    //         .map(|page_id| self.page_id_menu_process_folder_for_id(page_id.as_str().unwrap()))
    //         .flatten()
    //         .collect::<Vec<MenuItem>>();
    //     z
    // }

    // // This is the support for page_id_menu
    // pub fn page_id_menu_process_folder_for_id(&self, page_id: &str) -> Vec<MenuItem> {
    //     match self.pages.get(page_id) {
    //         Some(page) => {
    //             let mut response_vec = vec![];
    //             let menu_item = MenuItem {
    //                 page_id: page_id.to_string(),
    //                 children: page
    //                     .children()
    //                     .iter()
    //                     .map(|child_id| self.page_id_menu_process_folder_for_id(child_id.as_str()))
    //                     .flatten()
    //                     .collect(),
    //             };
    //             response_vec.push(menu_item);
    //             response_vec.append(
    //                 &mut self
    //                     .pages
    //                     .iter()
    //                     .filter_map(|check_page| {
    //                         if check_page.1.id().unwrap().as_str() != page_id {
    //                             if page.folders() == check_page.1.folders() {
    //                                 Some(MenuItem {
    //                                     page_id: check_page.1.id().unwrap().to_string(),
    //                                     children: check_page
    //                                         .1
    //                                         .children()
    //                                         .iter()
    //                                         .map(|child_id| {
    //                                             self.page_id_menu_process_folder_for_id(
    //                                                 child_id.as_str(),
    //                                             )
    //                                         })
    //                                         .flatten()
    //                                         .collect(),
    //                                     // children: vec![],
    //                                 })
    //                             } else {
    //                                 None
    //                             }
    //                         } else {
    //                             None
    //                         }
    //                     })
    //                     .collect::<Vec<MenuItem>>(),
    //             );
    //             response_vec
    //         }
    //         None => vec![],
    //     }
    // }

    pub fn page_ids(&self) -> Vec<String> {
        self.pages.iter().map(|p| p.1.id().unwrap()).collect()
    }

    pub fn page_ast(&self, args: &[Value]) -> Vec<Child> {
        if let Some(page) = self.pages.get(&args[0].to_string()) {
            page.ast.clone()
        } else {
            vec![]
        }
    }

    pub fn page_source(&self, args: &[Value]) -> Option<String> {
        if let Some(page) = self.pages.get(&args[0].to_string()) {
            Some(page.source())
        } else {
            None
        }
    }

    pub fn output_path_for_page(&self, args: &[Value]) -> Option<String> {
        let current_id = args[0].to_string();
        match self.pages.get(&current_id) {
            Some(p) => Some(p.output_path().unwrap().display().to_string()),
            None => None,
        }
    }

    pub fn template_for_page(&self, args: &[Value]) -> Option<String> {
        let current_id = args[0].to_string();
        match self.page_templates.get(&current_id) {
            Some(template) => Some(template.to_string()),
            None => None,
        }
    }

    pub fn place_section(&self, args: &[Value]) -> Vec<Child> {
        if let Some(page) = self.pages.get(&args[0].to_string()) {
            page.section(args[1].to_string())
        } else {
            vec![]
        }
    }

    pub fn title_for_page(&self, args: &[Value]) -> Option<String> {
        if let Some(page) = self.pages.get(&args[0].to_string()) {
            page.full_title()
        } else {
            None
        }
    }
}
