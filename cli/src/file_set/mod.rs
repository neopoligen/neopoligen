use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

pub struct FileSet {
    pages: BTreeMap<PathBuf, String>,
}

impl FileSet {
    pub fn load_pages(&mut self, dir: &PathBuf) {
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

    pub fn new() -> FileSet {
        FileSet {
            pages: BTreeMap::new(),
        }
    }
}
