use crate::config::*;
use std::collections::BTreeSet;
use std::path::PathBuf;

impl Config {
    pub fn nav_items1() -> Config {
        let json_config = JsonConfig::stub1();
        let project_root = PathBuf::from("leading-dir/Neopoligen/nav-items1-test-site");
        let theme_name = String::from("nav-items1-theme");
        let configuration_root =
            PathBuf::from(format!("{}/{}", project_root.display(), "configuration"));
        let content_root = PathBuf::from(format!("{}/{}", project_root.display(), "content"));
        let build_root =
            PathBuf::from(format!("{}/{}", project_root.display(), "NOT_USED_IN_TEST"));
        let files_root =
            PathBuf::from(format!("{}/{}", project_root.display(), "NOT_USED_IN_TEST"));
        let images_root =
            PathBuf::from(format!("{}/{}", project_root.display(), "NOT_USED_IN_TEST"));
        let mp3s_root = PathBuf::from(format!("{}/{}", project_root.display(), "NOT_USED_IN_TEST"));
        let plugins_root = PathBuf::from(format!(
            "{}/{}",
            configuration_root.display(),
            "NOT_USED_IN_TEST"
        ));
        let output_root = PathBuf::from(format!("{}/{}", project_root.display(), "docs"));
        let status_root = PathBuf::from(format!("{}/{}", project_root.display(), "docs"));

        // let parsing_errors_root = PathBuf::from(format!(
        //     "{}/_errors/{}",
        //     project_root.display(),
        //     "NOT_USED_IN_TEST"
        // ));
        // let theme_errors_root = PathBuf::from(format!(
        //     "{}/_errors/{}",
        //     project_root.display(),
        //     "NOT_USED_IN_TEST"
        // ));

        let themes_root =
            PathBuf::from(format!("{}/{}", project_root.display(), "NOT_USED_IN_TEST"));
        let theme_root = PathBuf::from(format!("{}/{}", themes_root.display(), theme_name));
        let theme_configuration_root =
            PathBuf::from(format!("{}/{}", theme_root.display(), "NOT_USED_IN_TEST"));
        let theme_assets_input_root =
            PathBuf::from(format!("{}/{}", theme_root.display(), "NOT_USED_IN_TEST"));
        let theme_assets_build_root =
            PathBuf::from(format!("{}/{}", output_root.display(), "NOT_USED_IN_TEST"));
        let theme_sections_root =
            PathBuf::from(format!("{}/{}", theme_root.display(), "NOT_USED_IN_TEST"));
        let theme_spans_root =
            PathBuf::from(format!("{}/{}", theme_root.display(), "NOT_USED_IN_TEST"));
        let theme_tests_root = PathBuf::from(format!(
            "{}/{}/{}",
            theme_root.display(),
            "tests",
            "NOT_USED_IN_TEST"
        ));

        let folders = ConfigFolders {
            build_root,
            configuration_root: configuration_root.clone(),
            files_root,
            images_root,
            mp3s_root,
            output_root,
            plugins_root,
            content_root,
            project_root,
            status_root,
            themes_root,
            theme_assets_input_root,
            theme_assets_build_root,
            theme_configuration_root: theme_configuration_root.clone(),
            theme_sections_root,
            theme_spans_root,
            theme_tests_root,
            theme_root,
        };

        let mut checklist = BTreeSet::new();
        checklist.insert("todo".to_string());

        let mut comment = BTreeSet::new();
        comment.insert("comment".to_string());
        comment.insert("tags".to_string());

        let detail = BTreeSet::new();

        let mut json = BTreeSet::new();
        json.insert("metadata".to_string());

        let json_plugin = BTreeSet::new();

        let mut list = BTreeSet::new();
        list.insert("notes".to_string());
        list.insert("list".to_string());

        let mut raw = BTreeSet::new();
        raw.insert("code".to_string());
        raw.insert("css".to_string());
        raw.insert("head".to_string());
        raw.insert("script".to_string());

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
            raw,
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

        let standard_spans: Vec<String> =
            vec!["strong".to_string(), "link".to_string(), "em".to_string()];

        let main_body_section_excludes: Vec<String> = vec![
            "metadata".to_string(),
            "title".to_string(),
            "forward".to_string(),
        ];

        let text_plugins: BTreeMap<String, String> = BTreeMap::new();

        let time_zone_offset = "-5:00".to_string();

        let domain = "localhost".to_string();

        let section_attribute_excludes = vec![];

        let default_language = String::from("en");

        Config {
            default_language,
            domain,
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
            time_zone_offset,
        }
    }
}
