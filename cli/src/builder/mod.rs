use crate::og_image::*;
use crate::page_v2::PageV2;
use crate::site_config::SiteConfig;
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
    pub pages: BTreeMap<PathBuf, PageV2>,
    pub config: SiteConfig,
    pub last_edit: Option<String>,
}

impl Builder {
    pub fn new(config: SiteConfig) -> Result<Builder> {
        Ok(Builder {
            pages: BTreeMap::new(),
            config,
            last_edit: None,
        })
    }
}

impl Builder {
    pub fn debug_flush_cache(&self) -> Result<()> {
        // this is a temporary thing to flush the cache until
        // another way to flush it is set up
        let conn = Connection::open(self.config.cache_db_path())?;
        conn.execute("DROP TABLE IF EXISTS page_archive", ())?;
        Ok(())
    }

    pub fn copy_theme_assets(&self) -> Result<(), std::io::Error> {
        let source_dir = self.config.theme_dir().join(PathBuf::from("files"));
        let dest_dir = self.config.output_dir().join(PathBuf::from("theme"));
        for entry in WalkDir::new(&source_dir) {
            let source_path = entry?.into_path();
            let dest_path = dest_dir.join(source_path.strip_prefix(&source_dir).unwrap());
            if source_path.is_dir() {
                fs::create_dir_all(dest_path)?;
            } else {
                let data = std::fs::read(source_path)?;
                std::fs::write(dest_path, &data)?;
            }
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_missing_asts(&mut self) -> Result<()> {
        self.pages.iter_mut().for_each(|p| {
            if p.1.ast.len() == 0 {
                p.1.generate_ast(&self.config);
            }
        });
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_page_content(&mut self) -> Result<()> {
        event!(Level::INFO, "Generating Page Content");
        let site = Value::from_object(SiteV2::new(&self.config, &self.pages));
        let mut env = Environment::new();
        env.set_debug(true);
        env.add_function("highlight_code", highlight_code);
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        WalkDir::new(self.config.templates_dir())
            .into_iter()
            .filter(|entry| match entry.as_ref().unwrap().path().extension() {
                Some(ext) => ext.to_str().unwrap() == "neoj",
                None => false,
            })
            .for_each(|entry| {
                let path = entry.as_ref().unwrap().path().to_path_buf();
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        let template_name = path.strip_prefix(self.config.templates_dir()).unwrap();
                        let _ = env.add_template_owned(
                            template_name.to_string_lossy().to_string(),
                            content,
                        );
                    }
                    Err(e) => {
                        event!(Level::ERROR, "{}", e)
                    }
                };
            });
        self.pages.iter_mut().for_each(|p| {
            match p.1.output {
                Some(_) => {}
                None => {
                    let template_name = "pages/post/published.neoj";
                    if let Ok(tmpl) = env.get_template(template_name) {
                        match tmpl.render(context!(
                            site => site,
                            page_id => p.1.id()
                        )) {
                            Ok(output) => {
                                self.last_edit = Some(output.clone());
                                p.1.output = Some(output);
                            }
                            Err(e) => {
                                // TODO: Provide error handling here
                                event!(Level::ERROR, "{}", e);
                                p.1.output = None;
                            }
                        }
                    } else {
                        // TODO: Provide error handling here
                        event!(Level::ERROR, "Could not get template: {}", template_name);
                    }
                }
            }
        });
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_cached_pages(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Cached Files");
        let conn = Connection::open(self.config.cache_db_path())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS page_archive (path TEXT, page TEXT)",
            (),
        )?;
        let mut stmt = conn.prepare("SELECT path, page FROM page_archive")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let path_string: String = row.get(0)?;
            let path = PathBuf::from(path_string);
            let page: String = row.get(1)?;
            let p: PageV2 = serde_json::from_str(&page.to_string())?;
            self.pages.insert(path, p);
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_source_files(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Source Files");
        let dir = &self.config.paths.get("content_root").unwrap();
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
                        let page =
                            PageV2::new_from_filesystem(path.clone(), self.config.clone(), content);
                        match self.pages.get(&path.clone()) {
                            Some(cache_page) => {
                                if cache_page.hash() != page.hash() {
                                    self.pages.insert(path.clone(), page);
                                }
                            }
                            None => {
                                self.pages.insert(path.clone(), page);
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
    pub fn make_images(&mut self) -> Result<()> {
        event!(Level::INFO, "Making Images");
        for entry in WalkDir::new(self.config.image_source_dir()) {
            let source_path = entry?.into_path();
            if let (Some(stem), Some(extension)) =
                (source_path.file_stem(), source_path.extension())
            {
                let stem = stem.to_string_lossy().to_string().to_lowercase();
                if extension.to_ascii_lowercase() == "jpg"
                    || extension.to_ascii_lowercase() == "jpeg"
                    || extension.to_ascii_lowercase() == "png"
                {
                    let cache_path = self.config.image_cache_dir().join(
                        source_path
                            .strip_prefix(self.config.image_source_dir())
                            .unwrap(),
                    );
                    let dest_path = self.config.image_dest_dir().join(
                        source_path
                            .strip_prefix(self.config.image_source_dir())
                            .unwrap(),
                    );
                    if source_path.is_dir() {
                        fs::create_dir_all(&cache_path)?;
                        fs::create_dir_all(&dest_path)?;
                    } else {
                        if is_cache_stale(&source_path, &cache_path) {
                            // don't use fs::copy here because it'll trigger an
                            // infinite loop with notify on macs
                            let data = std::fs::read(&source_path)?;
                            std::fs::write(&cache_path, &data)?;
                            let image_dir = &cache_path.parent().unwrap().join(stem);
                            // dbg!(image_dir);
                            fs::create_dir_all(&image_dir)?;
                            let decoder = Decoder::from_path(&source_path)?;
                            let image = decoder.decode()?;
                            let resize_width =
                                std::cmp::min(image.width(), self.config.max_image_width.unwrap());
                            // dbg!(resize_width);
                            let base_image_path = image_dir.join(format!(
                                "base.{}",
                                extension.to_string_lossy().to_ascii_lowercase()
                            ));
                            if &extension.to_ascii_lowercase() == "jpg"
                                || &extension.to_ascii_lowercase() == "jpeg"
                            {
                                resize_and_optimize_jpg(
                                    &cache_path,
                                    resize_width,
                                    &base_image_path,
                                )?;
                            }
                            if &extension.to_ascii_lowercase() == "png" {
                                resize_and_optimize_png(
                                    &cache_path,
                                    resize_width,
                                    &base_image_path,
                                )?;
                            }
                        }
                    }
                }
            }
        }
        for entry in WalkDir::new(self.config.image_cache_dir()) {
            let cache_path = entry?.into_path();
            let dest_path = self.config.image_dest_dir().join(
                cache_path
                    .strip_prefix(self.config.image_cache_dir())
                    .unwrap(),
            );
            if cache_path.is_dir() {
                fs::create_dir_all(&dest_path)?;
            } else {
                // don't use fs::copy here because it'll trigger an
                // infinite loop with notify on macs
                let data = std::fs::read(&cache_path)?;
                std::fs::write(&dest_path, &data)?;
            }
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn make_og_images(&self) -> Result<()> {
        event!(Level::INFO, "Making OG Images");
        let _ = fs::create_dir_all(self.config.tmp_dir());
        let _ = fs::create_dir_all(self.config.og_images_cache_dir());
        let _ = fs::create_dir_all(self.config.og_images_dir());
        let tmp_path = self.config.tmp_dir().join("og-image.png");
        for p in &self.pages {
            if let (Some(id), Some(title)) = (&p.1.id(), &p.1.title_as_plain_text()) {
                let mut make_image = false;
                let cache_path = &self
                    .config
                    .og_images_cache_dir()
                    .join(format!("{}.jpg", &id));
                let output_path = &self.config.og_images_dir().join(format!("{}.jpg", &id));
                if !cache_path.exists() {
                    make_image = true;
                } else {
                    if let (Ok(content_metadata), Ok(cache_metadata)) = (
                        fs::metadata(&p.1.source_path.clone().unwrap()),
                        fs::metadata(&cache_path),
                    ) {
                        if let (Ok(content_time), Ok(cache_time)) =
                            (content_metadata.modified(), cache_metadata.modified())
                        {
                            if content_time > cache_time {
                                make_image = true
                            }
                        }
                    }
                }
                if make_image {
                    event!(Level::INFO, "Making OG Image: {} - {}", &id, &title);
                    let og_image = OgImage {
                        text_areas: vec![
                            OgImageTextArea {
                                color: "#0481c5".to_string(),
                                font_family: "Arial".to_string(),
                                font_size: 20,
                                line_height: 40,
                                max_char_width: 20,
                                max_lines: 2,
                                text: "alanwsmith.com".to_string(),
                                x: 1000,
                                y: 600,
                            },
                            OgImageTextArea {
                                color: "#0481c5".to_string(),
                                font_family: "Arial".to_string(),
                                font_size: 86,
                                line_height: 100,
                                max_char_width: 18,
                                max_lines: 4,
                                text: title.to_string(),
                                x: 80,
                                y: 220,
                            },
                        ],
                    };
                    og_image.render_svg(&tmp_path);
                    let decoder = Decoder::from_path(&tmp_path)?;
                    let image = decoder.decode()?;
                    let config = EncoderConfig::new(Codec::MozJpeg)
                        .with_quality(80.0)
                        .unwrap();
                    let file = File::create(&cache_path)?;
                    let encoder = Encoder::new(file, DynamicImage::ImageRgba8(image.into()))
                        .with_config(config);
                    encoder.encode()?;
                }
                let _ = std::fs::copy(&cache_path, &output_path);
            }
        }
        let custom_og_images = get_files_with_extension_in_a_single_directory(
            &self.config.custom_og_images_dir(),
            "jpg",
        );
        for custom_image in custom_og_images {
            let output_path = self
                .config
                .og_images_dir()
                .join(custom_image.file_name().unwrap());
            fs::copy(&custom_image, &output_path)?;
        }
        Ok(())
    }

    pub fn output_content_files(&self) -> Result<()> {
        self.pages.iter().for_each(|p| {
            if let (Some(rel_file_path), Some(output)) = (p.1.rel_file_path(), p.1.output.clone()) {
                let output_path = self.config.output_dir().join(rel_file_path);
                // TODO: Add error handling here
                let _ = write_file_with_mkdir(&output_path, &output);
            }
        });
        Ok(())
    }

    pub fn output_last_edit(&self) -> Result<()> {
        let output_path = self.config.output_dir().join("last-edit/index.html");
        let no_last_edit_content = r#"<!DOCTYPE html>
<html><head><style>
body {
    background-color: #333;
    color: #aaa;
    font-size: 4rem;
}
</style></head>
<body>
<a href="/">Home</a>
<p>No edits made yet in this session</p>
</body>
</html>"#
            .to_string();
        if let Some(content) = &self.last_edit {
            let _ = write_file_with_mkdir(&output_path, &content);
        } else {
            let _ = write_file_with_mkdir(&output_path, &no_last_edit_content);
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn prep_dirs(&self) -> Result<()> {
        event!(Level::INFO, "Making Sure Directories Exist");
        let _ = fs::create_dir_all(self.config.custom_og_images_dir());
        let _ = fs::create_dir_all(self.config.image_cache_dir());
        let _ = fs::create_dir_all(self.config.image_dest_dir());
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn update_cache(&self) -> Result<()> {
        event!(Level::INFO, "Updating Cache");
        let mut conn = Connection::open(self.config.cache_db_path())?;
        conn.execute("DROP TABLE IF EXISTS page_archive", ())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS page_archive (path TEXT, page TEXT)",
            (),
        )?;
        let query = "INSERT INTO page_archive(path, page) VALUES (?1, ?2)";
        let tx = conn.transaction()?;
        for p in self.pages.iter() {
            if let Ok(data) = serde_json::to_string(p.1) {
                tx.execute(
                    query,
                    (
                        p.1.source_path
                            .clone()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                        data,
                    ),
                )?;
            };
        }
        tx.commit()?;
        Ok(())
    }

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

pub fn highlight_code(args: &[Value]) -> String {
    let code = args[0].to_string();
    let lang = args[1].to_string();
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set
        .find_syntax_by_token(&lang)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
    for line in LinesWithEndings::from(&trim_empty_lines(&code)) {
        let _ = html_generator.parse_html_for_line_which_includes_newline(line);
    }
    let initial_html = html_generator.finalize();
    let output_html: Vec<_> = initial_html
        .lines()
        .map(|line| format!(r#"<span class="line-marker"></span>{}"#, line))
        .collect();
    output_html.join("\n")
}

pub fn is_cache_stale(source_file: &PathBuf, cache_file: &PathBuf) -> bool {
    // TODO: Actually check the cache
    true
}

fn resize_and_optimize_jpg(source: &PathBuf, width: u32, dest: &PathBuf) -> Result<()> {
    let decoder = Decoder::from_path(source)?;
    let image = decoder.decode()?;
    let height = image.height() * width / image.width();
    let resized_image = image.resize_to_fill(width, height, FilterType::Lanczos3);
    let config = EncoderConfig::new(Codec::MozJpeg)
        .with_quality(90.0)
        .unwrap();
    let file = File::create(&dest)?;
    let encoder =
        Encoder::new(file, DynamicImage::ImageRgba8(resized_image.into())).with_config(config);
    encoder.encode()?;
    Ok(())
}

fn resize_and_optimize_png(source: &PathBuf, width: u32, dest: &PathBuf) -> Result<()> {
    let decoder = Decoder::from_path(source)?;
    let image = decoder.decode()?;
    let height = image.height() * width / image.width();
    let resized_image = image.resize_to_fill(width, height, FilterType::Lanczos3);
    let config = EncoderConfig::new(Codec::OxiPng);
    let file = File::create(&dest)?;
    let encoder =
        Encoder::new(file, DynamicImage::ImageRgba8(resized_image.into())).with_config(config);
    encoder.encode()?;
    Ok(())
}

pub fn trim_empty_lines(source: &str) -> String {
    let re = Regex::new(r"\S").unwrap();
    let trimmed_front = source.split("\n").fold("".to_string(), |acc, l| {
        if !acc.is_empty() {
            acc + l + "\n"
        } else {
            if re.is_match(l) {
                l.to_string() + "\n"
            } else {
                acc
            }
        }
    });
    trimmed_front.trim_end().to_string()
}

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
