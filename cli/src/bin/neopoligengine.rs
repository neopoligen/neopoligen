use axum::Router;
use dirs::{config_local_dir, document_dir};
use neopoligengine::builder::Builder;
use neopoligengine::config::Config;
use neopoligengine::file_set::FileSet;
use neopoligengine::template_tester::*;
use notify_debouncer_mini::new_debouncer;
use notify_debouncer_mini::notify::RecursiveMode;
use notify_debouncer_mini::DebounceEventResult;
use notify_debouncer_mini::DebouncedEventKind;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;
use tracing::{event, instrument, Level};

#[derive(RustEmbed)]
#[folder = "example-site"]
struct ExampleSite;

#[derive(Deserialize)]
pub struct EngineConfig {
    settings: EngineConfigSettings,
}

#[derive(Deserialize)]
pub struct EngineConfigSettings {
    active_site: String,
}

#[tokio::main]
#[instrument]
async fn main() {
    event!(Level::INFO, r#"Lanucing neopoligengine"#);
    match get_engine_config_file() {
        Ok(toml) => match toml::from_str::<EngineConfig>(&toml) {
            Ok(engine_config) => {
                let mut site_root = document_dir().unwrap();
                site_root.push("Neopoligen");
                site_root.push(engine_config.settings.active_site);
                match set_up_site_if_necessary(&site_root) {
                    Ok(_) => {}

                    // dbg!(site_root);

                    // let config = Config::new(site_root);
                    // let mut log_file_path = document_dir().unwrap();
                    // log_file_path.push("Neopoligen");
                    // log_file_path.push("log.log");
                    // let _ = fs::remove_file(&log_file_path);
                    // let file_appender = tracing_appender::rolling::never(
                    //     log_file_path.parent().unwrap(),
                    //     log_file_path.file_name().unwrap(),
                    // );
                    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
                    // let format = tracing_subscriber::fmt::format().pretty();
                    // tracing_subscriber::fmt()
                    //     .event_format(format)
                    //     .with_ansi(false)
                    //     .with_writer(non_blocking)
                    //     .init();
                    // event!(Level::INFO, r#"Processes started"#);

                    // build_site(&config);
                    // if true {
                    //     run_web_server(config).await;
                    // }
                    Err(e) => println!("{}", e),
                };
            }
            Err(e) => {
                println!("{}", e)
            }
        },
        Err(e) => {
            println!("{}", e)
        }
    }
}

// match fs::read_to_string(&engine_config_file) {
//     Ok(engine_config_string) => match toml::from_str::<EngineConfig>(&engine_config_string) {
//         Ok(engine_config) => {
//             let mut site_root = document_dir().unwrap();
//             site_root.push("Neopoligen");
//             site_root.push(engine_config.settings.active_site);
//             let config = Config::new(site_root);
//             let mut log_file_path = document_dir().unwrap();
//             log_file_path.push("Neopoligen");
//             log_file_path.push("log.log");
//             let _ = fs::remove_file(&log_file_path);
//             let file_appender = tracing_appender::rolling::never(
//                 log_file_path.parent().unwrap(),
//                 log_file_path.file_name().unwrap(),
//             );
//             let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
//             let format = tracing_subscriber::fmt::format().pretty();
//             tracing_subscriber::fmt()
//                 .event_format(format)
//                 .with_ansi(false)
//                 .with_writer(non_blocking)
//                 .init();
//             event!(Level::INFO, r#"Processes started"#);
//             build_site(&config);
//             if true {
//                 run_web_server(config).await;
//             }
//         }
//         Err(e) => {
//             println!("{}", e)
//         }
//     },
//     Err(e) => {
//         println!("{}", e)
//     }
//
// }

fn build_site(config: &Config) {
    println!("Starting build run");
    test_templates(&config);
    let mut file_set = FileSet::new();
    file_set.load_content(&config.folders.content_root);
    file_set.load_templates(&config.folders.theme_root);
    file_set.load_images(&config.folders.images_root);
    let builder = Builder::new(file_set, &config);
    builder.write_files();
    builder.copy_files();
    builder.copy_theme_assets();
}

fn get_engine_config_file() -> Result<String, String> {
    let mut engine_config_path = config_local_dir().unwrap();
    engine_config_path.push("Neopoligen");
    match engine_config_path.try_exists() {
        Ok(check) => {
            if check == false {
                match fs::create_dir(&engine_config_path) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("{}", e)),
                }
            }
        }
        Err(e) => return Err(format!("{}", e)),
    }
    engine_config_path.push("config.toml");
    match engine_config_path.try_exists() {
        Ok(check) => {
            if check == false {
                let default_config = "[settings]\nactive_site = \"example-site\"";
                match fs::write(&engine_config_path, default_config) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("{}", e)),
                }
            }
        }
        Err(e) => return Err(format!("{}", e)),
    }
    match fs::read_to_string(&engine_config_path) {
        Ok(toml) => Ok(toml),
        Err(e) => Err(format!("{}", e)),
    }
}

