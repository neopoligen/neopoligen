// DEPREDATED: This is the old version of
//  the site which can be removed when time allows
//
use crate::og_image::*;
use crate::page::Page;
use crate::site_config::SiteConfig;
use crate::template_test::*;
use anyhow::Result;
use html_escape::encode_text;
use image::DynamicImage;
use minijinja::context;
use minijinja::syntax::SyntaxConfig;
use minijinja::value::Value;
use minijinja::Environment;
use regex::Regex;
use rimage::config::{Codec, EncoderConfig};
use rimage::Decoder;
use rimage::Encoder;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub config: SiteConfig,
    pub missing_ids: BTreeMap<PathBuf, String>,
    pub pages: BTreeMap<String, Page>,
    pub source_files: BTreeMap<PathBuf, String>,
    pub templates: BTreeMap<String, String>,
    pub template_tests: Vec<TemplateTest>,

    // deprecate these so you can run with the
    // main templates with ``site.pages[page_id]``
    pub template_test_files: BTreeMap<PathBuf, String>,
    pub template_test_pages: BTreeMap<String, Page>,
    pub template_test_page_errors: Vec<Page>,
    pub template_test_render_errors: BTreeMap<PathBuf, String>,
}

impl Site {
    #[instrument]
    pub fn new(config: SiteConfig) -> Site {
        event!(Level::DEBUG, "Creating Site Object");
        Site {
            config,
            source_files: BTreeMap::new(),
            missing_ids: BTreeMap::new(),
            pages: BTreeMap::new(),
            templates: BTreeMap::new(),
            template_tests: vec![],
            // todo: deprecated these four
            template_test_files: BTreeMap::new(),
            template_test_pages: BTreeMap::new(),
            template_test_page_errors: vec![],
            template_test_render_errors: BTreeMap::new(),
        }
    }
}

