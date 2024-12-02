pub mod inits;

use crate::helpers::*;
use crate::image::Image;
use crate::neo_error_v42::NeoErrorV42;
use crate::page_v42::PageV42b;
use crate::site_config::SiteConfigV42;
use crate::site_v42::SiteV42b;
use anyhow::Result;
use html_escape::encode_safe;
use image::io::Reader;
use minijinja::Environment;
use minijinja::{context, Value};
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
    // pub errors: Vec<NeoError>,
    pub errors_v42: Vec<NeoErrorV42>,
    // pub image_maxes: Vec<usize>,
    pub images: BTreeMap<String, Image>,
    // pub generated_pages: BTreeMap<PathBuf, PageV42>,
    pub page_errors: Vec<NeoErrorV42>,
    // TODO: Deprecate pages in favor of pages_v42
    //pub pages: Vec<PageV42>,
    pub pages_v42: BTreeMap<PathBuf, PageV42b>,
    pub site_config: SiteConfigV42,
    // TODO: Deprecate in favor of StiV42b
    // pub site: Option<SiteV42>,
    pub site_v42b: Option<SiteV42b>,
    pub templates: BTreeMap<String, String>,
    pub template_load_errors: Vec<NeoErrorV42>,
}

impl BuilderV42<'_> {
    #[instrument(skip(self))]
    pub fn deploy_theme_files(&self) {
        event!(Level::DEBUG, "Deploying Theme Files");
        let source_folder = self.site_config.theme_files_input_folder();
        let dest_folder = self.site_config.theme_files_output_folder();
        let _ = copy_folder(&source_folder, &dest_folder);
    }

    #[instrument(skip(self))]
    pub fn empty_output_folders(&self) {
        event!(Level::INFO, "Emptying Output Dirs");
        let _ = empty_folder(&self.site_config.status_dest_folder());
        let _ = empty_folder(&self.site_config.site_output_root_folder());
    }

    #[instrument(skip(self))]
    pub fn generate_asts(&mut self) -> Result<()> {
        event!(Level::INFO, "Generating ASTs");
        for (_, page) in &mut self.pages_v42 {
            page.generate_ast(&self.site_config);
        }
        Ok(())
    }

    // #[instrument(skip(self))]
    // pub fn generate_images(&mut self) -> Result<()> {
    //     let source_images = load_source_images(&self.site_config.images_input_root_folder());
    //     dbg!(source_images);
    //     event!(Level::INFO, "Generating Images");
    //     Ok(())
    // }

    #[instrument(skip(self))]
    pub fn get_image(&mut self, stem: &str, path: &PathBuf) -> Option<Image> {
        if let Ok(ts) = file_epoch_mod_time(path) {
            if let Some(ext) = path.extension() {
                match ext.to_ascii_lowercase().as_os_str().to_str() {
                    Some("jpg") => {
                        let _ = &self.images.insert(stem.to_string(), Image::new(path, ts));
                    }
                    _ => (),
                }
            }
        };

        None
        // let _ = &self.images.iter_mut().for_each(|image| {
        //     dbg!("aaaaaaa");
        //     if let Ok(img_reader) = Reader::open(&image.1.source_image_path) {
        //         dbg!("bbbbbbbbbbbbb");
        //         if let Ok(img) = img_reader.decode() {
        //             dbg!("ccccccccccccc");
        //             image.1.raw_height = Some(img.height());
        //         }
        //     }
        // });
        // dbg!(&self.images);
    }

    #[instrument(skip(self))]
    pub fn load_source_images(&mut self) -> Result<()> {
        let walker = WalkDir::new(&self.site_config.images_input_root_folder()).into_iter();
        walker.for_each(|path_result| {
            match path_result {
                Ok(path) => {
                    // if let Ok(ts) = file_epoch_mod_time(&path.clone().into_path()) {
                    if let Some(stem) = path.path().file_stem() {
                        let stem_as_string = stem.to_string_lossy().to_string();
                        match self.get_image(&stem_as_string, &path.path().to_path_buf()) {
                            Some(i) => {
                                let _ = &self.images.insert(stem_as_string, i);
                                ()
                            }
                            _ => (),
                        }

                        //    }
                    };
                }
                Err(_e) => {
                    // TODO: hoist error here
                }
            }
        });

        // walker.filter(|path_result| match path_result {
        //     Ok(path) => match path.path().extension() {
        //         Some(ext) => {
        //             if ext.to_ascii_lowercase().eq("jpg") {
        //                 Some(path.path().to_path_buf())
        //             } else if ext.to_ascii_lowercase().eq("jpeg") {
        //                 Some(path.path().to_path_buf())
        //             } else if ext.to_ascii_lowercase().eq("png") {
        //                 Some(path.path().to_path_buf())
        //             } else {
        //                 None
        //             }
        //         }
        //         None => None,
        //     },
        //     Err(_) => None,
        // });

        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_outputs(&mut self) -> Result<()> {
        event!(Level::INFO, "Generating Outputs");
        let site_clone = Value::from_object(self.site_v42b.clone().unwrap());
        for (_, page) in &mut self.pages_v42 {
            if let Some(_rel_output_path) = &page.rel_output_path {
                match self.env.get_template("pages/post/default.neoj") {
                    Ok(template) => {
                        match template.render(context!(
                            page => Value::from_object(page.clone()),
                            site => &site_clone,
                            sections => &page.sections,
                        )) {
                            Ok(output) => {
                                page.output = Some(output);
                                ()
                            }
                            Err(e) => {
                                let err = NeoErrorV42::PageIssue {
                                    id: page.id.clone(),
                                    rel_output_path: page.rel_output_path.clone(),
                                    notes: get_minijinja_error_text(&e),
                                    source_display_path: page.source_display_path.clone(),
                                    template_list: vec![
                                        "issues/page-issue/default.neoj".to_string()
                                    ],
                                };
                                page.issues.push(err);
                            }
                        }
                    }
                    Err(e) => {
                        let err = NeoErrorV42::PageIssue {
                            id: page.id.clone(),
                            rel_output_path: page.rel_output_path.clone(),
                            notes: get_minijinja_error_text(&e),
                            source_display_path: page.source_display_path.clone(),
                            template_list: vec!["issues/page-issue/default.neoj".to_string()],
                        };
                        page.issues.push(err);
                    }
                }
            } else {
                dbg!("TODO: ERROR For no rel_output_path for page");
            }
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn initialize_site_with_pages(&mut self) -> Result<()> {
        event!(Level::INFO, "Initializing site");
        let site = SiteV42b::new_with_pages(&self.site_config, &self.pages_v42);
        self.site_v42b = Some(site);
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn ingest_source_files(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Content From Files");
        for entry in WalkDir::new(&self.site_config.content_input_root_folder()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neo"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        match fs::read_to_string(path.clone()) {
                            Ok(source) => {
                                let page = PageV42b::new_from_str_v42(
                                    &path,
                                    &source,
                                    &self.site_config.project_folder(),
                                );
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
    pub fn load_file_system_templates(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Templates");
        for entry in WalkDir::new(&self.site_config.theme_input_root_folder()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neoj"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        let template_name =
                            &path.strip_prefix(&self.site_config.theme_input_root_folder());
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

    #[instrument(skip(self))]
    pub fn generate_macro_loader(&mut self) -> Result<()> {
        event!(Level::INFO, "Generating Macro Loader");
        // TODO: Handle error if the macros directory gets deleted
        let mut macros: Vec<String> = vec![];
        for entry in WalkDir::new(&self.site_config.macros_input_root_folder()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neoj"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        macros.push(format!(
                            r#"[!- include "macros/{}" -!]"#,
                            filename.to_string_lossy().to_string()
                        ));
                    }
                }
            }
        }
        let template_name = "built-in/macros.neoj";
        let content = macros.join("\n");
        match self
            .env
            .add_template_owned(template_name.to_string(), content)
        {
            _ => (),
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn move_updated_pages_into_site(&mut self) -> Result<()> {
        let mut pages_with_ids: BTreeMap<String, PageV42b> = BTreeMap::new();
        let mut pages_without_ids: BTreeMap<PathBuf, PageV42b> = BTreeMap::new();
        for (_source_absolute_path, page) in &self.pages_v42 {
            match &page.id {
                Some(id) => pages_with_ids.insert(id.to_string(), page.clone()),
                None => pages_without_ids.insert(page.source_display_path.clone(), page.clone()),
            };
        }
        self.site_v42b.as_mut().unwrap().pages = pages_with_ids;
        self.site_v42b.as_mut().unwrap().pages_without_ids = pages_without_ids;
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn output_issues_index_page(&self) -> Result<()> {
        // let env = &self.get_minijinja_environment();
        // self.template_load_errors = template_load_errors;
        match self
            .env
            .get_template("helpers/issues-home-page/default.neoj")
        {
            Ok(template) => {
                match template.render(context!(
                    site => Value::from_object(self.site_v42b.clone().unwrap()),
                    // page_errors => self.page_errors.clone(),
                     // template_load_errors => self.template_load_errors.clone(),
                )) {
                    Ok(output) => {
                        match write_file_with_mkdir(
                            &self.site_config.errors_output_index_page(),
                            output.as_str(),
                        ) {
                            // TODO: Make dynamic page error here
                            _ => (),
                        }
                    }
                    Err(e) => {
                        match write_file_with_mkdir(
                            &self.site_config.errors_output_index_page(),
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
                    &self.site_config.errors_output_index_page(),
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

    #[instrument(skip(self))]
    pub fn output_issue_pages(&mut self) -> Result<()> {
        event!(Level::INFO, "Making Issue Pages");
        let site_clone = Value::from_object(self.site_v42b.clone().unwrap());
        for (_, page) in &mut self.pages_v42 {
            if page.issues.len() > 0 {
                if let Some(_href) = &page.href {
                    let absolute_output_path = &self
                        .site_config
                        .site_output_root_folder()
                        .join(page.rel_output_path.as_ref().unwrap());
                    match self.env.get_template("helpers/issue-page/default.neoj") {
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
        }

        Ok(())
    }

    #[instrument(skip(self))]
    pub fn output_pages_v42(&mut self) -> Result<()> {
        event!(Level::INFO, "Outputting pages");
        for (_, page) in &mut self.pages_v42 {
            if let Some(rel_output_path) = &page.rel_output_path {
                let absolute_output_path = &self
                    .site_config
                    .site_output_root_folder()
                    .join(rel_output_path);
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

    #[instrument(skip(self))]
    pub fn set_hrefs(&mut self) -> Result<()> {
        event!(Level::INFO, "Setting Page hrefs");
        for (_, page) in &mut self.pages_v42 {
            page.set_href();
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn set_ids(&mut self) -> Result<()> {
        event!(Level::INFO, "Setting Page IDs");
        for (_, page) in &mut self.pages_v42 {
            page.set_id();
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn set_languages_from_site_config(&mut self) -> Result<()> {
        for (_, page) in &mut self.pages_v42 {
            page.set_language_from_site_config(&self.site_config);
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn set_rel_output_paths(&mut self) -> Result<()> {
        event!(Level::INFO, "Setting Page IDs");
        for (_, page) in &mut self.pages_v42 {
            page.set_rel_output_path();
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn set_titles(&mut self) -> Result<()> {
        event!(Level::INFO, "Setting Page Titles");
        for (_, page) in &mut self.pages_v42 {
            page.set_titles();
        }
        Ok(())
    }

    //
}

// #[instrument(skip(self))]
// pub fn load_templates(&mut self) -> Result<()> {
//     event!(Level::INFO, "Loading Templates");
//     for entry in WalkDir::new(&self.site_config.theme_input_root_folder()) {
//         let path = entry?.path().to_path_buf();
//         if path.is_file() {
//             if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
//                 if ext.to_ascii_lowercase() == "neoj"
//                     && !filename.to_str().unwrap().starts_with(".")
//                 {
//                     let template_name = &path.strip_prefix(&self.site_config.theme_input_root_folder());
//                     let content = fs::read_to_string(&path)?;
//                     self.templates.insert(
//                         template_name
//                             .as_ref()
//                             .unwrap()
//                             .to_string_lossy()
//                             .to_string(),
//                         content,
//                     );
//                 }
//             }
//         }
//     }
//     Ok(())
// }

//#[instrument(skip(self, site_config))]
//pub fn prep_site(&mut self, site_config: &SiteConfigV42) -> Result<()> {
//    event!(Level::INFO, "Prepping Site");
//    let mut site = SiteV42 {
//        config: site_config.clone(),
//        pages: BTreeMap::new(),
//        // generated_at: Some(chrono::prelude::Local::now()),
//    };
//    self.pages.iter().for_each(|p| match &p.id {
//        Some(id) => {
//            site.pages.insert(id.to_string(), p.clone());
//            ()
//        }
//        None => {
//            // event!(Level::ERROR, "prep_site Error: {}", e);
//            //self.errors.push(e)
//        }
//    });
//    self.site = Some(site);
//    Ok(())
//}

//// TODO: Deprecate in favor of output_pages_v42
//#[instrument(skip(self))]
//pub fn output_pages(&mut self) -> Result<()> {
//    event!(Level::INFO, "Outputting pages");
//    //site_clone2.pages.iter().for_each(|(_id, page)| {
//    //    let builder_error: Option<NeoError>;
//    //    //let base_rel_path = PathBuf::from(self.site_config.language_default().unwrap());
//    //    let file_rel_path = page.html_output_path.clone();
//    //    let absolute_output_path = site_config
//    //        .project_folder()
//    //        .join(&site_config.output_folder_name)
//    //        .join(file_rel_path.unwrap());
//    //    if let Some(template) = page.template_list().iter().find_map(|name| {
//    //        if let Ok(tmpl) = env.get_template(name) {
//    //            Some(tmpl)
//    //        } else {
//    //            None
//    //        }
//    //    }) {
//    //        match template.render(context!(
//    //            site => &site_clone,
//    //            page => Value::from_object(page.clone())
//    //        )) {
//    //            Ok(output) => {
//    //                match write_file_with_mkdir(&absolute_output_path, output.as_str()) {
//    //                    Ok(()) => {
//    //                        //event!(Level::INFO, "Making file in output_pages",);
//    //                    }
//    //                    Err(e) => self.errors.push({
//    //                        event!(
//    //                            Level::ERROR,
//    //                            "Could not write file: {}",
//    //                            absolute_output_path.display()
//    //                        );
//    //                        NeoError {
//    //                            kind: NeoErrorKind::GenericErrorWithSourcePath {
//    //                                source_path: absolute_output_path.to_path_buf(),
//    //                                msg: e,
//    //                            },
//    //                        }
//    //                    }),
//    //                }
//    //            }
//    //            Err(e) => {
//    //                self.errors_v42.push(NeoErrorV42::FullPathWith {
//    //                    path: page.fs_path.clone(),
//    //                    err: e.display_debug_info().to_string(),
//    //                });
//    //                // if let Ok(tmpl404) = env.get_template("axum-404") {
//    //                //     match tmpl404.render(context!(
//    //                //         site => &site_clone,
//    //                //         page => Value::from_object(page.clone()),
//    //                //         ast => page.raw_ast.clone(),
//    //                //         error => e.display_debug_info().to_string(),
//    //                //         template_load_errors => &self.template_load_errors.clone(),
//    //                //     )) {
//    //                //         Ok(output) => {
//    //                //             let _ = write_file_with_mkdir(
//    //                //                 &self.site_config.error_page_404(),
//    //                //                 output.as_str(),
//    //                //             );
//    //                //         }
//    //                //         Err(_) => {}
//    //                //     }
//    //                // }
//    //                event!(
//    //                    Level::ERROR,
//    //                    "Could not render output for: {} - {} - {}",
//    //                    page.fs_path.display(),
//    //                    e.to_string(),
//    //                    e.display_debug_info(),
//    //                );
//    //                builder_error = Some(NeoError {
//    //                    kind: NeoErrorKind::GenericErrorWithSourcePath {
//    //                        source_path: absolute_output_path.to_path_buf(),
//    //                        msg: e.to_string(),
//    //                    },
//    //                });
//    //                if let Ok(template) = env.get_template("built-in-template-error") {
//    //                    dbg!("1111111111111111111111111111111");
//    //                    match template.render(context!(
//    //                        site => &site_clone,
//    //                        page => Value::from_object(page.clone()),
//    //                        builder_error => Value::from_serialize(builder_error),
//    //                    )) {
//    //                        Ok(output) => {
//    //                            dbg!("222222222222222222222222222");
//    //                            match write_file_with_mkdir(
//    //                                &absolute_output_path,
//    //                                output.as_str(),
//    //                            ) {
//    //                                Ok(()) => {
//    //                                    dbg!("333333333333333333333333333333333");
//    //                                    dbg!(&absolute_output_path);
//    //                                    ()
//    //                                    //event!(Level::INFO, "Making file in output_pages",);
//    //                                }
//    //                                Err(e) => self.errors.push({
//    //                                    dbg!("444444444444444444444444444444");
//    //                                    event!(
//    //                                        Level::ERROR,
//    //                                        "Could not write file: {}",
//    //                                        absolute_output_path.display()
//    //                                    );
//    //                                    NeoError {
//    //                                        kind: NeoErrorKind::GenericErrorWithSourcePath {
//    //                                            source_path: absolute_output_path.to_path_buf(),
//    //                                            msg: e,
//    //                                        },
//    //                                    }
//    //                                }),
//    //                            }
//    //                        }
//    //                        Err(e) => {
//    //                            dbg!(e);
//    //                            // page.error = Some(NeoError {
//    //                            //     kind: NeoErrorKind::GenericErrorWithSourcePath {
//    //                            //         source_path: absolute_output_path.to_path_buf(),
//    //                            //         msg: e.to_string(),
//    //                            //     },
//    //                            // });
//    //                        }
//    //                    }
//    //                }
//    //                self.errors.push(NeoError {
//    //                    kind: NeoErrorKind::GenericErrorWithSourcePath {
//    //                        source_path: page.fs_path.clone(),
//    //                        msg: e.to_string(),
//    //                    },
//    //                });
//    //            }
//    //        }
//    //    } else {
//    //        event!(
//    //            Level::ERROR,
//    //            "Could not find template for: {} - Missing: {}",
//    //            page.fs_path.display(),
//    //            page.template_list().join(",")
//    //        );
//    //        self.errors.push(NeoError {
//    //            kind: NeoErrorKind::GenericErrorWithSourcePath {
//    //                source_path: page.fs_path.clone(),
//    //                msg: format!(
//    //                    "Could not templates for page. {}",
//    //                    page.template_list().join(",")
//    //                ),
//    //            },
//    //        });
//    //    }
//    //});
//    Ok(())
//}

//// TODO: Deprecate in favor of load_source_from_files
//#[instrument(skip(self))]
//pub fn load_pages_from_file_system(&mut self) -> Result<()> {
//    event!(Level::INFO, "Loading Pages From FS");
//    for entry in WalkDir::new(&self.site_config.content_input_root_folder()) {
//        let path = entry?.path().to_path_buf();
//        if path.is_file() {
//            if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
//                if ext.to_ascii_lowercase() == "neo"
//                    && !filename.to_str().unwrap().starts_with(".")
//                {
//                    match PageV42::new_from_fs_path(&path, &self.site_config) {
//                        Ok(p) => {
//                            //dbg!(&p.ast());
//                            //event![Level::INFO, "Added raw page: {}", path.display()];
//                            self.pages.push(p)
//                        }
//                        Err(e) => {
//                            // event![Level::ERROR, "Error loading raw page: {}", path.display()];
//                            self.errors.push(e)
//                        }
//                    }
//                }
//            }
//        }
//    }
//    Ok(())
//}
