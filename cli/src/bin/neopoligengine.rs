use axum::Router;
use dirs::document_dir;
//use minijinja::__context::build;
use neopoligengine::engine_config::EngineConfig;
use neopoligengine::file_watcher::FileWatcher;
use neopoligengine::page::Page;
use neopoligengine::site::Site;
use neopoligengine::site_config::SiteConfig;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use tokio::sync::mpsc;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;
use tracing::{event, instrument, Level};
use tracing_subscriber::filter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

#[tokio::main]
#[instrument]
async fn main() {
    let mut neopoligen_root = document_dir().unwrap();
    neopoligen_root.push("Neopoligen");

    let log_root = neopoligen_root.clone();
    let log_basename = "log.log";

    let file_appender = tracing_appender::rolling::never(log_root, log_basename);
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer_format = tracing_subscriber::fmt::format().json();
    let file_layer = fmt::Layer::default()
        .event_format(file_layer_format)
        .with_writer(file_writer)
        .json();

    let stdout_format = tracing_subscriber::fmt::format()
        .without_time()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_ansi(false)
        .with_line_number(false)
        .with_file(false);

    let stdout_layer = fmt::Layer::default()
        .event_format(stdout_format)
        .with_writer(std::io::stdout)
        .with_filter(filter::LevelFilter::INFO);

    let subscriber = tracing_subscriber::Registry::default()
        .with(file_layer)
        .with(stdout_layer);
    tracing::subscriber::set_global_default(subscriber).expect("unable to set global subscriber");

    let engine_config_path = match std::env::var("NEOENV") {
        Ok(val) => neopoligen_root.join(PathBuf::from(format!("config-{}.json", val))),
        Err(_) => neopoligen_root.join(PathBuf::from(format!("config.json"))),
    };

    if let Ok(engine_config) = load_engine_config_file(&engine_config_path) {
        event!(
            Level::INFO,
            "Loaded config: {}",
            engine_config_path.display()
        );
        event!(Level::INFO, "Active site: {}", &engine_config.active_site);
        match load_site_config_file(&neopoligen_root, &engine_config.active_site) {
            Ok(mut site_config) => {
                site_config.load_sections();
                check_templates(&site_config);
                build_site(&site_config);
                run_web_server(site_config.clone()).await;
            }
            Err(e) => println!("{}", e),
        }
    } else {
        event!(
            Level::ERROR,
            "Could not load engine file: {}",
            engine_config_path.display()
        );
    }
}

#[instrument(skip(site_config))]
fn check_templates(site_config: &SiteConfig) {
    event!(Level::INFO, "Checking Templates");
    let mut site = Site::new(site_config.clone());
    let mut page_errors: Vec<Page> = vec![];
    let _ = empty_dir(&site.config.paths.get("theme_errors_root").unwrap());
    site.load_templates();
    site.load_template_test_files();
    site.load_template_test_template();
    site.parse_pages(&mut page_errors);
    site.find_template_errors().iter().for_each(|tt| {
        let error_file_path = &site
            .config
            .paths
            .get("theme_errors_root")
            .unwrap()
            .join(
                &tt.page
                    .source_path
                    .strip_prefix(&site.config.paths.get("theme_tests_content_root").unwrap())
                    .unwrap(),
            )
            .with_extension("txt");
        if let Some(render_error) = &tt.render_error {
            let _ = write_file_with_mkdir(error_file_path, &render_error);
        } else {
            let error_text =
                tt.template_errors
                    .iter()
                    .fold("".to_string(), |acc, (expected, got)| {
                        format!(
                            "{}### Expected:\n\n{}\n\n\n### Got:\n\n{}\n\n",
                            acc,
                            simple_format_html(expected),
                            simple_format_html(got)
                        )
                    });
            let _ = write_file_with_mkdir(error_file_path, &error_text);
        }
    });

    // TODO: See if this needs to be pulled back in
    //// render errors for templates
    //page_errors.iter().for_each(|p| {
    //    let error_file_path = &site
    //        .config
    //        .paths
    //        .get("theme_errors_root")
    //        .unwrap()
    //        .join(
    //            &p.source_path
    //                .strip_prefix(&site.config.paths.get("content_root").unwrap())
    //                .unwrap(),
    //        )
    //        .with_extension("txt");
    //    //dbg!(error_file_path);
    //    let _ = write_file_with_mkdir(error_file_path, &p.error.clone().unwrap().to_string());
    //});
}