#[instrument]
fn set_up_site_if_necessary(site_root: &PathBuf) -> Result<String, String> {
    let path = PathBuf::from(site_root);

    match path.try_exists() {
        Ok(check) => {
            if check == false {
                match fs::create_dir(&path) {
                    Ok(_) => {
                        for rel_file_path in ExampleSite::iter() {
                            let mut output_path = path.clone();
                            output_path.push(rel_file_path.as_ref());
                            let output_dir = output_path.parent().unwrap();
                            match output_dir.try_exists() {
                                Ok(status) => {
                                    if status == false {
                                        match fs::create_dir(output_dir) {
                                            Ok(_) => event!(
                                                Level::INFO,
                                                r#"Created dir: {}"#,
                                                output_dir.display()
                                            ),
                                            Err(e) => return Err(format!("{}", e)),
                                        }
                                    }
                                }
                                Err(e) => return Err(format!("{}", e)),
                            }
                            let output_data = ExampleSite::get(&rel_file_path).unwrap();
                            let _ = fs::write(output_path, output_data.data);
                        }
                    }

                    Err(e) => return Err(format!("{}", e)),
                }
                println!("Site doesnt' exist. making it");
                Ok("TODO".to_string())
            } else {
                Ok("Site already exists".to_string())
            }
        }
        Err(e) => return Err(format!("{}", e)),
    }
}

async fn run_web_server(config: Config) {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .nest_service("/", ServeDir::new(&config.folders.output_root))
        .layer(livereload);
    tokio::spawn(async move {
        run_watcher(reloader, config.clone());
    });
    println!("Starting web server");
    if let Ok(listener) = tokio::net::TcpListener::bind("localhost:1989").await {
        if (axum::serve(listener, app).await).is_ok() {
            // Server is going at this point
        }
    }
}

fn run_watcher(reloader: Reloader, config: Config) {
    println!("Starting watcher");
    let watch_path = config.folders.project_root.clone();
    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        move |res: DebounceEventResult| match res {
            Ok(events) => {
                events.iter().find_map(|e| {
                    match e.kind {
                        DebouncedEventKind::Any => {
                            let mut template_test_error_dir =
                                config.clone().folders.theme_tests_root;
                            template_test_error_dir.push("_errors");
                            if !e.path.starts_with(config.clone().folders.output_root)
                                && !e.path.starts_with(template_test_error_dir)
                            {
                                dbg!(&e);
                                let timestamp = std::time::SystemTime::now()
                                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs();
                                println!("CMD: CLEAR");
                                println!("Caught new change at {}", timestamp);
                                build_site(&config);
                                println!("Sending reload signal");
                                reloader.reload();
                            }
                            Some(e)
                        }
                        _ => None,
                    }
                    // if e.kind == DebouncedEventKind::Any {
                    //     dbg!(e.kind);
                    // };
                    // None::<String>
                });
                // dbg!(&events);
                // let mut site_builder = SiteBuilder::new(config);
                // site_builder.build_site();
                // println!("site build. calling reload");
                // println!("reload request sent");
            }
            Err(e) => println!("Error {:?}", e),
        },
    )
    .unwrap();
    debouncer
        .watcher()
        .watch(Path::new(&watch_path), RecursiveMode::Recursive)
        .unwrap();
    // TODO: Figure out how to keep this open without the
    // loop since clippy says that wastes cpu
    loop {}
}
