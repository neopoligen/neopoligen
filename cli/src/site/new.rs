use crate::config::Config;
use crate::file_set::FileSet;
use crate::page::Page;
use crate::site::Site;
use std::collections::BTreeMap;
use std::sync::Mutex;

impl Site {
    pub fn new(file_set: &FileSet, config: &Config) -> Site {
        let mut sd = Site {
            cache: Mutex::new(BTreeMap::new()),
            config: config.clone(),
            pages: BTreeMap::new(),
            invalid_pages: BTreeMap::new(),
            templates: BTreeMap::new(),
        };
        file_set.pages.iter().for_each(|f| {
            match Page::new(f.0.to_path_buf(), f.1.to_string(), &config) {
                Some(page) => {
                    sd.pages.insert(page.id.clone(), page);
                    ()
                }
                None => (),
            }
        });
        sd.templates = file_set.templates.clone();
        sd
    }
}
