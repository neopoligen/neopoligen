use dirs::document_dir;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SiteConfig {
    pub folders: SiteFolders,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SiteFolders {
    pub content_root: PathBuf,
    pub output_root: PathBuf,
    pub project_root: PathBuf,
}

impl SiteConfig {
    pub fn new(site_name: String) -> SiteConfig {
        let mut project_root = document_dir().clone().unwrap();
        project_root.push("Neopoligen");
        project_root.push(site_name.as_str());
        let folders = SiteFolders {
            project_root: project_root.clone(),
            content_root: get_folder_path(&project_root, "content"),
            output_root: get_folder_path(&project_root, "docs"),
        };
        SiteConfig { folders }
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
    default_language: String,
    theme: String,
    date_formats: Vec<String>,
    domain: Option<String>,
}
