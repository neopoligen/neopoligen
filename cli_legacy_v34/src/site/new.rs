use crate::config::Config;
use crate::file_set::FileSet;
use crate::image::Image;
use crate::mp3::Mp3;
use crate::page::Page;
use crate::site::Site;
use std::collections::BTreeMap;
// use std::sync::Mutex;

impl Site {
    pub fn new(file_set: &FileSet, config: &Config) -> Site {
        let images = file_set
            .images
            .iter()
            .map(|image_source_path| Image {
                file_stem: image_source_path
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                file_name: image_source_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                raw_href: format!(
                    "/{}",
                    image_source_path
                        .strip_prefix(&config.folders.project_root)
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                ),
                source_path: image_source_path.clone(),
            })
            .collect();
        let mp3s = file_set
            .mp3s
            .iter()
            .map(|source_path| Mp3 {
                file_stem: source_path
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                file_name: source_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                raw_href: format!(
                    "/{}",
                    source_path
                        .strip_prefix(&config.folders.project_root)
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                ),
                source_path: source_path.clone(),
            })
            .collect();
        let mut sd = Site {
            // cache: Mutex::new(BTreeMap::new()),
            config: config.clone(),
            pages: BTreeMap::new(),
            invalid_pages: BTreeMap::new(),
            templates: BTreeMap::new(),
            images,
            mp3s,
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
