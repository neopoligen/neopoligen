use axum::Router;
use dirs::config_dir;
use dirs::data_local_dir;
use neopoligengine::builder::Builder;
use neopoligengine::engine_config::EngineConfig;
use neopoligengine::file_watcher::FileWatcher;
use neopoligengine::site_config::SiteConfig;
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
    let log_root = data_local_dir().unwrap().join("Neopoligen");
    let log_basename = "neopoligen-log.txt";
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
    let engine_config_dir = config_dir().unwrap().join("Neopoligen");
    let engine_config_path = match std::env::var("NEOENV") {
        Ok(val) => engine_config_dir.join(PathBuf::from(format!("config-{}.json", val))),
        Err(_) => engine_config_dir.join(PathBuf::from(format!("config.json"))),
    };
    match EngineConfig::new_from_file(&engine_config_path) {
        Ok(engine_config) => {
            event!(
                Level::INFO,
                "Loaded Engine Config: {}",
                engine_config_path.display()
            );
            event!(Level::INFO, "Active site: {}", &engine_config.active_site);
            run_web_server(engine_config.clone()).await;
        }
        Err(e) => {
            dbg!(e);
            ()
        }
    }
}

#[instrument(skip(engine_config))]
async fn run_web_server(engine_config: EngineConfig) {
    event!(Level::INFO, "Starting web server");
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    build_site(&engine_config, &reloader);
    match SiteConfig::new_from_engine_config(&engine_config) {
        Ok(site_config) => {
            let localhost_domain = format!("localhost:{}", engine_config.port);
            let app = Router::new()
                .nest_service("/", ServeDir::new(&site_config.output_dest_dir()))
                .nest_service("/neo-status", ServeDir::new(&site_config.status_dest_dir()))
                .layer(livereload);
            let (tx, rx) = mpsc::channel(1);
            let _content_watcher =
                FileWatcher::new(&site_config.content_source_dir(), tx.clone()).await;
            let _theme_watcher = FileWatcher::new(&site_config.theme_dir(), tx.clone()).await;
            tokio::spawn(async move {
                catch_file_changes(reloader, engine_config.clone(), rx).await;
            });
            if let Ok(listener) = tokio::net::TcpListener::bind(localhost_domain).await {
                if (axum::serve(listener, app).await).is_ok() {
                    // Server is going at this point
                }
            }
        }
        Err(e) => {
            event!(
                Level::ERROR,
                "Could not load site config for web server: {:?}",
                e
            );
        }
    }
}

#[instrument(skip(reloader, engine_config, rx))]
async fn catch_file_changes(
    reloader: Reloader,
    engine_config: EngineConfig,
    mut rx: mpsc::Receiver<Vec<PathBuf>>,
) {
    while let Some(_) = rx.recv().await {
        build_site(&engine_config, &reloader);
    }
}

#[instrument(skip(engine_config, reloader))]
fn build_site(engine_config: &EngineConfig, reloader: &Reloader) {
    event!(Level::INFO, "Building Site");
    match Builder::new_from_engine_config(engine_config) {
        Ok(mut builder) => {
            let _ = builder.prep_output_dirs();
            builder.load_pages_from_cache().unwrap();
            builder.load_pages_from_fs().unwrap();
            builder.generate_missing_asts();
            let _ = builder.save_asts_to_cache();
            builder.generate_payloads();
            let _ = builder.load_templates();
            let _ = builder.empty_output_dirs();
            let _ = builder.output_pages();
            builder.tmp_output_errors().unwrap();

            // builder.todo("update_file_cache");
            // builder.todo("generate_site_object");
            // builder.todo("load_templates");
            // builder.todo("generate_page_output");
            // builder.todo("generated_last_edit_page");
            // builder.todo("empty_output_dirs");
            // builder.todo("prep_output_dirs");
            // builder.todo("output_pages");
            // builder.todo("deploy_theme_file_assets");
            // builder.todo("deploy_images");
            // builder.todo("deploy_og_images");
            // builder.todo("deploy_gifs");
            // builder.todo("deploy_mp3s");
            // builder.todo("deploy_svgs");
            // builder.todo("generate_feeds");
            // builder.todo("load_theme_test_files");
            // builder.todo("load_theme_test_templates");
            // builder.todo("test_theme");
            // builder.todo("update_status");
            // builder.todo("reload_browser");
            event!(Level::INFO, "Reloading Browser");
            reloader.reload();
        }
        Err(e) => {
            event!(Level::ERROR, "Could not make builder: {:?}", e);
        }
    }

    //
}

