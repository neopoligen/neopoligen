#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod inits;

use crate::error::page_error::PageError;
use crate::error::template_load_error::TemplateLoadError;
use crate::helpers::*;
use crate::neo_error::NeoErrorKind;
use crate::neo_error_v42::NeoErrorV42;
use crate::page_v42::PageV42b;
use crate::site_v42::b::SiteV42b;
use crate::{
    neo_error::NeoError, page_v42::PageV42, site_config::SiteConfigV42, site_v42::SiteV42,
};
use anyhow::Result;
use chrono::Local;
use chrono::{DateTime, TimeZone};
use html_escape::encode_safe;
use include_dir::{include_dir, Dir};
use minijinja::syntax::SyntaxConfig;
use minijinja::Environment;
use minijinja::{context, Value};
use rusqlite::ffi::sqlite3_table_column_metadata;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

pub struct BuilderV42<'a> {
    pub env: Environment<'a>,
    // TODO: Deprecate, errors and errors_v42 in favor of
    // page_errors and template_load_errors (and anything
    // else that needs to be created)
    pub errors: Vec<NeoError>,
    pub errors_v42: Vec<NeoErrorV42>,
    pub generated_pages: BTreeMap<PathBuf, PageV42>,
    pub page_errors: Vec<NeoErrorV42>,
    // TODO: Deprecate pages in favor of pages_v42
    pub pages: Vec<PageV42>,
    pub pages_v42: BTreeMap<PathBuf, PageV42b>,
    pub site_config: SiteConfigV42,
    // TODO: Deprecate in favor of StiV42b
    pub site: Option<SiteV42>,
    pub site_v42b: Option<SiteV42b>,
    pub templates: BTreeMap<String, String>,
    pub template_load_errors: Vec<NeoErrorV42>,
}

