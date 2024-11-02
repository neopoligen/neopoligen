pub mod inits;

use crate::helpers::*;
use crate::neo_error::{NeoErrorKind, NeoErrorV42};
use crate::{
    neo_error::NeoError, page_v42::PageV42, site_config::SiteConfigV42, site_v42::SiteV42,
};
use anyhow::Result;
use minijinja::syntax::SyntaxConfig;
use minijinja::Environment;
use minijinja::{context, Value};
use std::collections::BTreeMap;
use std::fs;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

pub struct BuilderV42 {
    pub errors: Vec<NeoError>,
    pub errors_v42: Vec<NeoErrorV42>,
    pub pages: Vec<PageV42>,
    pub site_config: SiteConfigV42,
    pub site: Option<SiteV42>,
    pub templates: BTreeMap<String, String>,
}

impl BuilderV42 {
    #[instrument(skip(self))]
    pub fn empty_output_dirs(&self) {
        event!(Level::DEBUG, "Emptying output dirs");
        let _ = empty_dir(&self.site_config.status_dest_dir());
        let _ = empty_dir(&self.site_config.output_dest_dir());
    }

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
    pub fn output_errors(&mut self) -> Result<()> {
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn output_pages(&mut self) -> Result<()> {
        event!(Level::INFO, "Outputting pages");
        let mut env = Environment::new();
        env.add_function("flatten_spans", flatten_spans_in_template_v42);
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
        let _ =
            env.add_template_owned("axum-404", include_str!("built-in-templates/axum-404.neoj"));
        let _ = env.add_template_owned(
            "built-in-generic-basic",
            include_str!("built-in-templates/generic-basic.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-generic-css",
            include_str!("built-in-templates/generic-css.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-generic-html",
            include_str!("built-in-templates/generic-html.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-generic-json5",
            include_str!("built-in-templates/generic-json5.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-generic-list",
            include_str!("built-in-templates/generic-list.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-generic-list-item",
            include_str!("built-in-templates/generic-list-item.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-generic-list-item-paragraph",
            include_str!("built-in-templates/generic-list-item-paragraph.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-generic-list-item-single",
            include_str!("built-in-templates/generic-list-item-single.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-generic-raw",
            include_str!("built-in-templates/generic-raw.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-generic-script",
            include_str!("built-in-templates/generic-script.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-css-output",
            include_str!("built-in-templates/css-output.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-helpers",
            include_str!("built-in-templates/helpers.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-script-output",
            include_str!("built-in-templates/script-output.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-span-text",
            include_str!("built-in-templates/span-text.neoj"),
        );
        let _ = env.add_template_owned(
            "built-in-template-error",
            include_str!("built-in-templates/template-error.neoj"),
        );
        let site_clone = Value::from_object(self.site.clone().unwrap());
        self.site
            .as_ref()
            .unwrap()
            .pages
            .iter()
            .for_each(|(_id, page)| {
                let builder_error: Option<NeoError>;
                //let base_rel_path = PathBuf::from(self.site_config.default_language().unwrap());
                let file_rel_path = page.html_output_path.clone();
                let absolute_output_path = &self
                    .site_config
                    .project_dir()
                    .join(&self.site_config.output_directory_name)
                    .join(file_rel_path.unwrap());
                if let Some(template) = page.template_list().iter().find_map(|name| {
                    if let Ok(tmpl) = env.get_template(name) {
                        Some(tmpl)
                    } else {
                        None
                    }
                }) {
                    match template.render(context!(
                        site => &site_clone,
                        page => Value::from_object(page.clone())
                    )) {
                        Ok(output) => {
                            match write_file_with_mkdir(&absolute_output_path, output.as_str()) {
                                Ok(()) => {
                                    //event!(Level::INFO, "Making file in output_pages",);
                                }
                                Err(e) => self.errors.push({
                                    event!(
                                        Level::ERROR,
                                        "Could not write file: {}",
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
                            self.errors_v42.push(NeoErrorV42::FullPathWith {
                                path: page.fs_path.clone(),
                                err: e.display_debug_info().to_string(),
                            });

                            if let Ok(tmpl404) = env.get_template("axum-404") {
                                match tmpl404.render(context!(
                                    site => &site_clone,
                                    page => Value::from_object(page.clone()),
                                    error => e.display_debug_info().to_string()))
                                {
                                    Ok(output) => {
                                        let _ = write_file_with_mkdir(
                                            &self.site_config.error_page_404(),
                                            output.as_str(),
                                        );
                                    }
                                    Err(_) => {}
                                }
                            }

                            // event!(
                            //     Level::ERROR,
                            //     "Could not render output for: {} - {} - {}",
                            //     page.fs_path.display(),
                            //     e.to_string(),
                            //     e.display_debug_info(),
                            // );

                            builder_error = Some(NeoError {
                                kind: NeoErrorKind::GenericErrorWithSourcePath {
                                    source_path: absolute_output_path.to_path_buf(),
                                    msg: e.to_string(),
                                },
                            });

                            if let Ok(template) = env.get_template("built-in-template-error") {
                                dbg!("1111111111111111111111111111111");
                                match template.render(context!(
                                    site => &site_clone,
                                    page => Value::from_object(page.clone()),
                                        builder_error => Value::from_serialize(builder_error),
                                )) {
                                    Ok(output) => {
                                        dbg!("222222222222222222222222222");
                                        match write_file_with_mkdir(
                                            &absolute_output_path,
                                            output.as_str(),
                                        ) {
                                            Ok(()) => {
                                                dbg!("333333333333333333333333333333333");
                                                dbg!(&absolute_output_path);
                                                ()
                                                //event!(Level::INFO, "Making file in output_pages",);
                                            }
                                            Err(e) => self.errors.push({
                                                dbg!("444444444444444444444444444444");
                                                event!(
                                                    Level::ERROR,
                                                    "Could not write file: {}",
                                                    absolute_output_path.display()
                                                );
                                                NeoError {
                                                    kind:
                                                        NeoErrorKind::GenericErrorWithSourcePath {
                                                            source_path: absolute_output_path
                                                                .to_path_buf(),
                                                            msg: e,
                                                        },
                                                }
                                            }),
                                        }
                                    }
                                    Err(e) => {
                                        dbg!(e);
                                        // page.error = Some(NeoError {
                                        //     kind: NeoErrorKind::GenericErrorWithSourcePath {
                                        //         source_path: absolute_output_path.to_path_buf(),
                                        //         msg: e.to_string(),
                                        //     },
                                        // });
                                    }
                                }
                            }
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
