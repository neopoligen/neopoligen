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
use serde_json;
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
    pub fn create_cache_db_if_necessary(&self) -> Result<()> {
        let conn = Connection::open(self.config.cache_db_path())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS page_archive (path TEXT, page TEXT)",
            (),
        )?;
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
        event!(Level::INFO, "Generating Page Content");
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
            match p.1.output {
                Some(_) => {}
                None => {
                    let template_name = "pages/post/published.neoj";
                    if let Ok(tmpl) = env.get_template(template_name) {
                        match tmpl.render(context!(
                            site => site,
                            page_id => p.1.id()
                        )) {
                            Ok(output) => {
                                p.1.output = Some(output);
                            }
                            Err(_) => {
                                // TODO: Provide error handling here
                                p.1.output = None;
                            }
                        }
                    } else {
                        // TODO: Provide error handling here
                        event!(Level::ERROR, "Could not get template: {}", template_name);
                    }
                }
            }
        });
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn load_cached_pages(&mut self) -> Result<()> {
        let conn = Connection::open(self.config.cache_db_path())?;
        let mut stmt = conn.prepare("SELECT path, page FROM page_archive")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let path_string: String = row.get(0)?;
            let path = PathBuf::from(path_string);
            let page: String = row.get(1)?;
            let p: PageV2 = serde_json::from_str(&page.to_string())?;
            self.pages.insert(path, p);
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
                        let page =
                            PageV2::new_from_filesystem(path.clone(), self.config.clone(), content);
                        match self.pages.get(&path.clone()) {
                            Some(cache_page) => {
                                if cache_page.hash() != page.hash() {
                                    self.pages.insert(path.clone(), page);
                                }
                            }
                            None => {
                                self.pages.insert(path.clone(), page);
                            }
                        }
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
        })
    }

    pub fn output_content_files(&self) -> Result<()> {
        self.pages.iter().for_each(|p| {
            if let (Some(rel_file_path), Some(output)) = (p.1.rel_file_path(), p.1.output.clone()) {
                let output_path = self.config.output_dir().join(rel_file_path);
                // TODO: Add error handling here
                let _ = write_file_with_mkdir(&output_path, &output);
            }
        });
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn update_cache(&self) -> Result<()> {
        event!(Level::INFO, "Updating Cache");
        let mut conn = Connection::open(self.config.cache_db_path())?;
        conn.execute("DROP TABLE IF EXISTS page_archive", ())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS page_archive (path TEXT, page TEXT)",
            (),
        )?;
        let query = "INSERT INTO page_archive(path, page) VALUES (?1, ?2)";
        let tx = conn.transaction()?;
        for p in self.pages.iter() {
            if let Ok(data) = serde_json::to_string(p.1) {
                tx.execute(
                    query,
                    (
                        p.1.source_path
                            .clone()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                        data,
                    ),
                )?;
            };
        }
        tx.commit()?;
        Ok(())
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

fn write_file_with_mkdir(path: &PathBuf, content: &str) -> Result<(), String> {
    match path.parent() {
        Some(parent_dir) => match fs::create_dir_all(parent_dir) {
            Ok(_) => match fs::write(path, content) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        },
        None => Err("Could not make directory".to_string()),
    }
}