impl BuilderV42<'_> {
    // #[instrument(skip(self))]
    // pub fn add_pages_to_site(&mut self) -> Result<()> {
    //     for (_, page) in &mut self.pages_v42 {
    //         match &page.id {
    //             Some(id) => {
    //                 self.site_v42b.pages.insert(id.to_string(), page.clone());
    //                 dbg!(&id);
    //                 ()
    //             }
    //             None => {
    //                 dbg!("noid");
    //                 ()
    //             }
    //         }
    //         // page.generate_href_from_ast();
    //     }
    //     Ok(())
    // }

    #[instrument(skip(self))]
    pub fn empty_output_dirs(&self) {
        event!(Level::DEBUG, "Emptying output dirs");
        let _ = empty_dir(&self.site_config.status_dest_dir());
        let _ = empty_dir(&self.site_config.output_dest_dir());
    }

    #[instrument(skip(self))]
    pub fn generate_asts(&mut self) -> Result<()> {
        for (_, page) in &mut self.pages_v42 {
            page.generate_ast(&self.site_config);
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_fallback_hrefs(&mut self) -> Result<()> {
        for (_, page) in &mut self.pages_v42 {
            page.generate_fallback_href();
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_fallback_ids(&mut self) -> Result<()> {
        for (_, page) in &mut self.pages_v42 {
            page.generate_fallback_id();
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_hrefs_from_ast(&mut self) -> Result<()> {
        for (_, page) in &mut self.pages_v42 {
            page.generate_href_from_ast();
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_ids_from_ast(&mut self) -> Result<()> {
        for (_, page) in &mut self.pages_v42 {
            page.generate_id_from_ast();
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_outputs(&mut self) -> Result<()> {
        let site_clone = Value::from_object(self.site_v42b.clone().unwrap());
        for (_, page) in &mut self.pages_v42 {
            if let Some(href) = &page.href {
                match self.env.get_template("built-in/backup-page-template.neoj") {
                    Ok(template) => {
                        match template.render(
                            context!(page => Value::from_object(page.clone()), site => site_clone),
                        ) {
                            Ok(output) => {
                                page.output = Some(output);
                                ()
                            }
                            Err(e) => {
                                // eprintln!();
                                // eprintln!("caused by: {:#}", &e);
                                // let mut err = &e as &dyn std::error::Error;
                                // while let Some(next_err) = err.source() {
                                //     eprintln!();
                                //     eprintln!("caused by: {:#}", next_err);
                                //     err = next_err;
                                // }

                                let err = NeoErrorV42::PageError {
                                    id: page.id.clone(),
                                    href: page.href.clone(),
                                    rel_path: page.source_path.clone(),
                                    parts: get_minijinja_error_text(&e), //message: Some(e.to_string()),
                                                                         //details: Some(e.display_debug_info().to_string()),
                                };
                                page.issues.push(err);
                                dbg!("TODO: Alfa Hoist this error from generate_outputs");
                                ()
                            }
                        }
                    }
                    Err(e) => {
                        dbg!("TODO: Bravo Hoist this error from generate_outputs");
                        ()
                    }
                }
            } else {
                dbg!("TODO: ERROR For no href for page");
            }
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_site(&mut self) -> Result<()> {
        let site = SiteV42b::new_with_pages(&self.site_config, &self.pages_v42);
        self.site_v42b = Some(site);
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn get_minijinja_environment(&mut self) -> Environment {
        event!(Level::INFO, "Making Template Environment");
        let mut env = Environment::new();
        // TODO: Deprecate this flatten_spans and move it to
        // the site struct/object
        env.add_function("flatten_spans", flatten_spans_in_template_v42);
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        const BUILT_IN_TEMPLATE_DIR: Dir = include_dir!("src/built_in_templates");
        let built_in_tempates_list = vec![
            "css-output",
            "generic-basic",
            "generic-css",
            "generic-html",
            "generic-json5",
            "generic-list-item-paragraph",
            "generic-list-item-single",
            "generic-list-item",
            "generic-list",
            "generic-raw",
            "generic-script",
            "helpers",
            "pages-problems-index-page",
            "script-output",
            "section-tree",
            "span-text",
            "template-error",
        ];
        built_in_tempates_list.iter().for_each(|name| {
            let data = BUILT_IN_TEMPLATE_DIR.get_file(format!("{}.neoj", name));
            match env.add_template_owned(
                format!("built-in/{}.neoj", name),
                data.expect(format!("Could not open built-in include: {}", name).as_str())
                    .contents_utf8()
                    .unwrap()
                    .to_string(),
            ) {
                Ok(()) => (),
                Err(e) => {
                    self.template_load_errors
                        .push(NeoErrorV42::TemplateLoadError {
                            message: e.to_string(),
                            details: e.display_debug_info().to_string(),
                        });
                    event!(Level::ERROR, "{}", e.display_debug_info().to_string());
                    ()
                }
            }
        });
        self.templates
            .iter()
            .for_each(|(k, v)| match env.add_template_owned(k, v) {
                Ok(()) => (),
                Err(e) => {
                    self.template_load_errors
                        .push(NeoErrorV42::TemplateLoadError {
                            message: e.to_string(),
                            details: e.display_debug_info().to_string(),
                        });
                    event!(Level::ERROR, "{}", e.display_debug_info().to_string());
                    ()
                }
            });
        env
    }

    #[instrument(skip(self))]
    pub fn ingest_source_files(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Content From Files");
        for entry in WalkDir::new(&self.site_config.content_source_dir()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neo"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        match fs::read_to_string(path.clone()) {
                            Ok(source) => {
                                let page = PageV42b::new_from_str_v42(&path, &source);
                                self.pages_v42.insert(path.clone(), page);
                                ()
                            }
                            Err(e) => {
                                // TODO: Output to main error page if there's problems
                                // reading files
                                dbg!(e);
                                ()
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_built_in_templates(&mut self) -> Result<()> {
        const BUILT_IN_TEMPLATE_DIR: Dir = include_dir!("src/built_in_templates");
        let built_in_tempates_list = vec![
            "css-output",
            "generic-basic",
            "generic-css",
            "generic-html",
            "generic-json5",
            "generic-list-item-paragraph",
            "generic-list-item-single",
            "generic-list-item",
            "generic-list",
            "generic-raw",
            "generic-script",
            "helpers",
            "pages-problems-index-page",
            "script-output",
            "section-tree",
            "span-text",
            "template-error",
        ];
        built_in_tempates_list.iter().for_each(|name| {
            let data = BUILT_IN_TEMPLATE_DIR.get_file(format!("{}.neoj", name));
            match self.env.add_template_owned(
                format!("built-in/{}.neoj", name),
                data.expect(format!("Could not open built-in template: {}", name).as_str())
                    .contents_utf8()
                    .unwrap()
                    .to_string(),
            ) {
                Ok(()) => (),
                Err(e) => {
                    event!(
                        Level::ERROR,
                        "{}\n{}",
                        e.to_string(),
                        e.display_debug_info().to_string()
                    );
                    self.template_load_errors
                        .push(NeoErrorV42::TemplateLoadError {
                            message: e.to_string(),
                            details: e.display_debug_info().to_string(),
                        });
                    ()
                }
            }
        });
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_file_system_templates(&mut self) -> Result<()> {
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
                        match self.env.add_template_owned(
                            template_name.clone().unwrap().display().to_string(),
                            content,
                        ) {
                            Ok(()) => (),
                            Err(e) => {
                                event!(
                                    Level::ERROR,
                                    "{}\n{}",
                                    e.to_string(),
                                    e.display_debug_info().to_string()
                                );
                                self.template_load_errors
                                    .push(NeoErrorV42::TemplateLoadError {
                                        message: e.to_string(),
                                        details: e.display_debug_info().to_string(),
                                    });
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // TODO: Deprecate in favor of load_source_from_files
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

    #[instrument(skip(self))]
    pub fn load_templates(&mut self) -> Result<()> {
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
    pub fn output_issues_index_page(&self) -> Result<()> {
        // let env = &self.get_minijinja_environment();
        // self.template_load_errors = template_load_errors;
        match self
            .env
            .get_template("built-in/pages-problems-index-page.neoj")
        {
            Ok(template) => {
                match template.render(context!(
                    site => Value::from_object(self.site_v42b.clone().unwrap()),
                    // page_errors => self.page_errors.clone(),
                     // template_load_errors => self.template_load_errors.clone(),
                )) {
                    Ok(output) => {
                        match write_file_with_mkdir(
                            &self.site_config.errors_index_page(),
                            output.as_str(),
                        ) {
                            // TODO: Make dynamic page error here
                            _ => (),
                        }
                    }
                    Err(e) => {
                        match write_file_with_mkdir(
                            &self.site_config.errors_index_page(),
                            format!(
                                r#"<!DOCTYPE><html><head>
<style>body {{ background-color: #200; color: #ffa; }}</style>
</head><body><p>ERRORS: {}</p><pre>{}</pre></body></html>"#,
                                e.to_string(),
                                encode_safe(&e.display_debug_info().to_string().as_str())
                            )
                            .as_str(),
                        ) {
                            // TODO: Make dynamic page error here
                            _ => (),
                        }
                    }
                }
            }
            Err(e) => {
                match write_file_with_mkdir(
                    &self.site_config.errors_index_page(),
                    format!(
                        r#"<!DOCTYPE><html><head>
<style>body {{ background-color: #200; color: #ffa; }}</style>
</head><body><p>ERRORS: {}</p><pre>{}</pre></body></html>"#,
                        e.to_string(),
                        encode_safe(&e.display_debug_info().to_string().as_str())
                    )
                    .as_str(),
                ) {
                    // TODO: Make dynamic page error here
                    _ => (),
                }
            }
        }
        Ok(())
    }

    // let skeleton = env.get_template("hello").unwrap();
    // let output = skeleton.render(context!(name => "World")).unwrap();
    // println!("{}", output);

    #[instrument(skip(self))]
    pub fn output_issue_pages(&mut self) -> Result<()> {
        event!(Level::INFO, "Making Issue Pages");
        let site_clone = Value::from_object(self.site_v42b.clone().unwrap());
        for (_, page) in &mut self.pages_v42 {
            if page.issues.len() > 0 {
                if let Some(href) = &page.href {
                    let rel_output_path = match href.strip_prefix("/") {
                        Ok(p) => p,
                        Err(_) => &href,
                    };
                    let absolute_output_path =
                        &self.site_config.output_dest_dir().join(rel_output_path);
                    match self.env.get_template("built-in/pages-issue-page.neoj") {
                        Ok(template) => match template.render(context!(
                            site => site_clone,
                            page => Value::from_object(page.clone()),
                        )) {
                            Ok(output) => {
                                match write_file_with_mkdir(&absolute_output_path, &output) {
                                    Ok(()) => {
                                        //event!(Level::INFO, "Making file in output_pages",);
                                    }
                                    Err(_e) => {
                                        dbg!("TODO: Hoist this error in output_page_v42");
                                        ()
                                    }
                                }
                            }
                            Err(e) => {
                                dbg!(e);
                                ()
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            ()
                        }
                    }
                }
                ()
            }

            // if let (None, Some(errors)) = (&page.output, &page.errors) {
            //     dbg!(errors);
            // }
        }

        Ok(())
    }

    #[instrument(skip(self))]
    pub fn output_pages_v42(&mut self) -> Result<()> {
        event!(Level::INFO, "Outputting pages");
        for (_, page) in &mut self.pages_v42 {
            if let Some(href) = &page.href {
                let rel_output_path = match href.strip_prefix("/") {
                    Ok(p) => p,
                    Err(_) => &href,
                };
                let absolute_output_path =
                    &self.site_config.output_dest_dir().join(rel_output_path);
                if let Some(output) = &page.output {
                    match write_file_with_mkdir(&absolute_output_path, output) {
                        Ok(()) => {
                            //event!(Level::INFO, "Making file in output_pages",);
                        }
                        Err(_e) => {
                            dbg!("TODO: Hoist this error in output_page_v42");
                            ()
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // TODO: Deprecate in favor of output_pages_v42
    #[instrument(skip(self))]
    pub fn output_pages(&mut self) -> Result<()> {
        event!(Level::INFO, "Outputting pages");

        //site_clone2.pages.iter().for_each(|(_id, page)| {
        //    let builder_error: Option<NeoError>;
        //    //let base_rel_path = PathBuf::from(self.site_config.default_language().unwrap());
        //    let file_rel_path = page.html_output_path.clone();
        //    let absolute_output_path = site_config
        //        .project_dir()
        //        .join(&site_config.output_directory_name)
        //        .join(file_rel_path.unwrap());
        //    if let Some(template) = page.template_list().iter().find_map(|name| {
        //        if let Ok(tmpl) = env.get_template(name) {
        //            Some(tmpl)
        //        } else {
        //            None
        //        }
        //    }) {
        //        match template.render(context!(
        //            site => &site_clone,
        //            page => Value::from_object(page.clone())
        //        )) {
        //            Ok(output) => {
        //                match write_file_with_mkdir(&absolute_output_path, output.as_str()) {
        //                    Ok(()) => {
        //                        //event!(Level::INFO, "Making file in output_pages",);
        //                    }
        //                    Err(e) => self.errors.push({
        //                        event!(
        //                            Level::ERROR,
        //                            "Could not write file: {}",
        //                            absolute_output_path.display()
        //                        );
        //                        NeoError {
        //                            kind: NeoErrorKind::GenericErrorWithSourcePath {
        //                                source_path: absolute_output_path.to_path_buf(),
        //                                msg: e,
        //                            },
        //                        }
        //                    }),
        //                }
        //            }
        //            Err(e) => {
        //                self.errors_v42.push(NeoErrorV42::FullPathWith {
        //                    path: page.fs_path.clone(),
        //                    err: e.display_debug_info().to_string(),
        //                });
        //                // if let Ok(tmpl404) = env.get_template("axum-404") {
        //                //     match tmpl404.render(context!(
        //                //         site => &site_clone,
        //                //         page => Value::from_object(page.clone()),
        //                //         ast => page.raw_ast.clone(),
        //                //         error => e.display_debug_info().to_string(),
        //                //         template_load_errors => &self.template_load_errors.clone(),
        //                //     )) {
        //                //         Ok(output) => {
        //                //             let _ = write_file_with_mkdir(
        //                //                 &self.site_config.error_page_404(),
        //                //                 output.as_str(),
        //                //             );
        //                //         }
        //                //         Err(_) => {}
        //                //     }
        //                // }
        //                event!(
        //                    Level::ERROR,
        //                    "Could not render output for: {} - {} - {}",
        //                    page.fs_path.display(),
        //                    e.to_string(),
        //                    e.display_debug_info(),
        //                );
        //                builder_error = Some(NeoError {
        //                    kind: NeoErrorKind::GenericErrorWithSourcePath {
        //                        source_path: absolute_output_path.to_path_buf(),
        //                        msg: e.to_string(),
        //                    },
        //                });
        //                if let Ok(template) = env.get_template("built-in-template-error") {
        //                    dbg!("1111111111111111111111111111111");
        //                    match template.render(context!(
        //                        site => &site_clone,
        //                        page => Value::from_object(page.clone()),
        //                        builder_error => Value::from_serialize(builder_error),
        //                    )) {
        //                        Ok(output) => {
        //                            dbg!("222222222222222222222222222");
        //                            match write_file_with_mkdir(
        //                                &absolute_output_path,
        //                                output.as_str(),
        //                            ) {
        //                                Ok(()) => {
        //                                    dbg!("333333333333333333333333333333333");
        //                                    dbg!(&absolute_output_path);
        //                                    ()
        //                                    //event!(Level::INFO, "Making file in output_pages",);
        //                                }
        //                                Err(e) => self.errors.push({
        //                                    dbg!("444444444444444444444444444444");
        //                                    event!(
        //                                        Level::ERROR,
        //                                        "Could not write file: {}",
        //                                        absolute_output_path.display()
        //                                    );
        //                                    NeoError {
        //                                        kind: NeoErrorKind::GenericErrorWithSourcePath {
        //                                            source_path: absolute_output_path.to_path_buf(),
        //                                            msg: e,
        //                                        },
        //                                    }
        //                                }),
        //                            }
        //                        }
        //                        Err(e) => {
        //                            dbg!(e);
        //                            // page.error = Some(NeoError {
        //                            //     kind: NeoErrorKind::GenericErrorWithSourcePath {
        //                            //         source_path: absolute_output_path.to_path_buf(),
        //                            //         msg: e.to_string(),
        //                            //     },
        //                            // });
        //                        }
        //                    }
        //                }
        //                self.errors.push(NeoError {
        //                    kind: NeoErrorKind::GenericErrorWithSourcePath {
        //                        source_path: page.fs_path.clone(),
        //                        msg: e.to_string(),
        //                    },
        //                });
        //            }
        //        }
        //    } else {
        //        event!(
        //            Level::ERROR,
        //            "Could not find template for: {} - Missing: {}",
        //            page.fs_path.display(),
        //            page.template_list().join(",")
        //        );
        //        self.errors.push(NeoError {
        //            kind: NeoErrorKind::GenericErrorWithSourcePath {
        //                source_path: page.fs_path.clone(),
        //                msg: format!(
        //                    "Could not templates for page. {}",
        //                    page.template_list().join(",")
        //                ),
        //            },
        //        });
        //    }
        //});

        Ok(())
    }

    #[instrument(skip(self, site_config))]
    pub fn prep_site(&mut self, site_config: &SiteConfigV42) -> Result<()> {
        event!(Level::INFO, "Prepping Site");
        let mut site = SiteV42 {
            config: site_config.clone(),
            pages: BTreeMap::new(),
            // generated_at: Some(chrono::prelude::Local::now()),
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

    #[instrument(skip(self))]
    pub fn set_languages_from_site_config(&mut self) -> Result<()> {
        for (_, page) in &mut self.pages_v42 {
            page.set_language_from_site_config(&self.site_config);
        }
        Ok(())
    }

    //
}
