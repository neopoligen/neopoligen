use crate::config::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

impl Config {
    pub fn mock_basic_config() -> Config {
        let site_project_root = PathBuf::from("some-project-root");
        let theme_name = String::from("dev-theme");

        let site_configuration_root = PathBuf::from(format!(
            "{}/{}",
            site_project_root.display(),
            "configuration"
        ));

        let site_production_content_root =
            PathBuf::from(format!("{}/{}", site_project_root.display(), "pages"));
        let site_extras_root =
            PathBuf::from(format!("{}/{}", site_project_root.display(), "extras"));
        let site_images_root =
            PathBuf::from(format!("{}/{}", site_project_root.display(), "images"));
        let site_plugins_root = PathBuf::from(format!(
            "{}/{}",
            site_configuration_root.display(),
            "plugins"
        ));
        let site_output_root = PathBuf::from(format!("{}/{}", site_project_root.display(), "docs"));
        let site_themes_root =
            PathBuf::from(format!("{}/{}", site_project_root.display(), "themes"));
        let theme_root = PathBuf::from(format!("{}/{}", site_themes_root.display(), theme_name));
        let theme_configuration_root =
            PathBuf::from(format!("{}/{}", theme_root.display(), "configuration"));
        let theme_assets_root = PathBuf::from(format!("{}/{}", theme_root.display(), "assets"));

        let theme_sections_root = PathBuf::from(format!("{}/{}", theme_root.display(), "sections"));

        let theme_helpers_root = PathBuf::from(format!("{}/{}", theme_root.display(), "helpers"));
        let theme_includes_root = PathBuf::from(format!("{}/{}", theme_root.display(), "includes"));
        let theme_page_types_root =
            PathBuf::from(format!("{}/{}", theme_root.display(), "page_types"));
        let theme_spans_root = PathBuf::from(format!("{}/{}", theme_root.display(), "spans"));
        let theme_tests_root =
            PathBuf::from(format!("{}/{}/{}", theme_root.display(), "tests", "sites"));
        let theme_wrappers_root = PathBuf::from(format!("{}/{}", theme_root.display(), "wrappers"));

        let folders = ConfigFolders {
            site_configuration_root: site_configuration_root.clone(),
            site_extras_root,
            site_images_root,
            site_output_root,
            site_plugins_root,
            site_production_content_root,
            site_project_root,
            site_themes_root,
            theme_assets_root,
            theme_configuration_root: theme_configuration_root.clone(),
            theme_sections_root,
            theme_helpers_root,
            theme_includes_root,
            theme_page_types_root,
            theme_spans_root,
            theme_tests_root,
            theme_root,
            theme_wrappers_root,
        };

        let mut checklist = BTreeSet::new();
        checklist.insert("todo".to_string());

        let mut comment = BTreeSet::new();
        comment.insert("comment".to_string());

        let detail = BTreeSet::new();

        let mut json = BTreeSet::new();
        json.insert("metadata".to_string());

        let json_plugin = BTreeSet::new();

        let mut list = BTreeSet::new();
        list.insert("notes".to_string());
        list.insert("list".to_string());

        let mut preformatted = BTreeSet::new();
        preformatted.insert("code".to_string());

        let mut standard = BTreeSet::new();
        standard.insert("bookmark".to_string());
        standard.insert("div".to_string());
        standard.insert("p".to_string());
        standard.insert("title".to_string());
        standard.insert("tldr".to_string());

        let mut table = BTreeSet::new();
        table.insert("table".to_string());

        let mut text_plugin = BTreeSet::new();
        text_plugin.insert("random-color-square".to_string());

        let mut yaml = BTreeSet::new();
        yaml.insert("yaml-example".to_string());

        let section_categories = ConfigSectionCategories {
            checklist,
            comment,
            detail,
            json,
            json_plugin,
            list,
            preformatted,
            standard,
            table,
            text_plugin,
            yaml,
        };

        let input_date_formats: Vec<String> = vec![
            "%Y-%m-%d %H:%M:%S".to_string(),
            "%Y-%m-%dT%H:%M:%S".to_string(),
        ];

        let json_plugins: BTreeMap<String, String> = BTreeMap::new();

        let key_value_spans: Vec<String> = vec!["class".to_string(), "ilink".to_string()];

        let standard_spans: Vec<String> = vec!["strong".to_string(), "link".to_string()];

        let mut main_body_section_excludes: BTreeSet<String> = BTreeSet::new();
        main_body_section_excludes.insert("comment".to_string());
        main_body_section_excludes.insert("metadata".to_string());
        main_body_section_excludes.insert("title".to_string());

        let text_plugins: BTreeMap<String, String> = BTreeMap::new();

        let time_zone_offset = -5;

        let domain = "localhost".to_string();

        let section_attribute_excludes = vec![];

        let default_language = String::from("en");

        Config {
            default_language,
            domain,
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
            time_zone_offset,
        }
    }
}