use crate::helpers::*;
use crate::image::{Image, ImageSize};
use crate::neo_error::{NeoError, NeoErrorKind};
use crate::page_payload::{PagePayload, ThemeTestOrPage};
use crate::site::Site;
use crate::site_config::SiteConfig;
use crate::source_page::SourcePage;
use crate::theme_test::ThemeTest; // this might be deprecated
use anyhow::Result;
use fs_extra::dir::copy;
use image::io::Reader;
use minijinja::syntax::SyntaxConfig;
use minijinja::Environment;
use minijinja::{context, Value};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Builder {
    pub cache_buffer: BTreeMap<PathBuf, SourcePage>,
    pub config: Option<SiteConfig>,
    pub errors: Vec<NeoError>,
    pub images: BTreeMap<String, Image>,
    pub source_pages: BTreeMap<PathBuf, SourcePage>,
    pub payloads: BTreeMap<String, PagePayload>,
    pub templates: BTreeMap<String, String>,
    pub theme_test_source_pages: BTreeMap<PathBuf, ThemeTest>,
}

impl Builder {
    pub fn new_from_site_config(site_config: &SiteConfig) -> Result<Builder, NeoError> {
        let b = Builder {
            cache_buffer: BTreeMap::new(),
            config: Some(site_config.clone()),
            errors: vec![],
            images: BTreeMap::new(),
            source_pages: BTreeMap::new(),
            payloads: BTreeMap::new(),
            templates: BTreeMap::new(),
            theme_test_source_pages: BTreeMap::new(),
        };
        Ok(b)
    }

    // // DEPRECATED: Remove this when new_from_site_config is working
    // pub fn new_from_engine_config(engine_config: &EngineConfig) -> Result<Builder, NeoError> {
    //     let project_root = engine_config
    //         .sites_dir
    //         .join(engine_config.active_site.as_str());
    //     let config_path = project_root.join("admin").join("config.json");
    //     match fs::read_to_string(config_path) {
    //         Ok(text) => match serde_json::from_str::<SiteConfig>(&text) {
    //             Ok(mut config) => {
    //                 config.project_root = Some(project_root);
    //                 config.load_sections();
    //                 let b = Builder {
    //                     cache_buffer: BTreeMap::new(),
    //                     config: Some(config),
    //                     errors: vec![],
    //                     source_pages: BTreeMap::new(),
    //                     payloads: BTreeMap::new(),
    //                     templates: BTreeMap::new(),
    //                 };
    //                 Ok(b)
    //             }
    //             Err(e) => Err(NeoError {
    //                 kind: NeoErrorKind::GenericErrorWithoutSourcePath {
    //                     msg: format!("could not load admin/config.json file: {}", e),
    //                 },
    //             }),
    //         },
    //         Err(e) => Err(NeoError {
    //             kind: NeoErrorKind::GenericErrorWithoutSourcePath {
    //                 msg: format!("could not load admin/config.json file: {}", e),
    //             },
    //         }),
    //     }
    // }

    //
}

