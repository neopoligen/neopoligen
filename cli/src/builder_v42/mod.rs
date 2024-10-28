pub mod inits;

use crate::helpers::write_file_with_mkdir;
use crate::neo_error::NeoErrorKind;
use crate::{neo_error::NeoError, page_v42::PageV42, site::SiteV42, site_config::SiteConfigV42};
use anyhow::Result;
use minijinja::syntax::SyntaxConfig;
use minijinja::Environment;
use minijinja::{context, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

pub struct BuilderV42 {
    pub errors: Vec<NeoError>,
    pub pages: Vec<PageV42>,
    pub site_config: SiteConfigV42,
    pub site: Option<SiteV42>,
    pub templates: BTreeMap<String, String>,
}

impl BuilderV42 {
    #[instrument(skip(self))]
    pub fn load_pages_from_file_system(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Pages From FS");
        for entry in WalkDir::new(&self.site_config.content_source_dir()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neo"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        match PageV42::new_from_fs_path(&path, &self.site_config) {
                            Ok(p) => {
                                //dbg!(&p.ast());
                                //event![Level::INFO, "Added raw page: {}", path.display()];
                                self.pages.push(p)
                            }
                            Err(e) => {
                                // event![Level::ERROR, "Error loading raw page: {}", path.display()];
                                self.errors.push(e)
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[instrument(skip(self, _site_config))]
    pub fn load_templates(&mut self, _site_config: &SiteConfigV42) -> Result<()> {
        event!(Level::INFO, "Loading Templates");
        for entry in WalkDir::new(&self.site_config.theme_dir()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neoj"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        let template_name = &path.strip_prefix(&self.site_config.theme_dir());
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
    pub fn output_pages(&mut self) -> Result<()> {
        event!(Level::INFO, "Outputting pages");
        let mut env = Environment::new();
        // env.add_function("highlight_code", highlight_code);
        // env.add_function("highlight_code_no_nums", highlight_code_no_nums);
        // env.add_function("highlight_span", highlight_span);
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        self.templates.iter().for_each(|(k, v)| {
            let _ = env.add_template_owned(k, v);
        });
        let site_clone = Value::from_object(self.site.clone().unwrap());
        self.site
            .as_ref()
            .unwrap()
            .pages
            .iter()
            .for_each(|(_id, page)| {
                //let base_rel_path = PathBuf::from(self.site_config.default_language().unwrap());
                let file_rel_path = page.html_output_path.clone();
                let absolute_output_path = &self
                    .site_config
                    .project_dir()
                    .join(&self.site_config.output_directory)
                    .join(file_rel_path.unwrap());
                event!(
                    Level::INFO,
                    "Ready to run: {}",
                    absolute_output_path.display()
                );

                if let Some(template) = page.template_list().iter().find_map(|name| {
                    if let Ok(tmpl) = env.get_template(name) {
                        Some(tmpl)
                    } else {
                        None
                    }
                }) {
                    match template.render(context!(
                        //site => &site_clone,
                       // page => Value::from_object(page.clone())
                    )) {
                        Ok(output) => {
                            match write_file_with_mkdir(&absolute_output_path, output.as_str()) {
                                Ok(()) => {
                                    //event!(Level::INFO, "Making file in output_pages",);
                                }
                                Err(e) => self.errors.push({
                                    event!(
                                        Level::ERROR,
                                        "Error on page output in output_pages: {}",
                                        absolute_output_path.display()
                                    );
                                    NeoError {
                                        kind: NeoErrorKind::GenericErrorWithSourcePath {
                                            source_path: absolute_output_path.to_path_buf(),
                                            msg: e,
                                        },
                                    }
                                }),
                            }
                        }
                        Err(e) => {
                            // event!(
                            //     Level::ERROR,
                            //     "Could not render output for: {}",
                            //     page.fs_path.display()
                            // );
                            self.errors.push(NeoError {
                                kind: NeoErrorKind::GenericErrorWithSourcePath {
                                    source_path: page.fs_path.clone(),
                                    msg: e.to_string(),
                                },
                            });
                        }
                    }
                } else {
                    event!(
                        Level::ERROR,
                        "Could not find template for: {} - Missing: {}",
                        page.fs_path.display(),
                        page.template_list().join(",")
                    );
                    self.errors.push(NeoError {
                        kind: NeoErrorKind::GenericErrorWithSourcePath {
                            source_path: page.fs_path.clone(),
                            msg: format!(
                                "Could not templates for page. {}",
                                page.template_list().join(",")
                            ),
                        },
                    });
                }
            });
        Ok(())
    }

    #[instrument(skip(self, site_config))]
    pub fn prep_site(&mut self, site_config: &SiteConfigV42) -> Result<()> {
        event!(Level::INFO, "Prepping Site");
        let mut site = SiteV42 {
            config: site_config.clone(),
            pages: BTreeMap::new(),
        };
        self.pages.iter().for_each(|p| match &p.id {
            Some(id) => {
                site.pages.insert(id.to_string(), p.clone());
                ()
            }
            None => {
                // event!(Level::ERROR, "prep_site Error: {}", e);
                //self.errors.push(e)
            }
        });
        self.site = Some(site);
        Ok(())
    }

    //
}
