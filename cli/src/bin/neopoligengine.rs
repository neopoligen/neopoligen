use dirs::document_dir;
use neopoligengine::engine_config::EngineConfig;
use neopoligengine::site::Site;
use neopoligengine::site_config::SiteConfig;
use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};
use tracing_subscriber::filter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

#[instrument]
fn main() {
    let mut log_file_path = document_dir().unwrap();
    log_file_path.push("Neopoligen");
    let log_dir = log_file_path.clone();
    log_file_path.push("log.log");

    let file_appender = tracing_appender::rolling::never(log_dir, log_file_path);
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

    let mut config_path = document_dir().clone().unwrap();
    config_path.push("Neopoligen");
    config_path.push("config-v0-1-0.json");
    if let Ok(engine_conf) = load_config_file(config_path) {
        let site_conf = SiteConfig::new(engine_conf.dev.active_site);
        let mut site = Site::new(site_conf);
        site.load_pages();
        site.parse_pages();
        let _ = empty_dir(&site.config.folders.error_root);
        site.parsing_errors.iter().for_each(|p| {
            if let Err(e) = write_file_with_mkdir(&p.0, &p.1) {
                event!(Level::ERROR, "Could not write error file: {}", e);
            }
        });
        site.missing_ids.iter().for_each(|p| {
            let _ = write_file_with_mkdir(&p.0, &p.1);
        });
    } else {
        event!(Level::ERROR, "Could not open config file");
    }
}

fn load_config_file(path: PathBuf) -> Result<EngineConfig, String> {
    match path.try_exists() {
        Ok(exists) => {
            if exists == true {
                match fs::read_to_string(&path) {
                    Ok(text) => match serde_json::from_str::<EngineConfig>(text.as_str()) {
                        Ok(data) => Ok(data),
                        Err(_) => Err(format!("Could not parse JSON file: {}", &path.display())),
                    },
                    Err(_) => Err(format!("Could not read JSON file: {}", &path.display())),
                }
            } else {
                Err(format!("Could not read JSON file: {}", &path.display()))
            }
        }
        Err(_) => Err(format!("No file at: {}", &path.display())),
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
