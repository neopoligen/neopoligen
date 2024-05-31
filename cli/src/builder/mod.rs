#![allow(unused_imports)]

pub mod mocks;
pub mod object;

use crate::build_issue::*;
use crate::feed::Feed;
use crate::helpers::*;
use crate::image::Image;
use crate::og_image::*;
use crate::page_v39::PageV39;
use crate::site_config::SiteConfig;
use crate::site_v2::SiteMp3;
use crate::site_v2::SiteV2;
use anyhow::Result;
use image::DynamicImage;
use minijinja::context;
use minijinja::syntax::SyntaxConfig;
use minijinja::value::Value;
use minijinja::Environment;
use regex::Regex;
use rimage::config::{Codec, EncoderConfig};
use rimage::image::imageops::FilterType;
use rimage::Decoder;
use rimage::Encoder;
use rusqlite::Connection;
use serde_json;
use std::collections::BTreeMap;
use std::fs::File;
use std::{fs, path::PathBuf};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Builder {
    pub pages: BTreeMap<PathBuf, PageV39>,
    pub config: SiteConfig,
    pub issues: Vec<BuildIssue>,
    pub feeds: BTreeMap<String, Feed>,
    pub images: BTreeMap<PathBuf, Image>,
    pub last_edit: Option<String>,
    pub mp3s: BTreeMap<String, SiteMp3>,
}

impl Builder {
    pub fn new(config: SiteConfig) -> Result<Builder> {
        Ok(Builder {
            config,
            issues: vec![],
            feeds: BTreeMap::new(),
            images: BTreeMap::new(),
            last_edit: None,
            mp3s: BTreeMap::new(),
            pages: BTreeMap::new(),
        })
    }
}