// fn load_engine_config_file(path: &PathBuf) -> Result<EngineConfig, String> {
//     match path.try_exists() {
//         Ok(exists) => {
//             if exists == true {
//                 match fs::read_to_string(&path) {
//                     Ok(text) => match serde_json::from_str::<EngineConfig>(text.as_str()) {
//                         Ok(data) => Ok(data),
//                         Err(e) => Err(format!(
//                             "Could not parse JSON file: {}\n{}",
//                             &path.display(),
//                             e
//                         )),
//                     },
//                     Err(e) => Err(format!(
//                         "Could not read JSON file: {}\n{}",
//                         &path.display(),
//                         e
//                     )),
//                 }
//             } else {
//                 Err(format!("Could not read JSON file: {}", &path.display()))
//             }
//         }
//         Err(e) => Err(format!("{}", e)),
//     }
// }

// // todo. move paths, to dir function calls
// fn load_site_config_file(neo_root: &PathBuf, active_site: &str) -> Result<SiteConfig, String> {
//     let mut project_root = neo_root.clone();
//     project_root.push(active_site);
//     let mut site_config_path = project_root.clone();
//     site_config_path.push("config.json");
//     match site_config_path.try_exists() {
//         Ok(exists) => {
//             if exists == true {
//                 match fs::read_to_string(&site_config_path) {
//                     Ok(text) => match serde_json::from_str::<SiteConfig>(text.as_str()) {
//                         Ok(mut config) => {
//                             config.project_root = Some(project_root.clone());
//                             Ok(config)
//                         }
//                         Err(e) => Err(format!(
//                             "Could not parse JSON file: {}\n{}",
//                             &site_config_path.display(),
//                             e
//                         )),
//                     },
//                     Err(e) => Err(format!(
//                         "Could not read JSON file: {}\n{}",
//                         &site_config_path.display(),
//                         e
//                     )),
//                 }
//             } else {
//                 Err(format!(
//                     "Could not read JSON file: {}",
//                     &site_config_path.display()
//                 ))
//             }
//         }
//         Err(e) => Err(format!("{}", e)),
//     }
// }

// DEPRECATED: This is the reference copy that can be
// delete when the server set up is done
// #[instrument(skip(site_config))]
// async fn run_web_server(site_config: SiteConfig) {
//     let livereload = LiveReloadLayer::new();
//     let reloader = livereload.reloader();
//     let app = Router::new()
//         .nest_service("/", ServeDir::new(&site_config.output_dir()))
//         .nest_service("/neo-status", ServeDir::new(&site_config.status_dir()))
//         .layer(livereload);
//     event!(Level::INFO, "Starting web server");
//     let (tx, rx) = mpsc::channel(1);
//     let _theme_watcher = FileWatcher::new(&site_config.theme_dir(), tx.clone()).await;
//     let _content_watcher = FileWatcher::new(&site_config.content_dir(), tx.clone()).await;
//     tokio::spawn(async move {
//         catch_file_changes(reloader, site_config, rx).await;
//     });
//     if let Ok(listener) = tokio::net::TcpListener::bind("localhost:1989").await {
//         if (axum::serve(listener, app).await).is_ok() {
//             // Server is going at this point
//         }
//     }
// }
