use crate::config::Config;
use crate::config::JsonConfig;
use crate::config_folders::ConfigFolders;
use crate::config_section_categories::ConfigSectionCategories;
use crate::helpers::file_exists::file_exists;
use crate::helpers::get_file_paths_for_extension::*;
use crate::helpers::get_folders_in_folder::*;
use chrono::offset::Local;
use itertools::sorted;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

impl Config {
    pub fn new(project_root: PathBuf) -> Config {
        let json_config_path =
            PathBuf::from(format!("{}/{}", project_root.display(), "config.json"));

        let json_config = load_config_file(json_config_path).unwrap();

        let configuration_root =
            PathBuf::from(format!("{}/{}", project_root.display(), "configuration"));
        // Deprecated: TODO - get this from the JSON
        // let default_language = json_config.default_language;

        let files_root = PathBuf::from(format!("{}/{}", project_root.display(), "files"));
        let images_root = PathBuf::from(format!("{}/{}", project_root.display(), "images"));
        let mp3s_root = PathBuf::from(format!("{}/{}", project_root.display(), "mp3s"));
        let plugins_root = PathBuf::from(format!("{}/{}", project_root.display(), "plugins"));
        let content_root = PathBuf::from(format!("{}/{}", project_root.display(), "content"));
        let themes_root = PathBuf::from(format!("{}/{}", project_root.display(), "themes"));
        let output_root = PathBuf::from(format!("{}/{}", project_root.display(), "docs"));
        let status_root = PathBuf::from(format!("{}/{}", project_root.display(), "status"));
        let build_root = PathBuf::from(format!("{}/{}", project_root.display(), ".building"));
        // let parsing_errors_root =
        //  PathBuf::from(format!("{}/parsing-errors", status_root.display(),));
        // let theme_errors_root = PathBuf::from(format!("{}/theme-errors", status_root.display(),));

        let theme_name = get_config_file_single_line(&configuration_root, "theme.txt").unwrap();

        let mut theme_root = themes_root.clone();
        theme_root.push(&theme_name);

        let mut theme_configuration_root = theme_root.clone();
        theme_configuration_root.push("_configuration");

        let mut theme_assets_input_root = theme_root.clone();
        theme_assets_input_root.push("_assets");

        let mut theme_assets_build_root = build_root.clone();
        theme_assets_build_root.push("theme");

        let mut theme_sections_root = theme_root.clone();
        theme_sections_root.push("sections");

        let mut theme_helpers_root = theme_root.clone();
        theme_helpers_root.push("helpers");

        let mut theme_includes_root = theme_root.clone();
        theme_includes_root.push("includes");

        let mut theme_page_types_root = theme_root.clone();
        theme_page_types_root.push("pages");

        let mut theme_spans_root = theme_root.clone();
        theme_spans_root.push("spans");

        let mut theme_tests_root = theme_root.clone();
        theme_tests_root.push("_tests");

        // let mut theme_wrappers_root = theme_root.clone();
        // theme_wrappers_root.push("wrappers");

        let folders = ConfigFolders {
            build_root,
            configuration_root: configuration_root.clone(),
            files_root,
            images_root,
            mp3s_root,
            output_root,
            plugins_root,
            content_root,
            // parsing_errors_root,
            // theme_errors_root,
            status_root,
            project_root,
            themes_root,
            theme_assets_input_root,
            theme_assets_build_root,
            theme_configuration_root: theme_configuration_root.clone(),
            theme_sections_root: theme_sections_root.clone(),
            // theme_helpers_root,
            // theme_includes_root,
            // theme_page_types_root,
            theme_spans_root,
            theme_tests_root,
            theme_root,
            // theme_wrappers_root,
        };

        // Section Categories
        let section_folders = get_folders_in_folder(&theme_sections_root.clone());

        let mut checklist: BTreeSet<String> = BTreeSet::new();
        let mut comment: BTreeSet<String> = BTreeSet::new();
        let mut detail: BTreeSet<String> = BTreeSet::new();
        let mut json: BTreeSet<String> = BTreeSet::new();
        let mut list: BTreeSet<String> = BTreeSet::new();
        let mut raw: BTreeSet<String> = BTreeSet::new();
        let mut standard: BTreeSet<String> = BTreeSet::new();
        let mut table: BTreeSet<String> = BTreeSet::new();
        let mut yaml: BTreeSet<String> = BTreeSet::new();

        section_folders.iter().for_each(|section_folder| {
            let mut category_file = section_folder.clone();
            category_file.push("category.txt");
            if file_exists(category_file.clone()) {
                let category_data = fs::read_to_string(category_file).unwrap();
                let thing = section_folder
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                // dbg!(&thing);
                match category_data.trim() {
                    "checklist" => checklist.insert(thing),
                    "comment" => comment.insert(thing),
                    "detail" => detail.insert(thing),
                    "json" => json.insert(thing),
                    "list" => list.insert(thing),
                    "raw" => raw.insert(thing),
                    "standard" => standard.insert(thing),
                    "table" => table.insert(thing),
                    "yaml" => yaml.insert(thing),
                    _ => true,
                };
            }
        });

        let main_body_section_excludes =
            get_config_file_lines(&theme_configuration_root, "main-body-excludes.txt");

        // Plugins

        // TODO: Add the plug section names here
        let json_plugin_sections = BTreeSet::new();
        let text_plugin_sections = BTreeSet::new();

        let json_plugins: BTreeMap<String, String> = BTreeMap::new();
        let text_plugins: BTreeMap<String, String> = BTreeMap::new();

        // get_folders_in_folder(&folders.plugins_root)
        //     .iter()
        //     .for_each(|p| {
        //         let plugin_name = p.file_name().unwrap();
        //         let mut type_file = p.clone();
        //         type_file.push("type.txt");
        //         let type_contents = fs::read_to_string(type_file);
        //         let plugin_type = type_contents.unwrap().as_str().trim().to_string();
        //         let mut sections_file = p.clone();
        //         sections_file.push("sections.txt");
        //         let sections_contents = fs::read_to_string(sections_file);
        //         let section_lines: Vec<String> = sections_contents
        //             .unwrap()
        //             .as_str()
        //             .trim()
        //             .lines()
        //             .map(|line| line.trim().to_string())
        //             .collect();
        //         if !section_lines.is_empty() {
        //             if plugin_type == *"json" {
        //                 section_lines.iter().for_each(|ln| {
        //                     json_plugins
        //                         .insert(ln.to_string(), plugin_name.to_string_lossy().to_string());
        //                 })
        //             } else if plugin_type == *"text" {
        //                 section_lines.iter().for_each(|ln| {
        //                     text_plugins
        //                         .insert(ln.to_string(), plugin_name.to_string_lossy().to_string());
        //                 })
        //             }
        //         };
        //     });

        let now = Local::now();
        let time_zone_offset = now.offset();

        let span_file_paths = get_file_paths_for_extension(&folders.theme_spans_root, "neojinja");
        // dbg!(&span_file_paths);
        let unsorted_standard_spans: Vec<String> = span_file_paths
            .iter()
            .filter_map(|path| {
                let check_text = fs::read_to_string(path).unwrap();
                let text_matches: Vec<&str> = check_text.matches("type: standard").collect();
                // dbg!(&path.file_stem().unwrap());
                if !text_matches.is_empty() {
                    Some(path.file_stem().unwrap().to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect();
        let standard_spans = sorted(unsorted_standard_spans).rev().collect();

        let span_file_paths = get_file_paths_for_extension(&folders.theme_spans_root, "neojinja");
        let unsorted_key_value_spans: Vec<String> = span_file_paths
            .iter()
            .filter_map(|path| {
                let check_text = fs::read_to_string(path).unwrap();
                let text_matches: Vec<&str> = check_text.matches("type: key-value").collect();
                // dbg!(&path.file_stem().unwrap());
                if !text_matches.is_empty() {
                    Some(path.file_stem().unwrap().to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect();
        let key_value_spans = sorted(unsorted_key_value_spans).rev().collect();

        let input_date_formats =
            get_config_file_lines(&configuration_root, "input-date-formats.txt");

        let section_attribute_excludes =
            get_config_file_lines(&theme_configuration_root, "section-attribute-excludes.txt");

        let section_categories = ConfigSectionCategories {
            checklist,
            comment,
            detail,
            json,
            json_plugin: json_plugin_sections,
            list,
            raw,
            standard,
            table,
            text_plugin: text_plugin_sections,
            yaml,
        };

        Config {
            //
            // default_language,
            folders,
            input_date_formats,
            json_config,
            json_plugins,
            key_value_spans,
            main_body_section_excludes,
            section_attribute_excludes,
            section_categories,
            standard_spans,
            text_plugins,
            theme_name,
            time_zone_offset: time_zone_offset.to_string(),
        }
    }
}

// Deprecated: TODO: Remove this in favor of using the JSON
fn get_config_file_lines(file_dir: &PathBuf, file_name: &str) -> Vec<String> {
    let mut file_path = file_dir.clone();
    file_path.push(file_name);
    match fs::read_to_string(&file_path) {
        Ok(data) => data
            .lines()
            .filter_map(|line| {
                if line.trim().starts_with("#") {
                    None
                } else if line.trim() != "" {
                    Some(line.trim().to_string())
                } else {
                    None
                }
            })
            .collect(),
        Err(e) => panic!(
            "\nERROR: Could not read config file:\n({})\n{}\n",
            e,
            &file_path.display()
        ),
    }
}

fn get_config_file_single_line(file_dir: &PathBuf, file_name: &str) -> Option<String> {
    let mut file_path = file_dir.clone();
    file_path.push(file_name);
    match fs::read_to_string(&file_path) {
        Ok(data) => data.lines().find_map(|line| {
            if line.trim().starts_with("#") {
                None
            } else if line.trim() != "" {
                Some(line.trim().to_string())
            } else {
                None
            }
        }),
        Err(e) => panic!(
            "\nERROR: Could not read config file:\n({})\n{}\n",
            e,
            &file_path.display()
        ),
    }
}

#[cfg(test)]
mod test {
    // NOTE: This is basically all file system
    // stuff. TODO is to add better error
    // messages for missing files
}

fn load_config_file(path: PathBuf) -> Result<JsonConfig, String> {
    match path.try_exists() {
        Ok(exists) => {
            if exists == true {
                match fs::read_to_string(&path) {
                    Ok(text) => match serde_json::from_str::<JsonConfig>(text.as_str()) {
                        Ok(data) => Ok(data),
                        Err(e) => Err(format!(
                            "Could not parse JSON file: {}\n{}",
                            &path.display(),
                            e
                        )),
                    },
                    Err(_) => Err(format!("Could not read JSON file: {}", &path.display())),
                }
            } else {
                Err(format!("Could not read JSON file: {}", &path.display()))
            }
        }
        Err(_) => Err(format!("No file at: {}", &path.display())),
    }
}