#[instrument(skip(site_config))]
fn build_site(site_config: &SiteConfig) {
    event!(Level::INFO, "Building Site");

    // This is the first run through the does the template tests
    let mut site = Site::new(site_config.clone());
    let mut page_errors: Vec<Page> = vec![];
    let _ = empty_dir(&site.config.paths.get("output_root").unwrap());
    let _ = empty_dir(&site.config.paths.get("render_errors_root").unwrap());
    site.load_templates();

    // This builds the actual files (don't clear the dirs otherwise
    // you'll erase the template output errors. // TODO: split
    // template test errors into their own dir
    site.load_templates();
    site.load_source_files();
    site.parse_pages(&mut page_errors);
    site.generate_content_pages().iter().for_each(|p| {
        let output_path = &site
            .config
            .paths
            .get("output_root")
            .unwrap()
            .join(p.0.strip_prefix("/").unwrap());
        let _ = write_file_with_mkdir(output_path, p.1);
    });

    page_errors.iter().for_each(|p| {
        let error_file_path = &site
            .config
            .paths
            .get("render_errors_root")
            .unwrap()
            .join(
                &p.source_path
                    .strip_prefix(&site.config.paths.get("content_root").unwrap())
                    .unwrap(),
            )
            .with_extension("txt");
        //dbg!(error_file_path);
        let _ = write_file_with_mkdir(error_file_path, &p.error.clone().unwrap().to_string());
    });

    // TODO: Move this so it's not included in the site
    // object so it doesn't show all the other errors for every
    // error in a semi-recursive way.
    site.render_errors.iter().for_each(|p| {
        let error_file_path = &site
            .config
            .paths
            .get("render_errors_root")
            .unwrap()
            .join(
                &p.0.strip_prefix(&site.config.paths.get("content_root").unwrap())
                    .unwrap(),
            )
            .with_extension("txt");
        dbg!(error_file_path);
        let _ = write_file_with_mkdir(error_file_path, p.1);
    });

    let _ = site.make_og_images();
    let _ = site.copy_theme_assets();
    let _ = site.copy_images();
    //
}

fn load_engine_config_file(path: &PathBuf) -> Result<EngineConfig, String> {
    match path.try_exists() {
        Ok(exists) => {
            if exists == true {
                match fs::read_to_string(&path) {
                    Ok(text) => match serde_json::from_str::<EngineConfig>(text.as_str()) {
                        Ok(data) => Ok(data),
                        Err(e) => Err(format!(
                            "Could not parse JSON file: {}\n{}",
                            &path.display(),
                            e
                        )),
                    },
                    Err(e) => Err(format!(
                        "Could not read JSON file: {}\n{}",
                        &path.display(),
                        e
                    )),
                }
            } else {
                Err(format!("Could not read JSON file: {}", &path.display()))
            }
        }
        Err(e) => Err(format!("{}", e)),
    }
}

fn empty_dir(dir: &PathBuf) -> std::io::Result<()> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

// todo. move paths, to dir function calls
fn load_site_config_file(neo_root: &PathBuf, active_site: &str) -> Result<SiteConfig, String> {
    let mut project_root = neo_root.clone();
    project_root.push(active_site);
    let mut site_config_path = project_root.clone();
    site_config_path.push("config.json");
    match site_config_path.try_exists() {
        Ok(exists) => {
            if exists == true {
                match fs::read_to_string(&site_config_path) {
                    Ok(text) => match serde_json::from_str::<SiteConfig>(text.as_str()) {
                        Ok(mut config) => {
                            config.project_root = Some(project_root.clone());
                            config.paths.insert(
                                "theme_root".to_string(),
                                project_root
                                    .join(PathBuf::from(format!("themes/{}", config.theme))),
                            );
                            config.paths.insert(
                                "theme_tests_root".to_string(),
                                project_root
                                    .join(PathBuf::from(format!("themes/{}/tests", config.theme))),
                            );
                            config.paths.insert(
                                "theme_tests_content_root".to_string(),
                                project_root.join(PathBuf::from(format!(
                                    "themes/{}/tests/content",
                                    config.theme
                                ))),
                            );
                            config
                                .paths
                                .insert("neopoligen_root".to_string(), neo_root.clone());
                            config
                                .paths
                                .insert("project_root".to_string(), project_root.clone());
                            config.paths.insert(
                                "content_root".to_string(),
                                project_root.join(PathBuf::from("content")),
                            );
                            config.paths.insert(
                                "render_errors_root".to_string(),
                                project_root.join(PathBuf::from("status/render-errors")),
                            );
                            config.paths.insert(
                                "theme_errors_root".to_string(),
                                project_root.join(PathBuf::from("status/theme-errors")),
                            );
                            config.paths.insert(
                                "themes_root".to_string(),
                                project_root.join(PathBuf::from("themes")),
                            );
                            config.paths.insert(
                                "output_root".to_string(),
                                project_root.join(PathBuf::from("docs")),
                            );
                            config.paths.insert(
                                "status_root".to_string(),
                                project_root.join(PathBuf::from("status")),
                            );
                            config.paths.insert(
                                "files_root".to_string(),
                                project_root.join(PathBuf::from("files")),
                            );
                            config.paths.insert(
                                "images_root".to_string(),
                                project_root.join(PathBuf::from("images")),
                            );
                            config.paths.insert(
                                "mp3s_root".to_string(),
                                project_root.join(PathBuf::from("mp3s")),
                            );
                            config.paths.insert(
                                "scripts_root".to_string(),
                                project_root.join(PathBuf::from("scripts")),
                            );
                            config
                                .paths
                                .insert("site_config_path".to_string(), site_config_path.clone());
                            Ok(config)
                        }
                        Err(e) => Err(format!(
                            "Could not parse JSON file: {}\n{}",
                            &site_config_path.display(),
                            e
                        )),
                    },
                    Err(e) => Err(format!(
                        "Could not read JSON file: {}\n{}",
                        &site_config_path.display(),
                        e
                    )),
                }
            } else {
                Err(format!(
                    "Could not read JSON file: {}",
                    &site_config_path.display()
                ))
            }
        }
        Err(e) => Err(format!("{}", e)),
    }
}

