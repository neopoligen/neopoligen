use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct ConfigFolders {
    pub site_configuration_root: PathBuf,
    pub site_extras_root: PathBuf,
    pub site_images_root: PathBuf,
    pub site_output_root: PathBuf,
    pub site_plugins_root: PathBuf,
    pub site_production_content_root: PathBuf,
    pub site_project_root: PathBuf,
    pub site_themes_root: PathBuf,
    pub theme_assets_root: PathBuf,
    pub theme_configuration_root: PathBuf,
    pub theme_sections_root: PathBuf,
    pub theme_helpers_root: PathBuf,
    pub theme_includes_root: PathBuf,
    pub theme_page_types_root: PathBuf,
    pub theme_spans_root: PathBuf,
    pub theme_tests_root: PathBuf,
    pub theme_root: PathBuf,
    pub theme_wrappers_root: PathBuf,
}
