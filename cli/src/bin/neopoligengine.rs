use axum::Router;
use dirs::document_dir;
use neopoligengine::builder::Builder;
use neopoligengine::config::Config;
use neopoligengine::file_set::FileSet;
use neopoligengine::neo_config::NeoConfig;
use neopoligengine::neo_config::NeoEnv;
// use neopoligengine::template_tester::test_templates;
use notify_debouncer_mini::new_debouncer;
use notify_debouncer_mini::notify::RecursiveMode;
use notify_debouncer_mini::DebounceEventResult;
use notify_debouncer_mini::DebouncedEventKind;
use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;
use tracing::{event, instrument, Level};
use tracing_subscriber::filter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

#[derive(RustEmbed)]
#[folder = "example-site"]
struct ExampleSite;

#[tokio::main]
#[instrument]
async fn main() {
    let mut log_file_path = document_dir().unwrap();
    log_file_path.push("Neopoligen");
    let log_dir = log_file_path.clone();
    log_file_path.push("log.log");
    let _ = fs::remove_file(&log_file_path);

    let file_appender = tracing_appender::rolling::never(log_dir, log_file_path);
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::Layer::default()
        .with_writer(file_writer)
        .with_ansi(false);

    let stdout_layer = fmt::Layer::default()
        .with_writer(std::io::stdout)
        .with_ansi(false)
        .with_filter(filter::LevelFilter::INFO);

    let subscriber = tracing_subscriber::Registry::default()
        .with(file_layer)
        .with(stdout_layer);

    tracing::subscriber::set_global_default(subscriber).expect("unable to set global subscriber");

    let neo_env_var = match std::env::var("NEOENV") {
        Ok(val) => val,
        Err(_) => "prod".to_string(),
    };

    event!(Level::INFO, r#"Launching neopoligengine"#);
    match get_engine_config_file() {
        Ok(json) => match serde_json::from_str::<NeoConfig>(&json) {
            Ok(engine_config) => {
                // TODO set up for dev/prod/test switch here
                // based off env var
                let neo_env = match neo_env_var.as_str() {
                    "prod" => engine_config.clone().prod,
                    "dev" => engine_config.clone().dev,
                    _ => engine_config.clone().dev,
                };
                let active_site = neo_env.active_site.clone().unwrap();
                event!(Level::DEBUG, r#"Active site: {}"#, &active_site);

                let mut site_root = document_dir().unwrap();
                site_root.push("Neopoligen");
                site_root.push(active_site);
                match set_up_site_if_necessary(&site_root) {
                    Ok(_) => {
                        let config = Config::new(site_root);
                        let now = Instant::now();
                        build_site(&config, &neo_env);
                        event!(Level::DEBUG, "SITEBUILDTIME: {:?}", now.elapsed());
                        if true {
                            // TODO Set a flag so this can be toggled on/off in the config
                            run_web_server(config, neo_env).await;
                        }
                    }
                    Err(e) => {
                        event!(
                            Level::ERROR,
                            "Problem with set_up_site_if_necessary: {:?}",
                            e
                        );
                    }
                };
            }
            Err(e) => {
                event!(
                    Level::ERROR,
                    "Problem with config file (bravo message): {:?}",
                    e
                );
            }
        },
        Err(e) => {
            event!(
                Level::ERROR,
                "Problem with config file (chralie message): {:?}",
                e
            );
        }
    }
}

#[instrument(skip(config, neo_env))]
fn build_site(config: &Config, neo_env: &NeoEnv) {
    event!(
        Level::INFO,
        "Starting Build Run: {}",
        neo_env.active_site.as_ref().unwrap()
    );
    let _ = verify_dir(&config.folders.build_root);
    let _ = empty_dir(&config.folders.build_root);
    //test_templates(&config, neo_env.clone());

    event!(Level::INFO, "Loading Content");
    let mut file_set = FileSet::new();
    file_set.load_content(&config.folders.content_root);
    file_set.load_images(&config.folders.images_root);
    file_set.load_mp3s(&config.folders.mp3s_root);
    file_set.load_templates(&config.folders.theme_root);
    event!(Level::INFO, "Building Site");
    let mut builder = Builder::new(file_set, &config, &neo_env);
    builder.generate_files();
    builder.output_files();
    //builder.write_changed_files(); // TODO: finishing dev for write_changed_files
    //builder.write_files(); // TODO: Rename to write_all_files
    builder.copy_asset_folders();
    builder.copy_theme_assets();
    builder.move_files_in_place();
}

#[instrument]
fn get_engine_config_file() -> Result<String, String> {
    let mut engine_config_path = document_dir().unwrap();
    engine_config_path.push("Neopoligen");
    match engine_config_path.try_exists() {
        Ok(check) => {
            if check == false {
                match fs::create_dir(&engine_config_path) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("-----{}", e)),
                }
            }
        }
        Err(e) => return Err(format!("------{}", e)),
    }
    engine_config_path.push("config.json");
    match engine_config_path.try_exists() {
        Ok(check) => {
            if check == false {
                let default_config = r#"{ "active_site": "Example-Site" }"#;
                match fs::write(&engine_config_path, default_config) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("-------{}", e)),
                }
            }
        }
        Err(e) => return Err(format!("--------{}", e)),
    }
    match fs::read_to_string(&engine_config_path) {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("---------{}", e)),
    }
}