// #[instrument(skip(reloader, site_config))]
// fn run_watcher(reloader: Reloader, site_config: SiteConfig) {
//     println!("Starting watcher");
//     let watch_path = site_config.paths.get("project_root").unwrap().clone();
//     let mut debouncer = new_debouncer(
//         Duration::from_millis(100),
//         move |res: DebounceEventResult| match res {
//             Ok(events) => {
//                 match events.iter().find_map(|e| match e.kind {
//                     DebouncedEventKind::Any => {
//                         if e.path
//                             .starts_with(&site_config.paths.get("content_root").unwrap())
//                             || e.path
//                                 .starts_with(&site_config.paths.get("files_root").unwrap())
//                             || e.path
//                                 .starts_with(&site_config.paths.get("images_root").unwrap())
//                             || e.path
//                                 .starts_with(&site_config.paths.get("mp3s_root").unwrap())
//                             || e.path
//                                 .starts_with(&site_config.paths.get("scripts_root").unwrap())
//                             || e.path
//                                 .starts_with(&site_config.paths.get("themes_root").unwrap())
//                         {
//                             Some(e)
//                         } else {
//                             None
//                         }
//                     }
//                     _ => None,
//                 }) {
//                     Some(_) => {
//                         build_site(&site_config);
//                         reloader.reload();
//                     }
//                     None => {}
//                 }
//             }
//             Err(e) => println!("Error {:?}", e),
//         },
//     )
//     .unwrap();
//     debouncer
//         .watcher()
//         .watch(Path::new(&watch_path), RecursiveMode::Recursive)
//         .unwrap();
//     // TODO: Figure out how to keep this open without the
//     // loop since clippy says that wastes cpu
//     loop {}
// }

#[instrument(skip(reloader, site_config, rx))]
async fn catch_file_changes(
    reloader: Reloader,
    site_config: SiteConfig,
    mut rx: mpsc::Receiver<Vec<PathBuf>>,
) {
    while let Some(_r) = rx.recv().await {
        check_templates(&site_config);
        build_site(&site_config);
        event!(Level::INFO, "Reloading Browser");
        reloader.reload();
    }
}

#[instrument(skip(site_config))]
async fn run_web_server(site_config: SiteConfig) {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .nest_service(
            "/",
            ServeDir::new(&site_config.paths.get("output_root").unwrap()),
        )
        .layer(livereload);
    event!(Level::INFO, "Starting web server");
    let (tx, rx) = mpsc::channel(1);
    //dbg!(&site_config.content_dir());
    //dbg!(&site_config.theme_dir());
    let _theme_watcher = FileWatcher::new(&site_config.theme_dir(), tx.clone()).await;
    let _content_watcher = FileWatcher::new(&site_config.content_dir(), tx.clone()).await;
    tokio::spawn(async move {
        catch_file_changes(reloader, site_config, rx).await;
    });
    if let Ok(listener) = tokio::net::TcpListener::bind("localhost:1989").await {
        if (axum::serve(listener, app).await).is_ok() {
            // Server is going at this point
        }
    }
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

fn simple_format_html(code: &str) -> String {
    let mut re = Regex::new(r"\n").unwrap();
    let output = re.replace_all(code, " ");
    re = Regex::new(r" \s+").unwrap();
    let output = re.replace_all(&output, " ");
    re = Regex::new(r"\s+<").unwrap();
    let output = re.replace_all(&output, "<");
    re = Regex::new(r">\s+").unwrap();
    let output = re.replace_all(&output, ">");
    let parts: Vec<&str> = output.split("<").collect();
    let mut assembler: Vec<String> = vec![];
    let mut level = 0i8;
    assembler.push(parts[0].to_string());
    parts.iter().skip(1).for_each(|part| {
        if part.starts_with("/") {
            level -= 2;
        }
        for _ in 0..level {
            assembler.push(" ".to_string());
        }
        assembler.push(format!("<{}\n", part));
        if !part.starts_with("/") {
            level += 2;
        }
    });
    assembler.join("").to_string()
}
