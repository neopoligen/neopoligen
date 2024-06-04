use crate::helpers::*;
use crate::neo_error::{NeoError, NeoErrorKind};
use crate::page_payload::PagePayload;
use crate::source_page::SourcePage;
use crate::{engine_config::EngineConfig, site_config::SiteConfig};
use anyhow::Result;
use minijinja::syntax::SyntaxConfig;
use minijinja::Environment;
use minijinja::{context, Value};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::fs;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Builder {
    config: Option<SiteConfig>,
    errors: Vec<NeoError>,
    source_pages: Vec<SourcePage>,
    payloads: Vec<PagePayload>,
    templates: BTreeMap<String, String>,
}

impl Builder {
    pub fn new_from_engine_config(engine_config: &EngineConfig) -> Result<Builder, NeoError> {
        let project_root = engine_config
            .sites_dir
            .join(engine_config.active_site.as_str());
        let config_path = project_root.join("admin").join("config.json");
        match fs::read_to_string(config_path) {
            Ok(text) => match serde_json::from_str::<SiteConfig>(&text) {
                Ok(mut config) => {
                    config.project_root = Some(project_root);
                    config.load_sections();
                    let b = Builder {
                        config: Some(config),
                        errors: vec![],
                        source_pages: vec![],
                        payloads: vec![],
                        templates: BTreeMap::new(),
                    };
                    Ok(b)
                }
                Err(e) => Err(NeoError {
                    kind: NeoErrorKind::GenericErrorWithoutSourcePath {
                        msg: format!("could not load admin/config.json file: {}", e),
                    },
                }),
            },
            Err(e) => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithoutSourcePath {
                    msg: format!("could not load admin/config.json file: {}", e),
                },
            }),
        }
    }
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
        self.source_pages.iter_mut().for_each(|page| {
            if let Err(e) = page.generate_ast() {
                self.errors.push(e);
            }
        })
    }

    #[instrument(skip(self))]
    pub fn generate_payloads(&mut self) {
        self.payloads = self
            .source_pages
            .iter()
            .filter_map(|page| match PagePayload::new_from_source_page(&page) {
                Ok(p) => Some(p),
                Err(_) => {
                    event!(
                        Level::ERROR,
                        "Page load error: TODO: make this a better message"
                    );
                    None
                }
            })
            .collect()
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
                        let mut page = SourcePage::new_from_source_path(
                            &path,
                            self.config.as_ref().unwrap().clone(),
                        )?;
                        match page.generate_ast() {
                            Ok(()) => self.source_pages.push(page),
                            Err(e) => {
                                // dbg!("OUTPUT ERROR HERE");
                                //dbg!(e);
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

    pub fn output_pages(&mut self) -> Result<()> {
        let mut env = Environment::new();
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
        for page in self.payloads.iter_mut() {
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

    pub fn tmp_output_errors(&self) -> Result<()> {
        let mut env = Environment::new();
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        env.add_template_owned(
            "tmp_status",
            r#"
<!DOCTYPE html>
<html><head><style> 
body { background-color: #111; color: #aaa; } 
</style></head><body><h1>Status</h1>
<ul>
[! for error in errors !]
<li><pre>[@ errors|tojson(true) @]</pre></li>
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
}
