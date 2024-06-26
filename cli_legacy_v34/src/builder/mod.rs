pub mod new;

use crate::config::Config;
use crate::file_set::FileSet;
use crate::neo_config::NeoEnv;
use crate::site::Site;
use crate::template_error::TemplateError;
use fs_extra::dir::copy;
use minijinja::context;
use minijinja::Environment;
use minijinja::Syntax;
use minijinja::Value;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::time::Instant;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use tracing::{event, instrument, Level};

pub struct Builder {
    pub build_time: Option<String>,
    config: Config,
    file_set: FileSet,
    #[allow(dead_code)]
    neo_env: NeoEnv,
    pub outputs: BTreeMap<PathBuf, String>,
    pub outputs_dev: Vec<Output>,
    pub template_tests_errors: Vec<TemplateError>,
    pub template_tests_found: usize,
    pub template_tests_run: usize,
    pub template_tests_skipped: usize,
}

pub struct Output {
    pub content: String,
    pub output_path: PathBuf,
    pub source_path: PathBuf,
}

impl Builder {
    pub fn copy_asset_folders(&self) {
        let now = Instant::now();
        let asset_folders = vec!["files", "images", "mp3s", "scripts"];
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        // options.content_only = true;
        asset_folders.iter().for_each(|folder| {
            event!(Level::INFO, "Copying: {}", &folder);
            let mut source_folder = self.config.folders.project_root.clone();
            source_folder.push(folder);
            //let mut dest_folder = self.config.folders.build_root.clone();
            //dest_folder.push(folder);
            //let _ = verify_dir(&PathBuf::from(&dest_folder));
            let _ = verify_dir(&self.config.folders.build_root);
            match copy(source_folder, &self.config.folders.build_root, &options) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        });
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
    }

    pub fn copy_theme_assets(&self) {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let in_dir = self
            .config
            .folders
            .theme_assets_input_root
            .display()
            .to_string();
        let site_build_root_dir = self
            .config
            .folders
            .theme_assets_build_root
            .display()
            .to_string();
        match copy(in_dir, site_build_root_dir, &options) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    #[instrument(skip(self))]
    pub fn generate_files(&mut self) {
        let timestamp = chrono::prelude::Local::now();
        self.build_time = Some(timestamp.to_rfc2822());
        let mut env = Environment::new();
        env.add_function("highlight_code", highlight_code);
        env.add_function("get_collection", get_collection);
        env.set_syntax(Syntax {
            block_start: "[!".into(),
            block_end: "!]".into(),
            variable_start: "[@".into(),
            variable_end: "@]".into(),
            comment_start: "[#".into(),
            comment_end: "#]".into(),
        })
        .unwrap();
        env.set_trim_blocks(true);
        env.set_lstrip_blocks(true);
        let site = Site::new(&self.file_set, &self.config);
        let site_obj = Value::from_serialize(&site.clone());
        self.file_set
            .templates
            .iter()
            .for_each(|t| env.add_template_owned(t.0, t.1).unwrap());
        site.pages.iter().for_each(|p| {
            let page = p.1;
            // event!(Level::INFO, "Processing: {}", page.source_path.display());
            // dbg!(page.id.clone());
            let template_searches = vec![
                format!(
                    "pages/{}/{}.neojinja",
                    &page.base_template.clone().unwrap(),
                    &page.status.clone().unwrap(),
                ),
                format!(
                    "pages/{}/published.neojinja",
                    &page.base_template.clone().unwrap()
                ),
                format!("pages/post/{}.neojinja", &page.status.clone().unwrap()),
                format!("pages/post/published.neojinja"),
            ];
            if let Some(template_name) =
                template_searches
                    .iter()
                    .find_map(|t| match &site.templates.get(t) {
                        Some(_) => Some(t),
                        None => None,
                    })
            {
                if let Ok(tmpl) = env.get_template(template_name) {
                    match tmpl.render(context!(
                         site => site_obj,
                        page_id => page.id
                    )) {
                        Ok(output) => {
                            self.outputs.insert(
                                PathBuf::from(&page.output_file_path.clone().unwrap()),
                                output.clone(),
                            );
                            self.outputs_dev.push(Output {
                                content: output,
                                source_path: page.source_path.clone(),
                                output_path: PathBuf::from(&page.output_file_path.clone().unwrap()),
                            });
                            ()
                        }
                        Err(e) => {
                            event!(Level::ERROR, "File: {}, {}", page.source_path.display(), e)
                        }
                    }
                } else {
                    event!(Level::ERROR, "Could not get template: {}", template_name);
                }
            }
        });
    }

    #[instrument(skip(self))]
    pub fn move_files_in_place(&self) {
        if self.config.folders.output_root.exists() {
            let _ = fs::remove_dir_all(&self.config.folders.output_root);
        }
        let _ = fs::rename(
            &self.config.folders.build_root,
            &self.config.folders.output_root,
        );
    }

    #[instrument(skip(self))]
    pub fn output_files(&self) {
        self.outputs.iter().for_each(|output| {
            if output
                .0
                .starts_with(self.config.folders.build_root.display().to_string())
            {
                let build_path = PathBuf::from(output.0);
                let parent_dir = build_path.parent().unwrap();
                let _ = create_dir_all(parent_dir);
                let _ = fs::write(build_path, output.1);
            } else {
                event!(
                    Level::ERROR,
                    "Tried to write outside the site root: {}",
                    output.0.display()
                );
            }
            event!(Level::DEBUG, "Writing: {}", output.0.display());
        });
    }
}

fn verify_dir(dir: &PathBuf) -> std::io::Result<()> {
    if dir.exists() {
        Ok(())
    } else {
        fs::create_dir_all(dir)
    }
}

// fn highlight_code_probably_not_necessary(code: &str, lang: &str) -> String {
//     let ps = SyntaxSet::load_defaults_newlines();
//     let ts = ThemeSet::load_defaults();
//     let syntax = ps
//         .find_syntax_by_token(lang)
//         .unwrap_or_else(|| ps.find_syntax_plain_text());
//     let mut h = HighlightLines::new(syntax, &ts.themes["Solarized (dark)"]);
//     let regions = h.highlight_line(code, &ps).unwrap();
//     let mut html = styled_line_to_highlighted_html(&regions[..], IncludeBackground::No).unwrap();
//     let replacements = vec![
//         ("color:#839496;", "alfa "),
//         ("color:#dc322f;", "bravo "),
//         ("color:#002b36;", "charlie "),
//         ("color:#2aa198;", "delta "),
//         ("color:#b58900;", "echo "),
//         ("color:#657b83;", "foxtrot "),
//         ("color:#268bd2;", "golf "),
//         ("color:#d33682;", "hotel "),
//         ("color:#6c71c4;", "india "),
//         ("color:#6e2e32;", "juliett "),
//         ("color:#586e75;", "kilo "),
//         ("color:#cb4b16;", "lima "),
//         ("color:#93a1a1;", "mike "),
//         ("color:#859900;", "november "),
//         ("background-color:#839496;", "alfa "),
//         ("background-color:#dc322f;", "bravo "),
//         ("background-color:#002b36;", "charlie "),
//         ("background-color:#b58900;", "delta "),
//         ("background-color:#657b83;", "echo "),
//         ("background-color:#6e2e32;", "foxtrot "),
//         ("background-color:#586e75;", "golf "),
//         ("background-color:#cb4b16;", "hotel "),
//     ];
//     replacements
//         .iter()
//         .for_each(|r| html = html.replace(r.0, r.1));
//     html = html.replace("span style", "span class");
//     html
// }

fn highlight_code(code: String, lang: String) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set
        .find_syntax_by_token(&lang)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
    for line in LinesWithEndings::from(code.trim()) {
        let _ = html_generator.parse_html_for_line_which_includes_newline(line);
    }
    html_generator.finalize()
}