#[instrument(skip(site_root))]
fn set_up_site_if_necessary(site_root: &PathBuf) -> Result<String, String> {
    let path = PathBuf::from(site_root);
    event!(Level::INFO, "Set Up Site Path: {}", &path.display());
    match path.try_exists() {
        Ok(check) => {
            if check == false {
                match fs::create_dir(&path) {
                    Ok(_) => {
                        let project_dirs =
                            vec!["configuration", "content", "files", "images", "themes"];
                        project_dirs.iter().for_each(|d| {
                            let mut pdp = path.clone();
                            pdp.push(d);
                            let _ = fs::create_dir(pdp);
                        });
                        for rel_file_path in ExampleSite::iter() {
                            let mut output_path = path.clone();
                            output_path.push(rel_file_path.as_ref());
                            let output_dir = output_path.parent().unwrap();
                            match output_dir.try_exists() {
                                Ok(status) => {
                                    if status == false {
                                        match fs::create_dir_all(output_dir) {
                                            Ok(_) =>
                                            // event!(
                                            // Level::DEBUG,
                                            // r#"Created dir: {}"#,
                                            // output_dir.display()
                                            // );
                                            {
                                                ()
                                            }
                                            Err(e) => {
                                                return Err(format!("-{}", e));
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    return Err(format!("--{}", e));
                                }
                            }
                            let output_data = ExampleSite::get(&rel_file_path).unwrap();
                            let _ = fs::write(output_path, output_data.data);
                        }
                    }
                    Err(e) => {
                        return Err(format!("---{}", e));
                    }
                }
                event!(Level::INFO, "Site doesn't exist. Making it...");
                Ok("TODO".to_string())
            } else {
                Ok("Site already exists".to_string())
            }
        }
        Err(e) => {
            return Err(format!("{}", e));
        }
    }
}

async fn run_web_server(config: Config, neo_env: NeoEnv) {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .nest_service("/", ServeDir::new(&config.folders.output_root))
        .layer(livereload);
    tokio::spawn(async move {
        run_watcher(reloader, config.clone(), neo_env);
    });
    println!("Starting web server");
    if let Ok(listener) = tokio::net::TcpListener::bind("localhost:1989").await {
        if (axum::serve(listener, app).await).is_ok() {
            // Server is going at this point
        }
    }
}

fn run_watcher(reloader: Reloader, config: Config, neo_env: NeoEnv) {
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
                            if !e.path.starts_with(&config.folders.output_root)
                                && !e.path.starts_with(template_test_error_dir)
                                && !e.path.starts_with(&config.folders.build_root)
                            {
                                dbg!(&e);
                                let timestamp = std::time::SystemTime::now()
                                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs();
                                println!("CMD: CLEAR");
                                println!("Caught new change at {}", timestamp);
                                build_site(&config, &neo_env);
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

fn verify_dir(dir: &PathBuf) -> std::io::Result<()> {
    if dir.exists() {
        Ok(())
    } else {
        fs::create_dir_all(dir)
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
