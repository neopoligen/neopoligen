use crate::site_sections::SiteSections;
use dirs::document_dir;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
// use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SiteConfigV2 {
    #[serde(default = "default_language")]
    pub default_language: String,

    #[serde(default = "empty_folders")]
    pub folders: BTreeMap<String, PathBuf>,

    #[serde(default = "empty_sections")]
    pub sections: BTreeMap<String, Vec<String>>,
}

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
    pub status_root: PathBuf,
}

impl SiteConfig {
    pub fn new(site_name: String) -> SiteConfig {
        let mut project_root = document_dir().clone().unwrap();
        project_root.push("Neopoligen");
        project_root.push(site_name.as_str());
        let mut site_config_json = project_root.clone();
        site_config_json.push("config.json");

        // TODO: add grace error handling if the config can't
        // be read.
        let main_config = load_config_file(site_config_json).unwrap();

        let folders = SiteFolders {
            content_root: get_folder_path(&project_root, "content"),
            error_root: get_folder_path(&project_root, "status/errors"),
            output_root: get_folder_path(&project_root, "docs"),
            project_root: project_root.clone(),
            status_root: get_folder_path(&project_root, "status"),
            themes_root: get_folder_path(&project_root, "themes"),
        };
        SiteConfig {
            folders,
            sections: SiteSections {
                basic: vec![
                    "title".to_string(),
                    "p".to_string(),
                    "div".to_string(),
                    "note".to_string(),
                ],
                checklist: vec!["checklist".to_string()],
                json: vec!["metadata".to_string()],
                list: vec!["list".to_string()],
                raw: vec!["code".to_string(), "html".to_string()],
            },
        }
    }
}

fn get_folder_path(project_root: &PathBuf, name: &str) -> PathBuf {
    let mut new_dir = project_root.clone();
    new_dir.push(name);
    new_dir
}

// fn load_config_file(path: PathBuf) -> Result<Value, String> {
//     match path.try_exists() {
//         Ok(exists) => {
//             if exists == true {
//                 let text = fs::read_to_string(&path).unwrap();
//                 match serde_json::from_str::<Value>(text.as_str()) {
//                     Ok(data) => Ok(data),
//                     Err(_) => Err(format!("Could not parse JSON file: {}", &path.display())),
//                 }
//             } else {
//                 Err(format!("Could not read JSON file: {}", &path.display()))
//             }
//         }
//         Err(_) => Err(format!("Could not read JSON file: {}", &path.display())),
//     }
// }

fn load_config_file(path: PathBuf) -> Result<SiteConfigV2, String> {
    match path.try_exists() {
        Ok(exists) => {
            if exists == true {
                match fs::read_to_string(&path) {
                    Ok(text) => match serde_json::from_str::<SiteConfigV2>(text.as_str()) {
                        Ok(data) => Ok(data),
                        Err(e) => Err(format!(
                            "Could not parse JSON file: {}\n{}",
                            &path.display(),
                            e
                        )),
                    },
                    Err(e) => Err(format!(
                        "Could not read JSON file: {}\n{}",
                        &path.display(),
                        e
                    )),
                }
            } else {
                Err(format!("Could not read JSON file: {}", &path.display()))
            }
        }
        Err(_) => Err(format!("No file at: {}", &path.display())),
    }
}

fn default_language() -> String {
    "en".to_string()
}

fn empty_folders() -> BTreeMap<String, PathBuf> {
    BTreeMap::new()
}

fn empty_sections() -> BTreeMap<String, Vec<String>> {
    BTreeMap::new()
}
