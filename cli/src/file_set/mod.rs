pub mod mocks;
pub mod new;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

// This is used to abstract away the file system
// and provide for more robust testing. The functions
// load up via the file system or things can be added
// manually for testing.

#[derive(Debug)]
pub struct FileSet {
    pub pages: BTreeMap<PathBuf, String>,
    pub templates: BTreeMap<String, String>,
    pub images: Vec<PathBuf>,
    pub mp3s: Vec<PathBuf>,
}

impl FileSet {
    pub fn load_content(&mut self, dir: &PathBuf) {
        if dir.exists() {
            WalkDir::new(dir)
                .into_iter()
                .filter(|entry| match entry.as_ref().unwrap().path().extension() {
                    Some(ext) => ext.to_str().unwrap() == "neo",
                    None => false,
                })
                .for_each(|entry| {
                    let path = entry.as_ref().unwrap().path().to_path_buf();
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            self.pages.insert(path, content);
                            ()
                        }
                        Err(e) => {
                            println!("{}", e);
                            ()
                        }
                    }
                });
        }
    }

    pub fn load_images(&mut self, dir: &PathBuf) {
        if dir.exists() {
            let walker = WalkDir::new(dir).into_iter();
            for entry in walker.filter_entry(|e| !is_hidden(e)) {
                self.images.push(entry.unwrap().path().to_path_buf());
            }
        }
    }

    pub fn load_mp3s(&mut self, dir: &PathBuf) {
        if dir.exists() {
            let walker = WalkDir::new(dir).into_iter();
            for entry in walker.filter_entry(|e| !is_hidden(e)) {
                self.mp3s.push(entry.unwrap().path().to_path_buf());
            }
        }
    }

    pub fn load_templates(&mut self, dir: &PathBuf) {
        if dir.exists() {
            WalkDir::new(dir)
                .into_iter()
                .filter(|entry| match entry.as_ref().unwrap().path().extension() {
                    Some(ext) => ext.to_str().unwrap() == "jinja",
                    None => false,
                })
                .for_each(|entry| {
                    let path = entry.as_ref().unwrap().path().to_path_buf();
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            let template_name = path.strip_prefix(dir.to_str().unwrap()).unwrap();
                            self.templates
                                .insert(template_name.to_string_lossy().to_string(), content);
                            ()
                        }
                        Err(e) => {
                            println!("{}", e);
                            ()
                        }
                    }
                });
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
