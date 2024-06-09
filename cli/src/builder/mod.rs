use crate::helpers::*;
use crate::neo_error::{NeoError, NeoErrorKind};
use crate::page_payload::PagePayload;
use crate::site_config::SiteConfig;
use crate::source_page::SourcePage;
use crate::theme_test::ThemeTest;
use anyhow::Result;
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
    cache_buffer: BTreeMap<PathBuf, SourcePage>,
    pub config: Option<SiteConfig>,
    errors: Vec<NeoError>,
    source_pages: BTreeMap<PathBuf, SourcePage>,
    payloads: BTreeMap<String, PagePayload>,
    templates: BTreeMap<String, String>,
    pub theme_test_source_pages: BTreeMap<PathBuf, ThemeTest>,
}

impl Builder {
    pub fn new_from_site_config(site_config: &SiteConfig) -> Result<Builder, NeoError> {
        let b = Builder {
            cache_buffer: BTreeMap::new(),
            config: Some(site_config.clone()),
            errors: vec![],
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
    pub fn generate_payloads(&mut self) {
        self.payloads = BTreeMap::new();
        self.source_pages.iter().for_each(|(source_path, page)| {

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
        });
    }

    #[instrument(skip(self))]
    pub fn load_pages_from_cache(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Cached Files");
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
        for entry in WalkDir::new(&self.config.as_ref().unwrap().templates_dir()) {
            let path = entry?.path().to_path_buf();
            if path.is_file() {
                if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                    if ext.to_ascii_lowercase() == "neoj"
                        && !filename.to_str().unwrap().starts_with(".")
                    {
                        let template_name =
                            &path.strip_prefix(&self.config.as_ref().unwrap().templates_dir());
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
        for entry in WalkDir::new(&self.config.as_ref().unwrap().templates_dir()) {
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
                    page => Value::from_serialize(&page)
                )) {
                    Ok(output) => {
                        let _ = write_file_with_mkdir(&output_path, &output);
                    }
                    Err(e) => {
                        dbg!(e);
                        ()
                    }
                };
            } else {
                event!(Level::ERROR, "Could not find template");
            };
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
            "sections/basic/expected-output/end/default.neoj",
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
                                            source_path: PathBuf::from(""),
                                        },
                                    })
                                }
                            }
                        });
                    }
                    Err(e) => {
                        dbg!(e);
                        ()
                    }
                };
            } else {
                event!(Level::ERROR, "Could not find template");
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
        env.add_template_owned(
            "tmp_status",
            r#"
<!DOCTYPE html>
<html><head><style> 
body { background-color: #111; color: #aaa; } 
</style></head><body>
<header><a href="/">Home</a></header>
<h1>Status</h1>
<ul>
[! for error in errors !]
<li>
[! if error.kind.source_path !]<h4>Path: [@ error.kind.source_path @]</h4>[! endif !]


[! if error.kind.type == "themetesterror" !]
    <h2>Theme Test Error</h2>
    <h3>Expected</h3>
    <pre>
    [@ error.kind.expected|escape @]
    </pre>
    <h3>Got</h3>
    <pre>
    [@ error.kind.got|escape @]
    </pre>
    <h3>Sections</h3>
    <pre>
    [@ error.kind.sections|tojson(true)@]
    </pre>
[! else !]
    <h2>[@ error.type @]</h2>
    <pre>
    [@ error|escape @]
    </pre>
[! endif !]
</li>
[! endfor !]
</ul>
</body></html>
        "#,
        )?;
        let tmpl = env.get_template("tmp_status")?;
        let output = tmpl.render(context!(
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
