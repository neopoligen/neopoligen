use crate::page::Page;
use crate::site_config::SiteConfig;
use crate::template_test::*;
use fs_extra::dir::copy;
use minijinja::context;
use minijinja::syntax::SyntaxConfig;
use minijinja::Environment;
use minijinja::Value;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub config: SiteConfig,
    pub page_errors: Vec<Page>,
    pub render_errors: BTreeMap<PathBuf, String>,
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
            page_errors: vec![],
            render_errors: BTreeMap::new(),
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
    //

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

        // let mut options = fs_extra::dir::CopyOptions::new();
        // options.overwrite = true;
        // options.content_only = true;

        // let in_dir = self.config.theme_dir().join(PathBuf::from("files"));
        // let out_dir = self.config.output_dir().join(PathBuf::from("theme"));
        // // let out_dir = PathBuf::from("/Users/alan/Desktop/_tmp_theme_output");

        //let mut in_dir = self.config.paths.get("theme_root").unwrap().to_path_buf();
        //in_dir.push("files");

        //let mut out_dir = self.config.paths.get("output_root").unwrap().to_path_buf();
        //out_dir.push("theme");

        // dbg!(&in_dir);
        // dbg!(&out_dir);

        // use fs::copy;
        // let tmp_source = PathBuf::from("/Users/alan/Documents/Neopoligen/v0.1.0-dev/themes/neopoligen-v0.1.0/files/styles/main.css");
        // let mut tmp_dest =
        //     PathBuf::from("/Users/alan/Documents/Neopoligen/v0.1.0-dev/docs/theme/styles");
        // let mut tmp_dest = PathBuf::from("/Users/alan/Desktop/tmp-output");
        // fs::create_dir_all(&tmp_dest).unwrap();
        // tmp_dest.push("main.css");
        // copy(tmp_source, tmp_dest).unwrap();

        // match copy(&in_dir, &out_dir, &options) {
        //     Ok(_) => (),
        //     Err(e) => println!("{:?}", e),
        // }

        //
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

    pub fn generate_content_pages(&mut self) -> BTreeMap<PathBuf, String> {
        let mut outputs = BTreeMap::new();
        let mut env = Environment::new();
        env.set_debug(true);
        let site_obj = Value::from_serialize(&self.clone());
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
                    page_id => p.0
                )) {
                    Ok(output) => {
                        // dbg!(&p.1.rel_output_path);
                        outputs.insert(p.1.rel_output_path.clone().unwrap(), output.clone());
                    }
                    Err(e) => {
                        // event!(Level::ERROR, "{}\n{:?}", p.1.source_path.display(), e);
                        self.render_errors
                            .insert(p.1.source_path.clone(), format!("{:?}", e));
                        ()
                    }
                }
            } else {
                event!(Level::ERROR, "Could not get template: {}", template_name);
            }
            //
        });
        outputs
    }

    pub fn parse_pages(&mut self) {
        self.page_errors = vec![]; // make sure templates are cleared
        self.pages = BTreeMap::new(); // make sure templates are cleared
        self.source_files.iter().for_each(|f| {
            let p = Page::new(f.1.clone(), f.0.clone(), &self.config);
            if let Some(_) = p.error.clone() {
                self.page_errors.push(p);
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
        let site_obj = Value::from_serialize(&self.clone());
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

    #[instrument]
    pub fn load_source_files(&mut self) {
        self.source_files = BTreeMap::new(); // do this to clear the templates
        let dir = &self.config.paths.get("content_root").unwrap();
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
        let mut dir = self.config.paths.get("theme_root").unwrap().clone();
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

    pub fn load_templates(&mut self) {
        let mut templates_root = self.config.paths.get("themes_root").unwrap().to_path_buf();
        templates_root.push(self.config.theme.name.clone());
        templates_root.push("templates");
        if templates_root.exists() {
            WalkDir::new(templates_root.clone())
                .into_iter()
                .filter(|entry| match entry.as_ref().unwrap().path().extension() {
                    Some(ext) => ext.to_str().unwrap() == "neoj",
                    None => false,
                })
                .for_each(|entry| {
                    let path = entry.as_ref().unwrap().path().to_path_buf();
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            let template_name = path.strip_prefix(templates_root.clone()).unwrap();
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
                &templates_root.display()
            );
        }
    }

    //
}