impl Builder {
    #[instrument(skip(self))]
    pub fn generate_missing_asts(&mut self) -> Result<()> {
        event!(Level::INFO, "Generating ASTs");
        for (_, page) in self.pages.iter_mut() {
            if let None = page.ast {
                page.generate_ast()?;
            }
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_page_content(&mut self) -> Result<()> {
        event!(Level::INFO, "Generating Page Content");
        for (_, page) in self.pages.iter_mut() {
            dbg!(&page);
            ()
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_source_files(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Source Files");
        // Reminder: clear pages to remove the template tests
        self.pages = BTreeMap::new();
        let dir = &self.config.content_dir();
        WalkDir::new(dir)
            .into_iter()
            .filter(|entry| match entry.as_ref().unwrap().path().extension() {
                Some(ext) => ext.to_str().unwrap() == "neo",
                None => false,
            })
            .for_each(|entry| {
                let source_path = entry.as_ref().unwrap().path().to_path_buf();
                match fs::read_to_string(&source_path) {
                    Ok(content) => {
                        match PageV39::new_from_fs(
                            source_path.clone(),
                            self.config.clone(),
                            content,
                        ) {
                            Ok(page) => {
                                self.pages.insert(source_path, page);
                                ()
                            }
                            Err(e) => {
                                self.issues.push(BuildIssue {
                                    source_path: Some(source_path.clone()),
                                    kind: BuildIssueKind::Generic {},
                                    details: Some(e.to_string()),
                                });
                            }
                        }
                    }
                    Err(e) => {
                        event!(Level::ERROR, "{}", e)
                    }
                }
            });
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn prep_dirs(&self) -> Result<()> {
        event!(Level::INFO, "Making Sure Directories Exist");
        let _ = fs::create_dir_all(self.config.custom_og_images_dir());
        let _ = fs::create_dir_all(self.config.feeds_dest_dir());
        let _ = fs::create_dir_all(self.config.image_cache_dir());
        let _ = fs::create_dir_all(self.config.image_dest_dir());
        let _ = fs::create_dir_all(self.config.mp3_dest_dir());
        Ok(())
    }

    // pub fn copy_image_cache_to_prod(&self) -> Result<()> {
    //     for entry in WalkDir::new(self.config.image_cache_dir()) {
    //         let cache_path = entry?.into_path();
    //         let dest_path = self.config.image_dest_dir().join(
    //             cache_path
    //                 .strip_prefix(self.config.image_cache_dir())
    //                 .unwrap(),
    //         );
    //         if cache_path.is_dir() {
    //             fs::create_dir_all(dest_path)?;
    //         } else {
    //             // Reminder: don't fs::copy because of notify loop bug
    //             let data = std::fs::read(&cache_path)?;
    //             std::fs::write(&dest_path, &data)?;
    //         }
    //     }
    //     Ok(())
    // }

    // pub fn debug_flush_cache(&self) -> Result<()> {
    //     // this is a temporary thing to flush the cache until
    //     // another way to flush it is set up
    //     let conn = Connection::open(self.config.cache_db_path())?;
    //     conn.execute("DROP TABLE IF EXISTS page_archive", ())?;
    //     Ok(())
    // }

    // pub fn copy_theme_assets(&self) -> Result<(), std::io::Error> {
    //     let source_dir = self.config.theme_dir().join(PathBuf::from("files"));
    //     let dest_dir = self.config.output_dir().join(PathBuf::from("theme"));
    //     for entry in WalkDir::new(&source_dir) {
    //         let source_path = entry?.into_path();
    //         let dest_path = dest_dir.join(source_path.strip_prefix(&source_dir).unwrap());
    //         if source_path.is_dir() {
    //             fs::create_dir_all(dest_path)?;
    //         } else {
    //             let data = std::fs::read(source_path)?;
    //             std::fs::write(dest_path, &data)?;
    //         }
    //     }
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn generate_missing_asts(&mut self) -> Result<()> {
    //     self.pages.iter_mut().for_each(|p| {
    //         if p.1.ast.len() == 0 {
    //             p.1.generate_ast(&self.config);
    //         }
    //     });
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn generate_page_content_and_feeds(&mut self) -> Result<()> {
    //     event!(Level::INFO, "Generating Page Content");
    //     let site_obj = Value::from_object(SiteV2::new(
    //         &self.config,
    //         &self.pages,
    //         &self.images,
    //         &self.mp3s,
    //     ));
    //     let mut env = Environment::new();
    //     env.set_debug(true);
    //     env.set_lstrip_blocks(true);
    //     env.set_trim_blocks(true);
    //     env.add_function("highlight_code", highlight_code);
    //     env.add_function("image_path", image_path);
    //     env.set_syntax(
    //         SyntaxConfig::builder()
    //             .block_delimiters("[!", "!]")
    //             .variable_delimiters("[@", "@]")
    //             .comment_delimiters("[#", "#]")
    //             .build()
    //             .unwrap(),
    //     );
    //     WalkDir::new(self.config.templates_dir())
    //         .into_iter()
    //         .filter(|entry| match entry.as_ref().unwrap().path().extension() {
    //             Some(ext) => ext.to_str().unwrap() == "neoj",
    //             None => false,
    //         })
    //         .for_each(|entry| {
    //             let path = entry.as_ref().unwrap().path().to_path_buf();
    //             match fs::read_to_string(&path) {
    //                 Ok(content) => {
    //                     let template_name = path.strip_prefix(self.config.templates_dir()).unwrap();
    //                     let _ = env.add_template_owned(
    //                         template_name.to_string_lossy().to_string(),
    //                         content,
    //                     );
    //                 }
    //                 Err(e) => {
    //                     event!(Level::ERROR, "{}", e)
    //                 }
    //             };
    //         });

    // self.pages.iter_mut().for_each(|p| {
    //     if let Ok(id) = p.1.id_v2() {
    //         match p.1.output {
    //             Some(_) => {}
    //             None => {
    //                 let template_name = "pages/post/published.neoj";
    //                 if let Ok(tmpl) = env.get_template(template_name) {
    //                     match tmpl.render(context!(
    //                         site => site_obj,
    //                         page_id => id,
    //                         page => Value::from_object(p.1.clone())
    //                     )) {
    //                         Ok(output) => {
    //                             self.last_edit = Some(output.clone());
    //                             p.1.output = Some(output);
    //                         }
    //                         Err(e) => {
    //                             // TODO: Provide error handling here
    //                             event!(Level::ERROR, "{}", e);
    //                             p.1.output = None;
    //                         }
    //                     }
    //                 } else {
    //                     // TODO: Provide error handling here
    //                     event!(Level::ERROR, "Could not get template: {}", template_name);
    //                 }
    //             }
    //         }
    //     } else {
    //         self.issues.push(BuildIssue {
    //             kind: BuildIssueKind::MissingPageId {},
    //             details: None,
    //             source_path: Some(p.0.to_path_buf()),
    //         })
    //     }
    // });
    // // Feeds
    // WalkDir::new(self.config.feeds_source_dir())
    //     .into_iter()
    //     .filter(|entry| match entry.as_ref().unwrap().path().extension() {
    //         Some(ext) => ext.to_str().unwrap() == "neoj",
    //         None => false,
    //     })
    //     .for_each(|entry| {
    //         let path = entry.as_ref().unwrap().path().to_path_buf();
    //         if let Some(stem) = path.file_stem() {
    //             let stem = stem.to_string_lossy().to_string();
    //             let template_name = format!("feeds/{}.neoj", stem);
    //             if let Ok(tmpl) = env.get_template(&template_name) {
    //                 match tmpl.render(context!(
    //                     site => site_obj,
    //                 )) {
    //                     Ok(output) => {
    //                         self.feeds.insert(
    //                             stem,
    //                             Feed {
    //                                 content: Some(output),
    //                             },
    //                         );
    //                     }
    //                     Err(e) => {
    //                         // TODO: Provide error handling here
    //                         event!(Level::ERROR, "{}", e);
    //                     }
    //                 }
    //             } else {
    //                 // TODO: Provide error handling here
    //                 event!(Level::ERROR, "Could not get template: {}", template_name);
    //             }
    //         }
    //     });
    // Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn load_cached_images(&mut self) -> Result<()> {
    //     event!(Level::INFO, "Loading Cached Images");
    //     let conn = Connection::open(self.config.cache_db_path())?;
    //     conn.execute(
    //         "CREATE TABLE IF NOT EXISTS image_cache (path TEXT, data TEXT)",
    //         (),
    //     )?;
    //     let mut stmt = conn.prepare("SELECT path, data FROM image_cache")?;
    //     let mut rows = stmt.query([])?;
    //     while let Some(row) = rows.next()? {
    //         let path_string: String = row.get(0)?;
    //         if let Some(_) = self.images.get(&PathBuf::from(&path_string)) {
    //             let path = PathBuf::from(path_string);
    //             let img: String = row.get(1)?;
    //             let i: Image = serde_json::from_str(&img.to_string())?;
    //             self.images.insert(path, i);
    //         }
    //     }
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn load_cached_pages(&mut self) -> Result<()> {
    //     event!(Level::INFO, "Loading Cached Files");
    //     let conn = Connection::open(self.config.cache_db_path())?;
    //     conn.execute(
    //         "CREATE TABLE IF NOT EXISTS page_archive (path TEXT, page TEXT)",
    //         (),
    //     )?;
    //     let mut stmt = conn.prepare("SELECT path, page FROM page_archive")?;
    //     let mut rows = stmt.query([])?;
    //     while let Some(row) = rows.next()? {
    //         let path_string: String = row.get(0)?;
    //         let path = PathBuf::from(path_string);
    //         let page: String = row.get(1)?;
    //         let p: PageV2 = serde_json::from_str(&page.to_string())?;
    //         self.pages.insert(path, p);
    //     }
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn load_mp3s(&mut self) -> Result<()> {
    //     // NOTE: Right now this just copies the files into
    //     // their proper place.
    //     event!(Level::INFO, "Loading Mp3s");
    //     for entry in WalkDir::new(self.config.mp3_source_dir()) {
    //         let source_path = entry?.into_path();
    //         if let Some(ext) = source_path.extension() {
    //             if ext.to_ascii_lowercase() == "mp3" {
    //                 if let Some(stem) = source_path.file_stem() {
    //                     if let Ok(key) = clean_for_url(&stem.to_string_lossy().to_string()) {
    //                         let dest_mp3_path =
    //                             self.config.mp3_dest_dir().join(format!("{}.mp3", key));
    //                         let _ = safe_copy_file(&source_path, &dest_mp3_path);
    //                         let _ = &self.mp3s.insert(
    //                             key.to_string(),
    //                             SiteMp3 {
    //                                 extension: "mp3".to_string(),
    //                                 key: key.clone(),
    //                             },
    //                         );
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn load_source_images(&mut self) -> Result<()> {
    //     event!(Level::INFO, "Loading Images");
    //     for entry in WalkDir::new(self.config.image_source_dir()) {
    //         let source_path = entry?.into_path();
    //         if let Some(_) = source_path.extension() {
    //             self.images.insert(
    //                 source_path.clone(),
    //                 Image {
    //                     alt_text: None,
    //                     alt_text_extended: None,
    //                     height: None,
    //                     source_path,
    //                     versions: vec![],
    //                     width: None,
    //                 },
    //             );
    //         }
    //     }
    //     Ok(())
    // }

    // DEPRECATED: This is from v38
    // #[instrument(skip(self))]
    // pub fn load_source_files(&mut self) -> Result<()> {
    //     event!(Level::INFO, "Loading Source Files");
    //     // Reminder: clear pages to remove the template tests
    //     self.pages = BTreeMap::new();
    //     let dir = &self.config.content_dir();
    //     WalkDir::new(dir)
    //         .into_iter()
    //         .filter(|entry| match entry.as_ref().unwrap().path().extension() {
    //             Some(ext) => ext.to_str().unwrap() == "neo",
    //             None => false,
    //         })
    //         .for_each(|entry| {
    //             let path = entry.as_ref().unwrap().path().to_path_buf();
    //             match fs::read_to_string(&path) {
    //                 Ok(content) => {
    //                     let page =
    //                         PageV2::new_from_filesystem(path.clone(), self.config.clone(), content);
    //                     match self.pages.get(&path.clone()) {
    //                         Some(cache_page) => {
    //                             if cache_page.hash() != page.hash() {
    //                                 self.pages.insert(path.clone(), page);
    //                             }
    //                         }
    //                         None => {
    //                             self.pages.insert(path.clone(), page);
    //                         }
    //                     }
    //                 }
    //                 Err(e) => {
    //                     event!(Level::ERROR, "{}", e)
    //                 }
    //             }
    //         });
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn make_og_images(&self) -> Result<()> {
    //     event!(Level::INFO, "Making OG Images");
    //     let _ = fs::create_dir_all(self.config.tmp_dir());
    //     let _ = fs::create_dir_all(self.config.og_images_cache_dir());
    //     let _ = fs::create_dir_all(self.config.og_images_dir());
    //     let tmp_path = self.config.tmp_dir().join("og-image.png");
    //     for p in &self.pages {
    //         if let (Some(id), Some(title)) = (&p.1.id(), &p.1.title_as_plain_text()) {
    //             let mut make_image = false;
    //             let cache_path = &self
    //                 .config
    //                 .og_images_cache_dir()
    //                 .join(format!("{}.jpg", &id));
    //             let output_path = &self.config.og_images_dir().join(format!("{}.jpg", &id));
    //             if !cache_path.exists() {
    //                 make_image = true;
    //             } else {
    //                 if let (Ok(content_metadata), Ok(cache_metadata)) = (
    //                     fs::metadata(&p.1.source_path.clone().unwrap()),
    //                     fs::metadata(&cache_path),
    //                 ) {
    //                     if let (Ok(content_time), Ok(cache_time)) =
    //                         (content_metadata.modified(), cache_metadata.modified())
    //                     {
    //                         if content_time > cache_time {
    //                             make_image = true
    //                         }
    //                     }
    //                 }
    //             }
    //             if make_image {
    //                 event!(Level::INFO, "Making OG Image: {} - {}", &id, &title);
    //                 let og_image = OgImage {
    //                     text_areas: vec![
    //                         OgImageTextArea {
    //                             color: "#0481c5".to_string(),
    //                             font_family: "Arial".to_string(),
    //                             font_size: 20,
    //                             line_height: 40,
    //                             max_char_width: 20,
    //                             max_lines: 2,
    //                             text: "alanwsmith.com".to_string(),
    //                             x: 1000,
    //                             y: 600,
    //                         },
    //                         OgImageTextArea {
    //                             color: "#0481c5".to_string(),
    //                             font_family: "Arial".to_string(),
    //                             font_size: 86,
    //                             line_height: 100,
    //                             max_char_width: 18,
    //                             max_lines: 4,
    //                             text: title.to_string(),
    //                             x: 80,
    //                             y: 220,
    //                         },
    //                     ],
    //                 };
    //                 og_image.render_svg(&tmp_path);
    //                 let decoder = Decoder::from_path(&tmp_path)?;
    //                 let image = decoder.decode()?;
    //                 let config = EncoderConfig::new(Codec::MozJpeg)
    //                     .with_quality(80.0)
    //                     .unwrap();
    //                 let file = File::create(&cache_path)?;
    //                 let encoder = Encoder::new(file, DynamicImage::ImageRgba8(image.into()))
    //                     .with_config(config);
    //                 encoder.encode()?;
    //             }
    //             let _ = std::fs::copy(&cache_path, &output_path);
    //         }
    //     }
    //     let custom_og_images = get_files_with_extension_in_a_single_directory(
    //         &self.config.custom_og_images_dir(),
    //         "jpg",
    //     );
    //     for custom_image in custom_og_images {
    //         let output_path = self
    //             .config
    //             .og_images_dir()
    //             .join(custom_image.file_name().unwrap());
    //         fs::copy(&custom_image, &output_path)?;
    //     }
    //     Ok(())
    // }

    // pub fn output_content_files(&self) -> Result<()> {
    //     self.pages.iter().for_each(|p| {
    //         if let (Some(rel_file_path), Some(output)) = (p.1.rel_file_path(), p.1.output.clone()) {
    //             let output_path = self.config.output_dir().join(rel_file_path);
    //             // TODO: Add error handling here
    //             let _ = write_file_with_mkdir(&output_path, &output);
    //         }
    //     });
    //     Ok(())
    // }

    // pub fn output_feeds(&self) -> Result<()> {
    //     for (key, feed) in self.feeds.iter() {
    //         let output_path = self.config.feeds_dest_dir().join(format!("{}.xml", key));
    //         if let Some(content) = &feed.content {
    //             fs::write(output_path, content)?;
    //         }
    //     }
    //     Ok(())
    // }

    // pub fn output_last_edit(&self) -> Result<()> {
    //     let output_path = self.config.output_dir().join("last-edit/index.html");
    //     let no_last_edit_content = r#"<!DOCTYPE html>
    // <html><head><style>
    // body {
    // background-color: #333;
    // color: #aaa;
    // font-size: 4rem;
    // }
    // </style></head>
    // <body>
    // <a href="/">Home</a>
    // <p>No edits made yet in this session</p>
    // </body>
    // </html>"#
    //         .to_string();
    //     if let Some(content) = &self.last_edit {
    //         let _ = write_file_with_mkdir(&output_path, &content);
    //     } else {
    //         let _ = write_file_with_mkdir(&output_path, &no_last_edit_content);
    //     }
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn test_theme(&mut self) -> Result<()> {
    //     event!(Level::INFO, "Testing Templates");
    //     // Reminder: Clear pages so order of build and test doesn't matter.
    //     self.pages = BTreeMap::new();
    //     // Add the testing sections to the site config
    //     let mut theme_test_config = self.config.clone();
    //     theme_test_config
    //         .sections
    //         .basic
    //         .push("start-theme-test".to_string());
    //     let site_obj = Value::from_object(SiteV2::new(
    //         &theme_test_config,
    //         &self.pages,
    //         &self.images,
    //         &self.mp3s,
    //     ));
    //     let mut env = Environment::new();
    //     env.set_debug(true);
    //     env.set_lstrip_blocks(true);
    //     env.set_trim_blocks(true);
    //     env.add_function("highlight_code", highlight_code);
    //     env.add_function("image_path", image_path);
    //     env.set_syntax(
    //         SyntaxConfig::builder()
    //             .block_delimiters("[!", "!]")
    //             .variable_delimiters("[@", "@]")
    //             .comment_delimiters("[#", "#]")
    //             .build()
    //             .unwrap(),
    //     );
    //     WalkDir::new(self.config.templates_dir())
    //         .into_iter()
    //         .filter(|entry| match entry.as_ref().unwrap().path().extension() {
    //             Some(ext) => ext.to_str().unwrap() == "neoj",
    //             None => false,
    //         })
    //         .for_each(|entry| {
    //             let path = entry.as_ref().unwrap().path().to_path_buf();
    //             match fs::read_to_string(&path) {
    //                 Ok(content) => {
    //                     let template_name = path.strip_prefix(self.config.templates_dir()).unwrap();
    //                     let _ = env.add_template_owned(
    //                         template_name.to_string_lossy().to_string(),
    //                         content,
    //                     );
    //                 }
    //                 Err(e) => {
    //                     event!(Level::ERROR, "{}", e)
    //                 }
    //             };
    //         });
    //     for entry in WalkDir::new(self.config.theme_tests_dir().join("content")) {
    //         let entry = entry?.path().to_path_buf();
    //         if entry.is_file() {
    //             if let Some(ext) = entry.extension() {
    //                 if ext == "neo" {
    //                     if let Ok(content) = fs::read_to_string(&entry) {
    //                         self.pages.insert(
    //                             entry.clone(),
    //                             PageV2::new_from_filesystem(
    //                                 entry.clone(),
    //                                 theme_test_config.clone(),
    //                                 content,
    //                             ),
    //                         );
    //                         ()
    //                     } else {
    //                         self.issues.push(BuildIssue {
    //                             kind: BuildIssueKind::CouldNotReadThemeTest {},
    //                             details: None,
    //                             source_path: Some(entry),
    //                         });
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     let test_template_name = "theme-test.neoj";
    //     let _ = env.add_template_owned(
    //         test_template_name,
    //         r#"
    // [!- include "includes/config.neoj" -!]
    // [!- import "includes/theme.neoj" as theme -!]
    // [! for section in page.all_sections() !]
    // [@- theme.output_section(site, page.id(), section) -@]
    // [! endfor !]
    // "#,
    //     );
    //     let _ = env.add_template_owned(
    //         "sections/start-theme-test/full/default.neoj",
    //         "<!-- START_THEME_TEST -->\n\n[! include 'subsections/content-full.neoj' !]",
    //     );
    //     let _ = self.generate_missing_asts();
    //     for (source_path, page) in self.pages.iter() {
    //         if let Ok(tmpl) = env.get_template(test_template_name) {
    //             match tmpl.render(context!(
    //                 site => site_obj,
    //                 page => Value::from_object(page.clone())
    //             )) {
    //                 Ok(output) => {
    //                     let tests = output
    //                         .split("<!-- START_THEME_TEST -->")
    //                         .collect::<Vec<&str>>();
    //                     if tests.len() == 1 {
    //                         self.issues.push(BuildIssue {
    //                             kind: BuildIssueKind::NoThemeTestsFound {},
    //                             details: None,
    //                             source_path: Some(source_path.to_path_buf()),
    //                         })
    //                     } else {
    //                         for t in tests.iter().skip(1) {
    //                             let parts =
    //                                 t.split("<!-- EXPECTED_OUTPUT -->").collect::<Vec<&str>>();
    //                             if parts.len() == 3 {
    //                                 let left = parts[0].replace("\n", "").replace(" ", "");
    //                                 let right = parts[1].replace("\n", "").replace(" ", "");
    //                                 if left != right {
    //                                     self.issues.push(BuildIssue {
    //                                         details: None,
    //                                         source_path: Some(source_path.to_path_buf()),
    //                                         kind: BuildIssueKind::FailedThemeTest {
    //                                             expected: Some(parts[1].to_string()),
    //                                             got: Some(parts[0].to_string()),
    //                                         },
    //                                     })
    //                                 }
    //                             } else {
    //                                 self.issues.push(BuildIssue {
    //                                     details: Some(t.to_string()),
    //                                     kind: BuildIssueKind::InvalidThemeTest {},
    //                                     source_path: Some(source_path.to_path_buf()),
    //                                 })
    //                             }
    //                         }
    //                     }
    //                 }
    //                 Err(e) => self.issues.push(BuildIssue {
    //                     details: Some(e.to_string()),
    //                     kind: BuildIssueKind::CouldNotRenderThemeTest {},
    //                     source_path: Some(source_path.to_path_buf()),
    //                 }),
    //             }
    //         } else {
    //             self.issues.push(BuildIssue {
    //                 details: Some("Could not get internal test template".to_string()),
    //                 kind: BuildIssueKind::Generic {},
    //                 source_path: Some(source_path.to_path_buf()),
    //             })
    //         }
    //     }
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn update_image_cache_db(&self) -> Result<()> {
    //     event!(Level::INFO, "Updating Image Cache DB");
    //     let mut conn = Connection::open(self.config.cache_db_path())?;
    //     conn.execute("DROP TABLE IF EXISTS image_cache", ())?;
    //     conn.execute(
    //         "CREATE TABLE IF NOT EXISTS image_cache(path TEXT, data TEXT)",
    //         (),
    //     )?;
    //     let query = "INSERT INTO image_cache(path, data) VALUES (?1, ?2)";
    //     let tx = conn.transaction()?;
    //     for (source_path, image) in self.images.iter() {
    //         if let Ok(data) = serde_json::to_string(image) {
    //             tx.execute(query, (source_path.to_string_lossy().to_string(), data))?;
    //         };
    //     }
    //     tx.commit()?;
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn update_page_cache(&self) -> Result<()> {
    //     event!(Level::INFO, "Updating Page Cache");
    //     let mut conn = Connection::open(self.config.cache_db_path())?;
    //     conn.execute("DROP TABLE IF EXISTS page_archive", ())?;
    //     conn.execute(
    //         "CREATE TABLE IF NOT EXISTS page_archive (path TEXT, page TEXT)",
    //         (),
    //     )?;
    //     let query = "INSERT INTO page_archive(path, page) VALUES (?1, ?2)";
    //     let tx = conn.transaction()?;
    //     for p in self.pages.iter() {
    //         if let Ok(data) = serde_json::to_string(p.1) {
    //             tx.execute(
    //                 query,
    //                 (
    //                     p.1.source_path
    //                         .clone()
    //                         .unwrap()
    //                         .to_string_lossy()
    //                         .to_string(),
    //                     data,
    //                 ),
    //             )?;
    //         };
    //     }
    //     tx.commit()?;
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn generate_cache_images(&mut self) -> Result<()> {
    //     event!(Level::INFO, "Updating Image Cache");
    //     for (_, image) in self.images.iter_mut() {
    //         let base_dir = self.config.image_cache_dir().join(image.key()?);
    //         let raw_cache_path = base_dir.join(format!("raw.{}", image.extension()?));
    //         if image.width == None || cache_is_stale(&image.source_path, &raw_cache_path) {
    //             event!(Level::INFO, "Generating Image: {}", image.key().unwrap());
    //             let _ = image.get_alt_text();
    //             let parent_dir = raw_cache_path
    //                 .parent()
    //                 .expect("could not get image cache parent director");
    //             fs::create_dir_all(parent_dir)?;
    //             // Reminder: don't fs::copy because of notify loop bug
    //             let data = std::fs::read(&image.source_path)?;
    //             std::fs::write(&raw_cache_path, &data)?;
    //             let decoder = Decoder::from_path(&image.source_path)?;
    //             let data = decoder.decode()?;
    //             image.width = Some(data.width());
    //             image.height = Some(data.height());
    //             image.set_dimensions(self.config.image_widths())?;
    //             for version in image.versions.iter() {
    //                 let version_path =
    //                     base_dir.join(format!("{}.{}", version.0, image.extension()?));
    //                 if image.extension()? == "jpg" || image.extension()? == "jpeg" {
    //                     resize_and_optimize_jpg(&image.source_path, version.0, &version_path)?;
    //                 } else if image.extension()? == "png" {
    //                     resize_and_optimize_png(&image.source_path, version.0, &version_path)?;
    //                 } else {
    //                     event!(
    //                         Level::ERROR,
    //                         "TODO: Process other image types: {}",
    //                         &image.source_path.display()
    //                     );
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    // #[instrument(skip(self))]
    // pub fn output_issues(&self) -> Result<()> {
    //     let mut env = Environment::new();
    //     env.add_function(
    //         "format_html_for_theme_test_display",
    //         format_html_for_theme_test_display,
    //     );
    //     env.set_debug(true);
    //     env.set_lstrip_blocks(true);
    //     env.set_trim_blocks(true);
    //     env.set_syntax(
    //         SyntaxConfig::builder()
    //             .block_delimiters("[!", "!]")
    //             .variable_delimiters("[@", "@]")
    //             .comment_delimiters("[#", "#]")
    //             .build()
    //             .unwrap(),
    //     );
    //     let _ = env.add_template_owned(
    //         "error-to-do.neoj",
    //         r#"
    // <h2>Error To Handle: [@ issue.kind().kind @]</h2>
    //         "#,
    //     );
    //     let _ = env.add_template_owned(
    //         "failedthemetest.neoj",
    //         r#"
    // <h2>Theme Test Issue: [@ issue.file_name() @]</h2>
    // <h3>Expected</h3>
    // <pre>
    // [@ format_html_for_theme_test_display(issue.expected())|escape @]
    // </pre>
    // <h3>Got</h3>
    // <pre>
    // [@ format_html_for_theme_test_display(issue.got())|escape @]
    // </pre>
    // "#,
    //     );
    //     let _ = env.add_template_owned(
    //         "error",
    //         r#"
    // <!DOCTYPE html>
    // <html><head><style>
    // body { background-color: #111; color: #aaa; }
    // </style></head>
    // <body>
    // <header>
    // <a href="/">Home</a>
    // </header>
    // <main>
    // <ul>
    // [! for issue in builder.issues() !]
    // <li>
    // <div>Kind> [@ issue.kind().kind @]</div>
    // <div>
    // [! include [issue.kind().kind + ".neoj", "error-to-do.neoj"] ignore missing !]
    // </div>
    // [! endfor !]
    // </li>
    // </ul>
    // </main>
    // </body>
    // </html>
    //     "#,
    //     );
    //     if let Ok(tmpl) = env.get_template("error") {
    //         match tmpl.render(context!(
    //         builder => Value::from_object(self.clone())
    //         )) {
    //             Ok(output) => {
    //                 let status_path = self.config.status_dir().join("index.html");
    //                 let _ = fs::write(status_path, output);
    //             }
    //             Err(e) => {
    //                 event!(Level::ERROR, "Stauts report error: {}", e);
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    //
}

pub fn get_files_with_extension_in_a_single_directory(
    dir: &PathBuf,
    extension: &str,
) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .filter(|p| {
            if p.as_ref().unwrap().path().is_file() {
                true
            } else {
                false
            }
        })
        .filter(|p| match p.as_ref().unwrap().path().extension() {
            Some(ext) => ext == extension,
            None => false,
        })
        .filter_map(|p| match p.as_ref().unwrap().path().strip_prefix(".") {
            Ok(_) => None,
            Err(_) => Some(p.as_ref().unwrap().path()),
        })
        .collect()
}

// pub fn highlight_code(args: &[Value]) -> String {
//     let code = args[0].to_string();
//     let lang = args[1].to_string();
//     let syntax_set = SyntaxSet::load_defaults_newlines();
//     let syntax = syntax_set
//         .find_syntax_by_token(&lang)
//         .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
//     let mut html_generator =
//         ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
//     for line in LinesWithEndings::from(&trim_empty_lines(&code)) {
//         let _ = html_generator.parse_html_for_line_which_includes_newline(line);
//     }
//     let initial_html = html_generator.finalize();
//     let output_html: Vec<_> = initial_html
//         .lines()
//         .map(|line| format!(r#"<span class="line-marker"></span>{}"#, line))
//         .collect();
//     output_html.join("\n")
// }

// // DEPRECATED, I think. TODO: Look up where this is being used.
// pub fn image_path(_args: &[Value]) -> Option<String> {
//     Some("/images/stills/hackers/images/hackers-frame-000003471/base.jpg".to_string())
// }

// fn resize_and_optimize_jpg(source: &PathBuf, width: u32, dest: &PathBuf) -> Result<()> {
//     let decoder = Decoder::from_path(source)?;
//     let image = decoder.decode()?;
//     let height = image.height() * width / image.width();
//     let resized_image = image.resize_to_fill(width, height, FilterType::Lanczos3);
//     let config = EncoderConfig::new(Codec::MozJpeg)
//         .with_quality(90.0)
//         .unwrap();
//     let file = File::create(&dest)?;
//     let encoder =
//         Encoder::new(file, DynamicImage::ImageRgba8(resized_image.into())).with_config(config);
//     encoder.encode()?;
//     Ok(())
// }

// fn resize_and_optimize_png(source: &PathBuf, width: u32, dest: &PathBuf) -> Result<()> {
//     let decoder = Decoder::from_path(source)?;
//     let image = decoder.decode()?;
//     let height = image.height() * width / image.width();
//     let resized_image = image.resize_to_fill(width, height, FilterType::Lanczos3);
//     let config = EncoderConfig::new(Codec::OxiPng);
//     let file = File::create(&dest)?;
//     let encoder =
//         Encoder::new(file, DynamicImage::ImageRgba8(resized_image.into())).with_config(config);
//     encoder.encode()?;
//     Ok(())
// }

// fn format_html_for_theme_test_display(code: &str) -> String {
//     let mut re = Regex::new(r"\n").unwrap();
//     let output = re.replace_all(code, " ");
//     re = Regex::new(r" \s+").unwrap();
//     let output = re.replace_all(&output, " ");
//     re = Regex::new(r"\s+<").unwrap();
//     let output = re.replace_all(&output, "<");
//     re = Regex::new(r">\s+").unwrap();
//     let output = re.replace_all(&output, ">");
//     let parts: Vec<&str> = output.split("<").collect();
//     let mut assembler: Vec<String> = vec![];
//     let mut level = 0i8;
//     assembler.push(parts[0].to_string());
//     parts.iter().skip(1).for_each(|part| {
//         if part.starts_with("/") {
//             level -= 2;
//         }
//         for _ in 0..level {
//             assembler.push(" ".to_string());
//         }
//         assembler.push(format!("<{}\n", part));
//         if !part.starts_with("/") {
//             level += 2;
//         }
//     });
//     assembler.join("").to_string()
// }

// pub fn trim_empty_lines(source: &str) -> String {
//     let re = Regex::new(r"\S").unwrap();
//     let trimmed_front = source.split("\n").fold("".to_string(), |acc, l| {
//         if !acc.is_empty() {
//             acc + l + "\n"
//         } else {
//             if re.is_match(l) {
//                 l.to_string() + "\n"
//             } else {
//                 acc
//             }
//         }
//     });
//     trimmed_front.trim_end().to_string()
// }

fn write_file_with_mkdir(path: &PathBuf, content: &str) -> Result<(), String> {
    match path.parent() {
        Some(parent_dir) => match fs::create_dir_all(parent_dir) {
            Ok(_) => match fs::write(path, content) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        },
        None => Err("Could not make directory".to_string()),
    }
}