impl Site {
    pub fn copy_images(&self) -> Result<(), std::io::Error> {
        let source_dir = self.config.image_source_dir();
        let dest_dir = self.config.output_dir().join(PathBuf::from("images"));
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
    pub fn make_og_images(&self) {
        event!(Level::INFO, "Making OG Images");
        let og_image_cache_dir = self.config.cache_dir().join("og-images");
        let _ = fs::create_dir_all(self.config.og_images_dir());
        let _ = fs::create_dir_all(&og_image_cache_dir);
        self.pages.iter().for_each(|p| {
            if let (Some(id), Some(title)) = (&p.1.id, &p.1.title_as_plain_text) {
                let mut make_image = false;
                let cache_path = og_image_cache_dir.join(format!("{}.png", &id));
                if !cache_path.exists() {
                    make_image = true;
                } else {
                    if let (Ok(content_metadata), Ok(cache_metadata)) =
                        (fs::metadata(&p.1.source_path), fs::metadata(&cache_path))
                    {
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
                    // event!(Level::INFO, "Making OG Image: {} - {}", &id, &title);
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
                                font_size: 92,
                                line_height: 100,
                                max_char_width: 18,
                                max_lines: 4,
                                text: encode_text(title).to_string(),
                                x: 110,
                                y: 240,
                            },
                        ],
                    };
                    og_image.render_svg(&cache_path);
                    let _ = optimize_png(&cache_path, &cache_path);
                }
                // always copy the image in from cache since the
                // output dir is blown away on each iteration
                let output_path = self.config.og_images_dir().join(format!("{}.png", id));
                let _ = std::fs::copy(&cache_path, output_path);
            }
        });
    }

    pub fn output_errors(&self) {
        // self.parsing_errors.iter().for_each(|p| {
        //     if let Err(e) = write_file_with_mkdir(&p.0, &p.1) {
        //         event!(Level::ERROR, "Could not write error file: {}", e);
        //     }
        // });
        // self.missing_ids.iter().for_each(|p| {
        //     let _ = write_file_with_mkdir(&p.0, &p.1);
        // });
    }

    pub fn generate_content_pages(
        &mut self,
        _render_errors: &mut BTreeMap<PathBuf, String>,
    ) -> Vec<(PathBuf, PathBuf, String)> {
        let outputs: Vec<(PathBuf, PathBuf, String)> = vec![];
        let mut env = Environment::new();
        env.set_debug(true);
        env.add_function("highlight_code", highlight_code);
        let site_obj = Value::from_serialize(self.clone());
        // let helper_struct = Helpers {};
        // let helper_obj = Value::from_object(helper_struct);
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        env.set_trim_blocks(true);
        env.set_lstrip_blocks(true);
        self.templates
            .iter()
            .for_each(|t| env.add_template_owned(t.0, t.1).unwrap());
        self.pages.iter().for_each(|p| {
            if p.1.do_render {
                let template_name = "pages/post/published.neoj";
                if let Ok(tmpl) = env.get_template(template_name) {
                    match tmpl.render(context!(
                        site => site_obj,
                        // helpers => helper_obj,
                        page_id => p.0
                    )) {
                        Ok(_output) => {
                            //outputs.push((
                            //p.1.full_cache_path.clone().unwrap(),
                            //p.1.full_output_path.clone().unwrap(),
                            //output,
                            //));
                            // dbg!(&p.1.rel_output_path);
                            //outputs.insert(p.0.to_string(), p.clone());
                        }
                        Err(_e) => {
                            // event!(Level::ERROR, "{}\n{:?}", p.1.source_path.display(), e);
                            //render_errors.insert(p.0, format!("{:?}", e));
                            ()
                        }
                    }
                } else {
                    event!(Level::ERROR, "Could not get template: {}", template_name);
                }
            }
        });
        outputs
    }

    #[instrument(skip(self, page_errors))]
    pub fn parse_pages(&mut self, page_errors: &mut Vec<Page>) {
        event!(Level::INFO, "Parsing Pages");
        self.pages = BTreeMap::new(); // make sure templates are cleared
        self.source_files.iter().for_each(|f| {
            let p = Page::new(f.1.clone(), f.0.clone(), &self.config);
            if let Some(_) = p.error.clone() {
                page_errors.push(p);
            } else {
                self.pages.insert(p.id.clone().unwrap(), p);
            }
        });
    }

    pub fn parse_template_tests(&mut self) {
        self.template_test_files.iter().for_each(|f| {
            let p = Page::new(f.1.clone(), f.0.clone(), &self.config);
            if let Some(_) = p.error.clone() {
                self.template_test_page_errors.push(p);
            } else {
                self.template_test_pages.insert(p.id.clone().unwrap(), p);
            }
        });
    }

    pub fn find_template_errors(&mut self) -> Vec<TemplateTest> {
        let mut outputs = vec![];
        let mut env = Environment::new();
        env.set_debug(true);
        env.add_function("highlight_code", highlight_code);
        let site_obj = Value::from_serialize(&self.clone());
        // let helper_struct = Helpers {};
        // let helper_obj = Value::from_object(helper_struct);
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        env.set_trim_blocks(true);
        env.set_lstrip_blocks(true);
        self.templates
            .iter()
            .for_each(|t| env.add_template_owned(t.0, t.1).unwrap());
        self.pages.iter().for_each(|p| {
            let template_name = "pages/post/published.neoj";
            if let Ok(tmpl) = env.get_template(template_name) {
                match tmpl.render(context!(
                     site => site_obj,
                    //helpers => helper_obj,
                    page_id => p.0
                )) {
                    Ok(output) => {
                        let tt = TemplateTest::new(p.1.clone(), Some(output.clone()), None);
                        if tt.template_errors.len() > 0 {
                            outputs.push(tt);
                        }
                        // match tt.status() {
                        //     TemplateTestStatus::Failed => {
                        //         outputs.push(tt);
                        //     }
                        //     _ => (),
                        // }
                    }
                    Err(e) => {
                        dbg!(&e);
                        let tt = TemplateTest::new(p.1.clone(), None, Some(format!("{:?}", e)));
                        outputs.push(tt);
                        //dbg!(p.0);
                        //event!(Level::ERROR, "{}\n{:?}", p.1.source_path.display(), e);
                        //self.template_test_render_errors
                        //   .insert(p.1.source_path.clone(), format!("{:?}", e));
                    }
                }
            } else {
                event!(Level::ERROR, "Could not get template: {}", template_name);
            }
        });
        outputs
    }

    //match ast(f.1, &self.config.sections) {
    //   Ok(ast) => {
    //  },
    //     Err(e) => {
    //}

    // if let Some(id) = get_page_id(&ast) {
    //     let mut output_path =
    //         self.config.paths.get("output_root").unwrap().to_path_buf();
    //     output_path.push(self.config.default_language.clone());
    //     output_path.push(id.clone());
    //     output_path.push("index.html");
    //     let page = Page {
    //         ast,
    //         id: id.clone(),
    //         source_path: f.0.clone(),
    //         output_path,
    //     };
    //     let _ = self.pages.insert(id, page).is_none();
    // } else {
    //     let _ = self
    //         .missing_ids
    //         .insert(
    //             error_file_path,
    //             format!("Missing ID: \n\n{}", f.1.to_string()),
    //         )
    //         .is_none();
    // }
    // }
    // Err(e) => {
    // self.parsing_errors.insert(error_file_path, e.to_string());
    // }

    // };

    // pub fn parse_pages(&mut self) {
    //     self.source_files.iter().for_each(|f| {
    //         let error_file_path = replace_path(
    //             &f.0,
    //             &self.config.paths.get("content_root").unwrap(),
    //             &self.config.paths.get("errors_root").unwrap(),
    //         )
    //         .unwrap()
    //         .with_extension("txt");
    //         match ast(f.1, &self.config.sections) {
    //             Ok(ast) => {
    //                 if let Some(id) = get_page_id(&ast) {
    //                     let mut output_path =
    //                         self.config.paths.get("output_root").unwrap().to_path_buf();
    //                     output_path.push(self.config.default_language.clone());
    //                     output_path.push(id.clone());
    //                     output_path.push("index.html");
    //                     let page = Page {
    //                         ast,
    //                         id: id.clone(),
    //                         source_path: f.0.clone(),
    //                         output_path,
    //                     };
    //                     let _ = self.pages.insert(id, page).is_none();
    //                 } else {
    //                     let _ = self
    //                         .missing_ids
    //                         .insert(
    //                             error_file_path,
    //                             format!("Missing ID: \n\n{}", f.1.to_string()),
    //                         )
    //                         .is_none();
    //                 }
    //             }
    //             Err(e) => {
    //                 self.parsing_errors.insert(error_file_path, e.to_string());
    //             }
    //         };
    //     });
    // }

    #[instrument(skip(self))]
    pub fn load_source_files(&mut self) {
        event!(Level::INFO, "Loading Source Files");
        self.source_files = BTreeMap::new(); // do this to clear the templates
        let dir = &self.config.content_dir();
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
                            self.source_files.insert(path, content);
                        }
                        Err(e) => {
                            event!(Level::ERROR, "{}", e)
                        }
                    }
                });
        } else {
            event!(
                Level::ERROR,
                "Direcotory does not exist: {}",
                &dir.display()
            );
        }
    }

    #[instrument(skip(self))]
    pub fn load_template_test_files(&mut self) {
        let mut dir = self.config.theme_dir();
        dir.push("tests/content");
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
                            self.source_files.insert(path, content);
                        }
                        Err(e) => {
                            event!(Level::ERROR, "{}", e)
                        }
                    }
                });
        } else {
            event!(
                Level::ERROR,
                "Direcotory does not exist: {}",
                &dir.display()
            );
        }
    }

    pub fn load_template_test_template(&mut self) {
        // This is designed to overwrite pages/post/published.neoj
        // for a basic output that can be used to test templates
        let template_as_string = String::from(
            r#"
[!- import "includes/theme.neoj" as theme -!]
[! for section in site.pages[page_id].ast !]
[@ theme.output_section(site, page_id, section) @]
[! endfor !]"#,
        );
        self.templates
            .insert("pages/post/published.neoj".to_string(), template_as_string);
    }

    #[instrument(skip(self))]
    pub fn load_templates(&mut self) {
        event!(Level::INFO, "Loading Templates");
        // let mut templates_root = self.config.paths.get("themes_root").unwrap().to_path_buf();
        // templates_root.push(self.config.theme.name.clone());
        // templates_root.push("templates");
        if self.config.templates_dir().exists() {
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
                            let template_name =
                                path.strip_prefix(self.config.templates_dir()).unwrap();
                            self.templates
                                .insert(template_name.to_string_lossy().to_string(), content);
                        }
                        Err(e) => {
                            event!(Level::ERROR, "{}", e)
                        }
                    }
                });
        } else {
            event!(
                Level::ERROR,
                "Direcotory does not exist: {}",
                self.config.templates_dir().display()
            );
        }
    }

    pub fn set_page_paths(&mut self) {
        self.pages.iter_mut().for_each(|p| {
            if let Some(rel_output_path) = p.1.rel_output_path.clone() {
                p.1.full_cache_path = Some(
                    self.config
                        .page_cache_dir()
                        .join(rel_output_path.strip_prefix("/").unwrap()),
                );
                p.1.full_output_path = Some(
                    self.config
                        .output_dir()
                        .join(rel_output_path.strip_prefix("/").unwrap()),
                );
            };
        });
    }

    pub fn toggle_cached_files(&mut self) {
        self.pages.iter_mut().for_each(|p| {
            if let Some(cache_path) = &p.1.full_cache_path {
                p.1.do_render = is_cache_stale(&p.1.source_path, &cache_path);
            }
        })
    }

    //
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

pub fn optimize_png(input: &PathBuf, output: &PathBuf) -> Result<()> {
    let decoder = Decoder::from_path(input)?;
    let image = decoder.decode()?;
    let config = EncoderConfig::new(Codec::OxiPng);
    let file = File::create(&output)?;
    let encoder = Encoder::new(file, DynamicImage::ImageRgba8(image.into())).with_config(config);
    encoder.encode()?;
    Ok(())
}

pub fn is_cache_stale(_source_file: &PathBuf, _cache_file: &PathBuf) -> bool {
    // This is deprecated in favor of the builder and will
    // be removed when site_v2 is in place
    true
}
