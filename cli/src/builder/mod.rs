use std::fs;

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
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Builder {
    config: Option<SiteConfig>,
    errors: Vec<NeoError>,
    source_pages: Vec<SourcePage>,
    payloads: Vec<PagePayload>,
}

impl Builder {
    pub fn new_from_engine_config(engine_config: &EngineConfig) -> Result<Builder> {
        let project_root = engine_config
            .sites_dir
            .join(engine_config.active_site.as_str());
        let config_path = project_root.join("admin").join("config.json");
        let text = fs::read_to_string(config_path)?;
        let mut config = serde_json::from_str::<SiteConfig>(&text)?;
        config.project_root = Some(project_root);
        config.load_sections();
        let b = Builder {
            config: Some(config),
            errors: vec![],
            source_pages: vec![],
            payloads: vec![],
        };
        Ok(b)
    }
}

impl Builder {
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
        self.source_pages.iter_mut().for_each(|page| {
            if let Some(id) = page.id() {
                let mut p = PagePayload::new_from_id(&id);
                p.rel_file_path = page.rel_file_path();
                self.payloads.push(p);
            } else {
                self.errors.push(NeoError {
                    kind: NeoErrorKind::FileError {
                        source_path: page.source_path.clone().unwrap(),
                        msg: "Could not get ID for file".to_string(),
                    },
                })
            }
        })
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
                        let page = SourcePage::new_from_source_path(
                            &path,
                            self.config.as_ref().unwrap().clone(),
                        )?;
                        self.source_pages.push(page);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn output_pages(&self) -> Result<()> {
        let mut env = Environment::new();
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        let _ = env.add_template_owned(
            "tmp-template",
            r#"
<!DOCTYPE html>
<html><head><style> 
body { background-color: #111; color: #aaa; } 
</style></head><body><h1>Page</h1>
[@ page @]
</body></html>"#
                .to_string(),
        );

        for page in self.payloads.iter() {
            let output_path = self
                .config
                .as_ref()
                .unwrap()
                .output_dest_dir()
                .join(page.rel_file_path.as_ref().unwrap());
            let tmpl = env.get_template("tmp-template").unwrap();
            match tmpl.render(context!(
                page => Value::from_serialize(&page)
            )) {
                Ok(output) => {
                    let _ = write_file_with_mkdir(&output_path, &output);
                }
                Err(_) => (),
            }
            ()
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
<li>[@ errors @]</li>
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