impl Builder {
    #[instrument(skip(self))]
    pub fn build_image_cache(&mut self) -> Result<()> {
        event!(Level::INFO, "Building Image Cache");
        let image_source_paths = get_image_paths(&self.config.as_ref().unwrap().image_source_dir());
        image_source_paths.iter().for_each(|img_path| {
            // TODO: Handle pngs and other image formats
            if let Some(ext) = img_path.extension() {
                if let Ok(img_reader) = Reader::open(img_path) {
                    if let Ok(img) = img_reader.decode() {
                        let width = img.width();
                        let height = img.height();
                        let image_name = img_path.file_stem().unwrap();
                        let image_dest_dir = self
                            .config
                            .as_ref()
                            .unwrap()
                            .image_cache_dir()
                            .join(image_name);
                        let _ = fs::create_dir_all(&image_dest_dir);
                        let image_dest_path = image_dest_dir.join(format!(
                            "{}w.{}",
                            width,
                            ext.to_string_lossy().to_ascii_lowercase()
                        ));
                        let _ = fs::copy(img_path, image_dest_path);
                        let mut sizes = vec![];
                        self.config
                            .as_ref()
                            .unwrap()
                            .theme_image_widths()
                            .iter()
                            .for_each(|img_width| {
                                if width > *img_width {
                                    let resize_dest_path = image_dest_dir.join(format!(
                                        "{}w.{}",
                                        img_width,
                                        ext.to_string_lossy().to_ascii_lowercase()
                                    ));
                                    if ext.to_ascii_lowercase() == "jpg"
                                        || ext.to_ascii_lowercase() == "jpeg"
                                    {
                                        let _ = resize_and_optimize_jpg(
                                            &img_path,
                                            *img_width,
                                            &resize_dest_path,
                                        );
                                        let resize_height = img.height() * *img_width / img.width();
                                        sizes.push(ImageSize {
                                            width: *img_width,
                                            height: resize_height,
                                        });
                                    } else if ext.to_ascii_lowercase() == "png" {
                                        let _ = resize_and_optimize_png(
                                            &img_path,
                                            *img_width,
                                            &resize_dest_path,
                                        );
                                        let resize_height = img.height() * *img_width / img.width();
                                        sizes.push(ImageSize {
                                            width: *img_width,
                                            height: resize_height,
                                        });
                                    }

                                    //                 } else if image.extension()? == "png" {
                                    //                 } else {
                                    //                     event!(
                                    //                         Level::ERROR,
                                    //                         "TODO: Process other image types: {}",
                                    //                         &image.source_path.display()
                                    //                     );
                                    //                 }
                                    //             }
                                };
                            });

                        if let Some(max_width_size) = self
                            .config
                            .clone()
                            .unwrap()
                            .theme_image_widths()
                            .into_iter()
                            .max()
                        {
                            if width < max_width_size {
                                sizes.push(ImageSize { width, height });
                            }
                        }

                        let img_obj = Image {
                            extension: ext.to_string_lossy().to_string(),
                            dir: PathBuf::from(format!(
                                "/neo-images/{}",
                                image_name.to_string_lossy().to_string()
                            )),
                            raw_width: width,
                            raw_height: height,
                            sizes,
                        };
                        self.images
                            .insert(image_name.to_string_lossy().to_string(), img_obj);
                    }
                }
            }
        });
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn deploy_images(&mut self) -> Result<()> {
        event!(Level::DEBUG, "Deploying Images");
        let image_source_paths = get_image_paths(&self.config.as_ref().unwrap().image_cache_dir());
        image_source_paths.iter().for_each(|source_path| {
            if let Ok(partial_path) =
                source_path.strip_prefix(&self.config.as_ref().unwrap().image_cache_dir())
            {
                let dest_path = &self
                    .config
                    .as_ref()
                    .unwrap()
                    .image_dest_dir()
                    .join(partial_path);
                if let Some(dest_dir) = dest_path.parent() {
                    let _ = fs::create_dir_all(dest_dir);
                    // NOTE: Using read/write here instead of copy to work
                    // around a notify reload bug that triggers on copy when it shouldn't
                    if let Ok(data) = std::fs::read(source_path) {
                        match std::fs::write(dest_path, &data) {
                            Ok(_) => {}
                            Err(e) => {
                                let _ = &self.errors.push(NeoError {
                                    kind: NeoErrorKind::GenericErrorWithSourcePath {
                                        source_path: source_path.to_path_buf(),
                                        msg: format!("Could not copy image: {}", e),
                                    },
                                });
                            }
                        }
                    }
                }
            } else {
                let _ = &self.errors.push(NeoError {
                    kind: NeoErrorKind::GenericErrorWithSourcePath {
                        source_path: source_path.to_path_buf(),
                        msg: "Could not move image".to_string(),
                    },
                });
            }
        });
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn deploy_theme_files(&self) {
        event!(Level::DEBUG, "Deploying Theme Files");
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let source_dir = self.config.as_ref().unwrap().theme_dir().join("files");
        let dest_dir = self
            .config
            .as_ref()
            .unwrap()
            .output_dest_dir()
            .join("theme");
        match copy(source_dir, dest_dir, &options) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    #[instrument(skip(self))]
    pub fn empty_output_dirs(&self) {
        event!(Level::DEBUG, "Emptying output dirs");
        let _ = empty_dir(&self.config.as_ref().unwrap().status_dest_dir());
        let _ = empty_dir(&self.config.as_ref().unwrap().output_dest_dir());
    }

    #[instrument(skip(self))]
    pub fn generate_missing_asts(&mut self) {
        event!(Level::INFO, "Generating Missing ASTs");
        self.source_pages.iter_mut().for_each(|(sp, page)| {
            if let None = page.ast {
                if let Err(e) = page.generate_ast() {
                    self.errors.push(match e.kind {
                        NeoErrorKind::ParserError {
                            line,
                            column,
                            remainder,
                            source,
                            message,
                            ..
                        } => NeoError {
                            kind: NeoErrorKind::ParserError {
                                source_path: Some(sp.clone()),
                                line,
                                column,
                                remainder,
                                source,
                                message,
                            },
                        },
                        _ => e,
                    })
                    // e.source_path = PathBuf::from("asdf");
                    //self.errors.push(e);
                }
            }
        })
    }

    #[instrument(skip(self))]
    pub fn generate_payloads(&mut self, theme_test_or_page: ThemeTestOrPage) {
        self.payloads = BTreeMap::new();
        self.source_pages.iter().for_each(|(source_path, page)| {
            match PagePayload::new_from_source_page(&source_path, &page, theme_test_or_page.clone())
            {
                Ok(p) => {
                    self.payloads.insert(p.id.as_ref().unwrap().to_string(), p);
                    ()
                }
                Err(e) => {
                    self.errors.push(e);
                    ()
                }
            };
            ()

            //match PagePayload::new_from_source_page(&page) {
            //    Ok(p) => match p.id {
            //        Some(id) => {
            //            self.payloads.insert(id, p);
            //            ()
            //        }
            //        None => {
            //            dbg!("TODO: Mark ERROR for missing page ID");
            //            ()
            //        }
            //    },
            //    Err(e) => {
            //        if let Some(source_path) = &page.source_path {
            //            //dbg!("--------------------------------");
            //            self.errors.push(NeoError {
            //                kind: NeoErrorKind::ForwardErrorWithSourcePath {
            //                    source_path: source_path.clone(),
            //                    //msg: e.to_string(),
            //                    msg: format!("Could not get file id: {}", e),
            //                },
            //            });
            //        } else {
            //            //dbg!("--------------------------------");
            //            self.errors.push(NeoError {
            //                // kind: NeoErrorKind::ForwardError { msg: e.to_string() },
            //                kind: NeoErrorKind::ForwardError {
            //                    msg: "ERROR HERE".to_string(),
            //                },
            //            });
            //        }
            //        event!(
            //            Level::ERROR,
            //            "Page load error: TODO: make this a better message: {}",
            //            e.to_string()
            //        );
            //    }
            //}

            //
        });
    }

    #[instrument(skip(self))]
    pub fn load_pages_from_cache(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Pages From Cache");
        let conn = Connection::open(self.config.as_ref().unwrap().cache_db_path())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS page_archive (path TEXT, page_object Text)",
            (),
        )?;
        let mut stmt = conn.prepare("SELECT path, page_object FROM page_archive")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let path_string: String = row.get(0)?;
            let path = PathBuf::from(path_string);
            let page: String = row.get(1)?;
            let p: SourcePage = serde_json::from_str(&page.to_string())?;
            self.cache_buffer.insert(path, p);
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_pages_from_fs(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Source Content Files");
        for entry in WalkDir::new(&self.config.as_ref().unwrap().content_source_dir()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neo"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        if let Some(cached_page) = self.cache_buffer.get(&path) {
                            let updated = fs::metadata(&path)
                                .unwrap()
                                .modified()
                                .unwrap()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs();
                            if cached_page.updated.unwrap() == updated {
                                let mut updated_cached_page = cached_page.clone();
                                updated_cached_page.config =
                                    Some(self.config.as_ref().unwrap().clone());
                                self.source_pages.insert(path.clone(), updated_cached_page);
                            } else {
                                match SourcePage::new_from_source_path(
                                    &path,
                                    self.config.as_ref().unwrap().clone(),
                                ) {
                                    Ok(p) => {
                                        self.source_pages.insert(path.clone(), p);
                                    }
                                    Err(_e) => {
                                        dbg!("TODO: hoist page could not load error");
                                        ()
                                    }
                                }
                            }
                        } else {
                            match SourcePage::new_from_source_path(
                                &path,
                                self.config.as_ref().unwrap().clone(),
                            ) {
                                Ok(p) => {
                                    self.source_pages.insert(path.clone(), p);
                                }
                                Err(_e) => {
                                    dbg!("TODO: hoist page could not load error");
                                    ()
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_templates(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Templates");
        for entry in WalkDir::new(&self.config.as_ref().unwrap().theme_dir()) {
            //for entry in WalkDir::new(&self.config.as_ref().unwrap().templates_dir()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neoj"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        let template_name =
                            &path.strip_prefix(&self.config.as_ref().unwrap().theme_dir());
                        //&path.strip_prefix(&self.config.as_ref().unwrap().templates_dir());
                        let content = fs::read_to_string(&path)?;
                        self.templates.insert(
                            template_name
                                .as_ref()
                                .unwrap()
                                .to_string_lossy()
                                .to_string(),
                            content,
                        );
                    }
                }
            }
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_theme_test_pages(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Theme Test Pages");
        // Reminder: clear the original pages
        self.source_pages = BTreeMap::new();
        for entry in WalkDir::new(&self.config.as_ref().unwrap().theme_dir()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neo"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        if let Some(cached_page) = self.cache_buffer.get(&path) {
                            let updated = fs::metadata(&path)
                                .unwrap()
                                .modified()
                                .unwrap()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs();
                            if cached_page.updated.unwrap() == updated {
                                let mut updated_cached_page = cached_page.clone();
                                updated_cached_page.config =
                                    Some(self.config.as_ref().unwrap().clone());
                                self.source_pages.insert(path.clone(), updated_cached_page);
                            } else {
                                match SourcePage::new_from_source_path(
                                    &path,
                                    self.config.as_ref().unwrap().clone(),
                                ) {
                                    Ok(p) => {
                                        self.source_pages.insert(path.clone(), p);
                                    }
                                    Err(_e) => {
                                        dbg!("TODO: hoist page could not load error");
                                        ()
                                    }
                                }
                            }
                        } else {
                            match SourcePage::new_from_source_path(
                                &path,
                                self.config.as_ref().unwrap().clone(),
                            ) {
                                Ok(p) => {
                                    self.source_pages.insert(path.clone(), p);
                                }
                                Err(_e) => {
                                    dbg!("TODO: hoist page could not load error");
                                    ()
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_image_details(&self) -> Value {
        Value::from("asdf")
    }

    #[instrument(skip(self))]
    pub fn output_pages(&mut self) -> Result<()> {
        event!(Level::INFO, "Outputting pages");
        let mut env = Environment::new();
        env.add_function("highlight_code", highlight_code);
        env.add_function("highlight_span", highlight_span);
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        for (id, data) in self.templates.iter() {
            match env.add_template_owned(id, data) {
                Ok(_) => {}
                Err(e) => {
                    dbg!(e);
                    {}
                }
            }
        }

        let mut site_obj =
            Site::new_from_payloads(self.config.as_ref().unwrap().clone(), &self.payloads);
        site_obj.images = self.images.clone();

        let site = Value::from_serialize(site_obj);

        for (_, page) in self.payloads.iter_mut() {
            let output_path = self
                .config
                .as_ref()
                .unwrap()
                .output_dest_dir()
                .join(page.rel_file_path.as_ref().unwrap());
            if let Some(template) = page.template_list.iter().find_map(|name| {
                if let Ok(tmpl) = env.get_template(name) {
                    page.used_template = Some(name.clone());
                    Some(tmpl)
                } else {
                    None
                }
            }) {
                match template.render(context!(
                    page => Value::from_serialize(&page),
                    site=> &site,
                )) {
                    Ok(output) => {
                        let _ = write_file_with_mkdir(&output_path, &output);
                    }
                    Err(e) => {
                        self.errors.push(NeoError {
                            kind: NeoErrorKind::GenericErrorWithSourcePath {
                                source_path: page.source_path.as_ref().unwrap().clone(),
                                msg: e.to_string(),
                            },
                        });
                    }
                };
            } else {
                self.errors.push(NeoError {
                    kind: NeoErrorKind::CouldNotFindPageTemplate {
                        rel_source_path: page.rel_source_path.clone(),
                        msg: None,
                        template_list: Some(page.template_list.clone()),
                    },
                });
            };

            //
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn prep_output_dirs(&self) -> Result<()> {
        fs::create_dir_all(self.config.as_ref().unwrap().cache_dir())?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn save_asts_to_cache(&self) -> Result<()> {
        event!(Level::INFO, "Saving ASTs to Cache");
        let mut conn = Connection::open(self.config.as_ref().unwrap().cache_db_path())?;
        conn.execute("DROP TABLE IF EXISTS page_archive", ())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS page_archive (path TEXT, page_object TEXT)",
            (),
        )?;
        let query = "INSERT INTO page_archive(path, page_object) VALUES (?1, ?2)";
        let tx = conn.transaction()?;
        for (_, p) in self.source_pages.iter() {
            match serde_json::to_string(p) {
                Ok(page_object) => {
                    match tx.execute(
                        query,
                        (
                            p.source_path.clone().unwrap().to_string_lossy().to_string(),
                            page_object.clone(),
                        ),
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            dbg!(e);
                            ()
                        }
                    }
                }
                Err(e) => {
                    dbg!(e);
                    ()
                }
            }
        }
        tx.commit()?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn test_theme(&mut self) -> Result<()> {
        let mut env = Environment::new();
        env.set_lstrip_blocks(true);
        env.set_trim_blocks(true);
        env.add_function("highlight_code", highlight_code);
        env.add_function("highlight_span", highlight_span);
        // TODO: Add start-test-template and expected-output templates
        // TODO: Update "pages/post/published.neoj" for theme test
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        for (id, data) in self.templates.iter() {
            match env.add_template_owned(id, data) {
                Ok(_) => {}
                Err(e) => {
                    dbg!(e);
                    {}
                }
            }
        }
        env.add_template_owned(
            "pages/post/published.neoj",
            r#"
[! include "includes/config.neoj" !]
[! import "includes/theme.neoj" as theme !]
[! for section in page.sections !]
[@ theme.output_section("", "", section) @]
[! endfor !]
"#
            .to_string(),
        )
        .unwrap();
        env.add_template_owned(
            "sections/basic/start-theme-test/full/default.neoj",
            r#"<!-- START_THEME_TEST -->"#.to_string(),
        )
        .unwrap();
        env.add_template_owned(
            "sections/raw/expected-output/start/default.neoj",
            r#"
[! import "includes/theme.neoj" as theme !]
<!-- EXPECTED_OUTPUT -->
[@ section.text @]
[! for child in section.children !]
[@ theme.output_section("", "", child) @]
[! endfor !]
"#
            .to_string(),
        )
        .unwrap();
        env.add_template_owned(
            "sections/raw/expected-output/end/default.neoj",
            "<!-- EXPECTED_OUTPUT -->".to_string(),
        )
        .unwrap();

        for (_, page) in self.payloads.iter_mut() {
            if let Some(template) = page.template_list.iter().find_map(|name| {
                if let Ok(tmpl) = env.get_template(name) {
                    page.used_template = Some(name.clone());
                    Some(tmpl)
                } else {
                    None
                }
            }) {
                match template.render(context!(
                    page => Value::from_serialize(&page)
                )) {
                    Ok(output) => {
                        let tests = output
                            .split("<!-- START_THEME_TEST -->")
                            .collect::<Vec<&str>>();
                        tests.iter().skip(1).for_each(|t| {
                            let parts = t.split("<!-- EXPECTED_OUTPUT -->").collect::<Vec<&str>>();
                            // dbg!(&parts);
                            if parts.len() == 3 {
                                let got = format_html_for_theme_test_display(parts[0]);
                                let expected = format_html_for_theme_test_display(parts[1]);
                                let got_compare = got.replace("\n", "").replace(" ", "");
                                let expected_compare = expected.replace("\n", "").replace(" ", "");
                                if got_compare != expected_compare {
                                    self.errors.push(NeoError {
                                        kind: NeoErrorKind::ThemeTestError {
                                            expected: expected.to_string(),
                                            got: got.to_string(),
                                            sections: page.sections.clone(),
                                            source_path: page.source_path.as_ref().unwrap().clone(),
                                        },
                                    })
                                }
                            }
                        });
                    }
                    Err(e) => {
                        self.errors.push(NeoError {
                            kind: NeoErrorKind::GenericErrorWithSourcePath {
                                source_path: page.source_path.as_ref().unwrap().clone(),
                                msg: e.to_string(),
                            },
                        });
                    }
                };
            } else {
                self.errors.push(NeoError {
                    kind: NeoErrorKind::CouldNotFindPageTemplate {
                        rel_source_path: page.rel_source_path.clone(),
                        msg: None,
                        template_list: Some(page.template_list.clone()),
                    },
                });
            };
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn tmp_output_errors(&self) -> Result<()> {
        event!(Level::INFO, "Outputting errors");
        let mut env = Environment::new();
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        env.set_debug(true);
        env.set_lstrip_blocks(true);
        env.set_trim_blocks(true);
        let _ = env.add_template(
            "genericerrorwithsourcepath",
            include_str!("../error_templates/genericerrorwithsourcepath.neoj"),
        );
        let _ = env.add_template(
            "parsererror",
            include_str!("../error_templates/parsererror.neoj"),
        );
        let _ = env.add_template(
            "themetesterror",
            include_str!("../error_templates/themetesterror.neoj"),
        );
        let _ = env.add_template("_todo", include_str!("../error_templates/_todo.neoj"));
        env.add_template_owned(
            "tmp_status",
            r#"
<!DOCTYPE html>
<html><head><style> 
body { background-color: #111; color: #aaa; } 
</style></head><body>
<header><a href="/">Home</a></header>
<h1>Status</h1>
<h2>Errors</h2>
<ul>
[! for error in errors !]
<li>
[! include [error.kind.type, "_todo"] !]
[#
[! if error.kind.source_path !]<h4>Path: [@ error.kind.source_path @]</h4>[! endif !]
[! if error.kind.rel_source_path !]<h4>File Path: [@ error.kind.rel_source_path @]</h4>[! endif !]
[! if error.kind.type == "themetesterror" !]
    <h2>Theme Test Error</h2>
    <h3>Expected</h3>
    <pre>
    [! autoescape true !]
    [@ error.kind.expected@]
    [! endautoescape !]
    </pre>
    <h3>Got</h3>
    <pre>
    [! autoescape true !]
    [@ error.kind.got @]
    [! endautoescape !]
    </pre>
    <h3>Sections</h3>
    <pre>
    [@ error.kind.sections|tojson(true)@]
    </pre>
[! else !]
    <h2>[@ error.kind.type @]</h2>
    <pre>
    </pre>
[! endif !]
#]
</li>
[! endfor !]
</ul>
<h2>Config</h2>
<pre>[@ config|tojson(true) @]</pre>
</body></html>
        "#,
        )?;
        let tmpl = env.get_template("tmp_status")?;
        let output = tmpl.render(context!(
            config => Value::from_serialize(&self.config),
            errors => Value::from_serialize(&self.errors)
        ))?;
        let status_path = self
            .config
            .as_ref()
            .unwrap()
            .status_dest_dir()
            .join("index.html");
        let _ = write_file_with_mkdir(&status_path, &output);
        Ok(())
    }

    #[instrument(skip(self, thing))]
    pub fn todo(&self, thing: &str) {
        event!(Level::INFO, "TODO: {}", thing);
    }

    #[instrument(skip(self))]
    pub fn update_config_for_theme_test(&mut self) {
        event!(Level::INFO, "Updeating config for theme test");
        self.config
            .as_mut()
            .unwrap()
            .sections
            .basic
            .push("start-theme-test".to_string());
        self.config
            .as_mut()
            .unwrap()
            .sections
            .raw
            .push("expected-output".to_string());
    }
}
