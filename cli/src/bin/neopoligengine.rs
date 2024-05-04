use dirs::document_dir;
use neopoligengine::engine_config;
use neopoligengine::engine_config::EngineConfig;
use neopoligengine::site::Site;
use neopoligengine::site_config::SiteConfig;
use neopoligengine::site_config::SiteConfigV2;
use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};
use tracing_subscriber::filter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

#[instrument]
fn main() {
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

    let mut engine_config_path = neopoligen_root.clone();
    engine_config_path.push("config-v0-1-0.json");

    if let Ok(engine_config) = load_engine_config_file(&engine_config_path) {
        match load_site_config_file(&neopoligen_root, &engine_config.dev.active_site) {
            Ok(mut site_config) => {
                site_config.load_sections();
                // dbg!(&site_config);

                let mut site = Site::new(site_config);
                site.load_pages();
                site.parse_pages();
                dbg!(site);
            }
            Err(e) => println!("{}", e),
        }

        // let site_conf = SiteConfig::new(engine_conf.dev.active_site);

        // let site_conf = SiteConfig::new(engine_conf.dev.active_site);
        // let mut site = Site::new(site_conf);
        // site.load_pages();
        // site.parse_pages();
        // let _ = empty_dir(&site.config.folders.error_root);
        // site.parsing_errors.iter().for_each(|p| {
        //     if let Err(e) = write_file_with_mkdir(&p.0, &p.1) {
        //         event!(Level::ERROR, "Could not write error file: {}", e);
        //     }
        // });
        // site.missing_ids.iter().for_each(|p| {
        //     let _ = write_file_with_mkdir(&p.0, &p.1);
        // });
    } else {
        event!(
            Level::ERROR,
            "Could not load engine file: {}",
            engine_config_path.display()
        );
    }
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

fn load_site_config_file(neo_root: &PathBuf, acitve_site: &str) -> Result<SiteConfigV2, String> {
    let mut project_root = neo_root.clone();
    project_root.push(acitve_site);
    let mut site_config_path = project_root.clone();
    site_config_path.push("config.json");
    match site_config_path.try_exists() {
        Ok(exists) => {
            if exists == true {
                match fs::read_to_string(&site_config_path) {
                    Ok(text) => match serde_json::from_str::<SiteConfigV2>(text.as_str()) {
                        Ok(mut config) => {
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
                                "errors_root".to_string(),
                                project_root.join(PathBuf::from("errors")),
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
                                "errors_root".to_string(),
                                project_root.join(PathBuf::from("errors")),
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