#[instrument(skip(site))]
fn get_collection(site: &Value, page_id: &Value, filters_raw: &Value) -> Value {
    let mut filters: Vec<(Vec<String>, Vec<String>)> = vec![];
    match filters_raw.try_iter() {
        Ok(filters_iter) => {
            filters_iter.for_each(|filter| match filter.try_iter() {
                Ok(items) => {
                    let mut accept: Vec<String> = vec![];
                    let mut reject: Vec<String> = vec![];
                    items.for_each(|item| {
                        if item.to_string().starts_with("!") {
                            let string_to_add = item.to_string();
                            reject.push(string_to_add.strip_prefix("!").unwrap().to_string());
                        } else {
                            accept.push(item.to_string());
                        }
                    });
                    filters.push((accept, reject));
                }
                Err(e) => event!(Level::ERROR, "{}", e),
            });
        }
        Err(e) => event!(Level::ERROR, "{}", e),
    }
    // dbg!(&filters);

    let mut ids: BTreeSet<String> = BTreeSet::new();
    match site.get_attr("pages") {
        Ok(pages) => {
            match pages.try_iter() {
                Ok(pages_iter) => pages_iter.for_each(|id| {
                    match pages.get_attr(id.as_str().expect("got str")) {
                        Ok(page) => {
                            let mut add_page = false;
                            match page.get_attr("tags") {
                                Ok(tags) => {
                                    let tags2 = tags.clone();

                                    // do the first loop to add
                                    match tags.try_iter() {
                                        Ok(tags_iter) => {
                                            tags_iter.for_each(|tag| {
                                                filters.iter().for_each(|filter| {
                                                    if filter.0.contains(&tag.to_string()) {
                                                        add_page = true;
                                                    }
                                                    ()
                                                });
                                            });
                                        }
                                        Err(e) => event!(Level::ERROR, "{}", e),
                                    };

                                    // loop again to remove because order matters
                                    match tags2.try_iter() {
                                        Ok(tags_iter) => {
                                            tags_iter.for_each(|tag| {
                                                filters.iter().for_each(|filter| {
                                                    if filter.1.contains(&tag.to_string()) {
                                                        add_page = false;
                                                    }
                                                    ()
                                                });
                                            });
                                        }
                                        Err(e) => event!(Level::ERROR, "{}", e),
                                    };
                                }
                                Err(e) => event!(Level::ERROR, "{}", e),
                            }
                            if add_page {
                                ids.insert(id.to_string());
                            }
                        }
                        Err(e) => event!(Level::ERROR, "{}", e),
                    };
                }),
                Err(e) => event!(Level::ERROR, "{}", e),
            };
        }
        Err(e) => {
            event!(Level::ERROR, "{}", e);
        }
    };
    Value::from_iter(ids.into_iter())
}
