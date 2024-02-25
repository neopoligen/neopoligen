use crate::config::Config;
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
        let configuration_root =
            PathBuf::from(format!("{}/{}", project_root.display(), "configuration"));

        let default_language =
            get_config_file_lines(&configuration_root, "default-language.txt")[0].clone();

        let files_root = PathBuf::from(format!("{}/{}", project_root.display(), "files"));

        let images_root = PathBuf::from(format!("{}/{}", project_root.display(), "images"));

        let plugins_root = PathBuf::from(format!("{}/{}", project_root.display(), "plugins"));

        let content_root = PathBuf::from(format!("{}/{}", project_root.display(), "content"));

        let themes_root = PathBuf::from(format!("{}/{}", project_root.display(), "themes"));

        let output_root = PathBuf::from(format!("{}/{}", project_root.display(), "docs"));

        // TODO: This is the way to do things now with pulling lines like this
        // change anything that calls config files to this approach
        let theme_name = get_config_file_lines(&configuration_root, "theme.txt")
            .first()
            .unwrap()
            .to_string();

        // let theme_name_config_file = PathBuf::from(format!(
        //     "{}/{}",
        //     configuration_root.display(),
        //     "theme-to-use.txt"
        // ));

        // let theme_name = fs::read_to_string(theme_name_config_file)
        //     .unwrap()
        //     .as_str()
        //     .trim()
        //     .to_string();

        let mut theme_root = themes_root.clone();
        theme_root.push(&theme_name);

        let mut theme_configuration_root = theme_root.clone();
        theme_configuration_root.push("configuration");

        let mut theme_assets_root = theme_root.clone();
        theme_assets_root.push("theme-assets");

        let mut theme_sections_root = theme_root.clone();
        theme_sections_root.push("sections");

        let mut theme_helpers_root = theme_root.clone();
        theme_helpers_root.push("helpers");

        let mut theme_includes_root = theme_root.clone();
        theme_includes_root.push("includes");

        let mut theme_page_types_root = theme_root.clone();
        theme_page_types_root.push("page_types");

        let mut theme_spans_root = theme_root.clone();
        theme_spans_root.push("spans");

        let mut theme_tests_root = theme_root.clone();
        theme_tests_root.push("tests");

        let mut theme_wrappers_root = theme_root.clone();
        theme_wrappers_root.push("wrappers");

        let folders = ConfigFolders {
            configuration_root: configuration_root.clone(),
            files_root,
            images_root,
            output_root,
            plugins_root,
            content_root,
            project_root,
            themes_root,
            theme_assets_root,
            theme_configuration_root: theme_configuration_root.clone(),
            theme_sections_root: theme_sections_root.clone(),
            theme_helpers_root,
            theme_includes_root,
            theme_page_types_root,
            theme_spans_root,
            theme_tests_root,
            theme_root,
            theme_wrappers_root,
        };

        // Section Categories
        let section_folders = get_folders_in_folder(&theme_sections_root.clone());

        // TODO: Rename "preformatted" to "raw" in the
        // rest of the code (it's already been done here)
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
            get_config_file_lines(&configuration_root, "main-body-section-excludes.txt");

        // let main_body_section_excludes_file = PathBuf::from(format!(
        //     "{}/{}",
        //     &folders.theme_configuration_root.display(),
        //     "main-body-section-excludes.txt"
        // ));

        // let mut main_body_section_excludes: BTreeSet<String> = BTreeSet::new();
        // fs::read_to_string(main_body_section_excludes_file)
        //     .unwrap()
        //     .as_str()
        //     .lines()
        //     .for_each(|sec| {
        //         main_body_section_excludes.insert(sec.to_string());
        //     });

        // Plugins

        // TODO: Add the plug section names here
        let json_plugin_sections = BTreeSet::new();
        let text_plugin_sections = BTreeSet::new();

        let mut json_plugins: BTreeMap<String, String> = BTreeMap::new();
        let mut text_plugins: BTreeMap<String, String> = BTreeMap::new();
        get_folders_in_folder(&folders.plugins_root)
            .iter()
            .for_each(|p| {
                let plugin_name = p.file_name().unwrap();
                let mut type_file = p.clone();
                type_file.push("type.txt");
                let type_contents = fs::read_to_string(type_file);
                let plugin_type = type_contents.unwrap().as_str().trim().to_string();
                let mut sections_file = p.clone();
                sections_file.push("sections.txt");
                let sections_contents = fs::read_to_string(sections_file);
                let section_lines: Vec<String> = sections_contents
                    .unwrap()
                    .as_str()
                    .trim()
                    .lines()
                    .map(|line| line.trim().to_string())
                    .collect();
                if !section_lines.is_empty() {
                    if plugin_type == *"json" {
                        section_lines.iter().for_each(|ln| {
                            json_plugins
                                .insert(ln.to_string(), plugin_name.to_string_lossy().to_string());
                        })
                    } else if plugin_type == *"text" {
                        section_lines.iter().for_each(|ln| {
                            text_plugins
                                .insert(ln.to_string(), plugin_name.to_string_lossy().to_string());
                        })
                    }
                };
            });

        let now = Local::now();
        let time_zone_offset = now.offset();

        // TODO: Switch to this pull instead of reading the file
        // here and then parsing that was in the next
        // block of comments
        let domain = get_config_file_lines(&configuration_root, "domain.txt")
            .first()
            .unwrap()
            .to_string();

        // let mut domain_file = configuration_root.clone();
        // domain_file.push("domain.txt");
        // let domain = fs::read_to_string(domain_file)
        //     .unwrap()
        //     .as_str()
        //     .trim()
        //     .to_string();

        let span_file_paths = get_file_paths_for_extension(&folders.theme_spans_root, "jinja");
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

        let span_file_paths = get_file_paths_for_extension(&folders.theme_spans_root, "jinja");
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

        // TODO: Remove this when the lines above are working
        // to load input_date_formats
        // let mut date_formats_file = configuration_root.clone();
        // date_formats_file.push("input-date-formats.txt");
        // let input_date_formats: Vec<String> = fs::read_to_string(date_formats_file)
        //     .unwrap()
        //     .as_str()
        //     .lines()
        //     .filter_map(|l| match l {
        //         "" => None,
        //         _ => Some(l.trim().to_string()),
        //     })
        //     .collect();

        let section_attribute_excludes =
            get_config_file_lines(&configuration_root, "section-attribute-excludes.txt");

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
            default_language,
            domain: domain.parse().unwrap(),
            folders,
            input_date_formats,
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

#[cfg(test)]
mod test {
    // NOTE: This is basically all file system
    // stuff. TODO is to add better error
    // messages for missing files
}
