use crate::page_v2::PageV2;
use crate::site_config::SiteConfig;
use crate::site_v2::SiteV2;
use anyhow::Result;
use minijinja::context;
use minijinja::syntax::SyntaxConfig;
use minijinja::value::Value;
use minijinja::Environment;
use regex::Regex;
use rusqlite::Connection;
use std::collections::BTreeMap;
use std::{fs, path::PathBuf};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Builder {
    pub pages: BTreeMap<PathBuf, PageV2>,
    pub config: SiteConfig,
    // pub pages: Vec<PageV2>,
}

impl Builder {
    pub fn clear_changed_asts(&self) -> Result<()> {
        // TODO: clear changed ASTs here
        Ok(())
    }

    pub fn create_cache_db_if_necessary(&self) -> Result<()> {
        let conn = Connection::open(self.config.cache_db_path())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS page_archive (source_path TEXT, cached_hash TEXT, ast TEXT, output_content TEXT)", ())?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_missing_asts(&mut self) -> Result<()> {
        self.pages.iter_mut().for_each(|p| {
            if p.1.ast.len() == 0 {
                p.1.generate_ast(&self.config);
            }
        });
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn generate_page_content(&mut self) -> Result<()> {
        let site = Value::from_serialize(SiteV2::new(&self.config, &self.pages));
        let mut env = Environment::new();
        env.set_debug(true);
        env.add_function("highlight_code", highlight_code);
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );

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
                        let template_name = path.strip_prefix(self.config.templates_dir()).unwrap();
                        let _ = env.add_template_owned(
                            template_name.to_string_lossy().to_string(),
                            content,
                        );
                    }
                    Err(e) => {
                        event!(Level::ERROR, "{}", e)
                    }
                };
            });

        self.pages.iter_mut().for_each(|p| {
            dbg!(&p.1.id());
        });
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_cached_pages(&self) -> Result<()> {
        let conn = Connection::open(self.config.cache_db_path())?;
        let mut stmt =
            conn.prepare("SELECT source_path, cached_hash, source_ast FROM page_archive")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            //let source_path = row.get(0)?.as_path_buf();
            //dbg!(source_path);
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_source_files(&mut self) -> Result<()> {
        event!(Level::INFO, "Loading Source Files");
        let dir = &self.config.paths.get("content_root").unwrap();
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
                        self.pages
                            .insert(path.clone(), PageV2::new_from_filesystem(path, content));
                        ()
                    }
                    Err(e) => {
                        event!(Level::ERROR, "{}", e)
                    }
                }
            });
        Ok(())
    }

    pub fn new(config: SiteConfig) -> Result<Builder> {
        Ok(Builder {
            pages: BTreeMap::new(),
            config,
            //pages: vec![],
        })
    }

    // pub fn cache_hashes(&self) -> Vec<(PathBuf, String)> {
    //     vec![]
    // }

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
