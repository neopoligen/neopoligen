use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub struct ConfigFolders {
    pub build_root: PathBuf,
    // Deprecated: TODO: remove configuration_root
    pub content_root: PathBuf,
    pub files_root: PathBuf,
    pub images_root: PathBuf,
    pub mp3s_root: PathBuf,
    pub output_root: PathBuf,
    // pub parsing_errors_root: PathBuf,
    pub plugins_root: PathBuf,
    pub project_root: PathBuf,
    pub status_root: PathBuf,
    pub themes_root: PathBuf,
    pub theme_assets_input_root: PathBuf,
    pub theme_assets_build_root: PathBuf,
    pub theme_configuration_root: PathBuf,
    pub theme_sections_root: PathBuf,
    //pub theme_errors_root: PathBuf,
    // pub theme_helpers_root: PathBuf,
    //     pub theme_includes_root: PathBuf,
    //  pub theme_page_types_root: PathBuf,
    pub theme_spans_root: PathBuf,
    pub theme_tests_root: PathBuf,
    pub theme_root: PathBuf,
    // pub theme_wrappers_root: PathBuf,
}
