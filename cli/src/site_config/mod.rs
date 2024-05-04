use crate::site_sections::SiteSections;
use dirs::document_dir;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SiteConfig {
    pub folders: SiteFolders,
    pub sections: SiteSections,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SiteFolders {
    pub content_root: PathBuf,
    pub output_root: PathBuf,
    pub project_root: PathBuf,
    pub themes_root: PathBuf,
    pub error_root: PathBuf,
}

impl SiteConfig {
    pub fn new(site_name: String) -> SiteConfig {
        let mut project_root = document_dir().clone().unwrap();
        project_root.push("Neopoligen");
        project_root.push(site_name.as_str());
        let folders = SiteFolders {
            project_root: project_root.clone(),
            content_root: get_folder_path(&project_root, "content"),
            error_root: get_folder_path(&project_root, "errors"),
            output_root: get_folder_path(&project_root, "docs"),
            themes_root: get_folder_path(&project_root, "themes"),
        };
        SiteConfig {
            folders,
            sections: SiteSections {
                basic: vec!["p".to_string(), "div".to_string(), "note".to_string()],
                checklist: vec!["checklist".to_string()],
                json: vec!["metadata".to_string()],
                list: vec!["list".to_string()],
                raw: vec!["code".to_string()],
            },
        }
    }
}

fn get_folder_path(project_root: &PathBuf, name: &str) -> PathBuf {
    let mut new_dir = project_root.clone();
    new_dir.push(name);
    new_dir
}

// this is what comes out of the config file.
// it's its own thing to provide straight access
// to errors from serde
pub struct BaseConfig {
    // default_language: String,
    // theme: String,
    // date_formats: Vec<String>,
    // domain: Option<String>,
}
