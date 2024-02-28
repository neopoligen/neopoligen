pub mod mocks;
pub mod new;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

// This is used to abstract away the file system
// and provide for more robust testing. The functions
// load up via the file system or things can be added
// manually for testing.

#[derive(Debug)]
pub struct FileSet {
    pub pages: BTreeMap<PathBuf, String>,
    pub templates: BTreeMap<String, String>,
}

impl FileSet {
    pub fn load_content(&mut self, dir: &PathBuf) {
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

    pub fn load_templates(&mut self, dir: &PathBuf) {
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
