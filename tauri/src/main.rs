// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use axum::Router;
// use notify::RecursiveMode;
// use notify_debouncer_mini::new_debouncer;
// use notify_debouncer_mini::DebounceEventResult;
// use serde::Serialize;
// use std::fmt::Display;
// use std::fs;
// use std::path::Path;
// use std::thread;
// use std::time::Duration;
// use std::time::SystemTime;
// use tower_http::services::ServeDir;
// use tower_livereload::LiveReloadLayer;
// use tower_livereload::Reloader;

use dirs::{self, config_local_dir, document_dir};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;
use std::{env, hint};
use sysinfo::System;
use tauri::{
    api::process::{Command, CommandEvent},
    api::shell::open,
    Manager,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct EngineConfig {
    dev: EngineEnv,
    prod: EngineEnv,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EngineEnv {
    active_site: String,
    port: u16,
}

fn main() {
    let builder = tauri::Builder::default();
    builder
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            // window.open_devtools();
            tauri::async_runtime::spawn(async move {
                // the process_by_example_name stuff below
                // kills the cli if it's already running on mac.
                // this is needed because when the `cargo tauri dev``
                // restarts, it doesn't kill the neopoligen_cli sidecar
                // process
                let s = System::new_all();
                for process in s.processes_by_exact_name("neopoligengine") {
                    kill(
                        Pid::from_raw(process.pid().as_u32().try_into().unwrap()),
                        Signal::SIGTERM,
                    )
                    .unwrap();
                }
                let (mut rx, mut _child) = Command::new_sidecar("neopoligengine")
                    .expect("failed to setup `neopoligengine` sidecar")
                    .spawn()
                    .expect("Failed to spawn packaged node");
                // let mut i = 0;
                while let Some(event) = rx.recv().await {
                    if let CommandEvent::Stdout(line) = event {
                        print!("{}", line);
                        window
                            .emit("neo_message", Some(format!("{}", line)))
                            .expect("failed to emit event");
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            delete_neopoligen_config,
            edit_in_vscode,
            get_active_site,
            get_state,
            get_template_error_status,
            open_browser,
            open_neo_folder,
            open_finder,
            open_link,
            set_active_site,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn active_site() -> String {
    let mut engine_config_path = document_dir().unwrap();
    engine_config_path.push("Neopoligen");
    engine_config_path.push("config.json");
    let config = load_config_file(engine_config_path).unwrap();
    match env::var("NEOENV") {
        Ok(current_env) => {
            if current_env == "dev" {
                config.dev.active_site
            } else {
                config.prod.active_site
            }
        }
        Err(_) => config.prod.active_site,
    }
}

#[tauri::command]
fn get_active_site() -> String {
    format!(r#"{{ "payload": "{}" }}"#, active_site())
}

#[tauri::command]
fn get_template_error_status() -> String {
    let mut status_file_path = document_dir().unwrap();
    status_file_path.push("Neopoligen");
    status_file_path.push(active_site());
    status_file_path.push("status");
    status_file_path.push("template_errors.htm");
    match fs::read_to_string(status_file_path) {
        Ok(html) => html,
        Err(e) => format!("{}", e),
    }
}

fn load_config_file(path: PathBuf) -> Result<EngineConfig, String> {
    match path.try_exists() {
        Ok(exists) => {
            if exists == true {
                let text = fs::read_to_string(&path).unwrap();
                match serde_json::from_str::<EngineConfig>(text.as_str()) {
                    Ok(data) => Ok(data),
                    Err(_) => Err(format!("Could not parse JSON file: {}", &path.display())),
                }
            } else {
                Err(format!("Could not read JSON file: {}", &path.display()))
            }
        }
        Err(_) => Err(format!("Could not read JSON file: {}", &path.display())),
    }
}

#[tauri::command]
fn delete_neopoligen_config() -> String {
    dbg!("Deleting Neopoligen Config");
    let mut path = config_local_dir().unwrap();
    path.push("Neopoligen");
    path.push("config.json");
    if path.exists() {
        let _ = fs::remove_file(path);
    }
    serde_json::to_string(&StatusPayload::Status(CurrentStatus::Ok)).unwrap()
}

#[tauri::command]
fn open_browser(app_handle: tauri::AppHandle) {
    open(&app_handle.shell_scope(), "http://localhost:1989/", None).unwrap();
}

#[tauri::command]
fn open_link(app_handle: tauri::AppHandle, url: String) {
    open(&app_handle.shell_scope(), &url, None).unwrap();
}

#[tauri::command]
fn open_finder() {
    dbg!("opening in finder");
    Command::new("open")
        .args([format!("/Users/alan/Documents/Neopoligen/")])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn open_neo_folder() -> String {
    Command::new("open")
        .args([format!("/Users/alan/Documents/Neopoligen/")])
        .spawn()
        .unwrap();
    r#"{ "status": { "type": "todo" } }"#.to_string()
}

#[tauri::command]
fn edit_in_vscode() -> String {
    dbg!("opening in vscode");
    let mut site_folder = document_dir().unwrap();
    site_folder.push("Neopoligen");
    // site_folder.push(site.clone());
    let status = Command::new("open")
        .args([
            "-a",
            "Visual Studio Code",
            &site_folder.display().to_string(),
        ])
        .status()
        .expect("failed to run open");
    match status.code() {
        Some(code) => {
            if code == 0 {
                r#"{ "status": { "type": "ok", "x": {}} }"#.to_string()
            } else {
                r#"{ "status": { "type": "error", "msg": "Could not open VS Code" } }"#.to_string()
            }
        }
        None => r#"{ "status": { "type": "error", "msg": "Could not open VS Code" } }"#.to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    key: String,
}

// Deprecated: Remove this
#[derive(Debug, Serialize, Deserialize)]
pub struct SiteList {
    sites: Vec<Site>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    config: NeoConfig,
    sites: Vec<Site>,
    status: Option<CurrentStatus>,
    app_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeoConfig {
    active_site: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusPayload {
    Status(CurrentStatus),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "lowercase")]
pub enum CurrentStatus {
    Ok,
    Todo { msg: String },
    Error { msg: String },
}

// fn get_active_site() -> String {
//     let mut config_file_path = config_local_dir().unwrap();
//     config_file_path.push("Neopoligen");
//     config_file_path.push("config.json");
//     if let Ok(json_string) = fs::read_to_string(config_file_path) {
//         let data: EngineConfig = serde_json::from_str(&json_string).unwrap();
//         dbg!(&data);
//     }
//     "example-site".to_string()
// }

#[tauri::command]
fn get_state() -> String {
    if let Some(mut engine_config_file) = config_local_dir() {
        engine_config_file.push("Neopoligen");
        engine_config_file.push("config.json");
        if let Ok(json_string) = fs::read_to_string(engine_config_file) {
            if let Ok(config) = serde_json::from_str::<NeoConfig>(&json_string) {
                let mut neopoligen_path = PathBuf::from(document_dir().unwrap());
                neopoligen_path.push("Neopoligen");
                let sites = match get_dirs_in_dir(&neopoligen_path) {
                    Ok(dirs) => dirs
                        .iter()
                        .map(|s| {
                            let site_key = s.file_name().unwrap().to_string_lossy().to_string();
                            Site { key: site_key }
                        })
                        .collect::<Vec<Site>>(),
                    Err(_e) => {
                        println!("Error gettings sites");
                        vec![]
                    }
                };
                let state = State {
                    config,
                    status: Some(CurrentStatus::Ok),
                    app_version: Some("0.0.1".to_string()),
                    sites,
                };
                serde_json::to_string(&state).unwrap()
            } else {
                serde_json::to_string(&StatusPayload::Status(CurrentStatus::Error {
                    msg: "Could not parse config JSON".to_string(),
                }))
                .unwrap()
            }
        } else {
            serde_json::to_string(&CurrentStatus::Ok).unwrap()
        }
    } else {
        serde_json::to_string(&CurrentStatus::Ok).unwrap()
    }
}

fn get_dirs_in_dir(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    Result::from_iter(
        fs::read_dir(dir)?
            .map(|entry| {
                let entry = entry?;
                Ok(entry)
            })
            .filter_map(|entry: Result<DirEntry, io::Error>| {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    match path.file_name() {
                        Some(file_name) => {
                            if file_name.to_string_lossy().starts_with(".") {
                                None
                            } else if file_name.to_string_lossy().starts_with("_") {
                                None
                            } else {
                                let mut check_config_path = path.clone();
                                check_config_path.push("config.json");
                                if check_config_path.exists() {
                                    Some(Ok(path))
                                } else {
                                    None
                                }
                            }
                        }
                        None => None,
                    }
                } else {
                    None
                }
            }),
    )
}

#[tauri::command(rename_all = "snake_case")]
fn set_active_site(site_key: String) {
    dbg!(&site_key);
    let updated_config = format!("[settings]\nactive_site = \"{}\"", site_key);
    let mut config_file_path = config_local_dir().unwrap();
    config_file_path.push("Neopoligen");
    config_file_path.push("config.toml");
    dbg!(&config_file_path);
    let _ = fs::write(config_file_path, updated_config);
}

// fn main() {
//     let builder = tauri::Builder::default();
//     // let livereload = LiveReloadLayer::new();
//     // let reloader = livereload.reloader();
//     builder
//         .setup(|_app| {
//             // tauri::async_runtime::spawn(run_web_server(livereload));
//             // tauri::async_runtime::spawn(run_watcher(reloader));
//             Ok(())
//         })
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

// async fn run_web_server(livereload: LiveReloadLayer) {
//     println!("Starting web server");
//     let app = Router::new()
//         .nest_service(
//             "/",
//             ServeDir::new("/Users/alan/Documents/Neopoligen/sites/neopoligen-site/docs"),
//         )
//         .layer(livereload);
//     if let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:1989").await {
//         if let Ok(_) = axum::serve(listener, app).await {
//             // server is running
//         }
//     }
// }

// async fn run_watcher(reloader: Reloader) {
//     println!("Starting watcher");
//     let mut debouncer = new_debouncer(
//         Duration::from_millis(150),
//         move |res: DebounceEventResult| match res {
//             Ok(_events) => {
//                 reloader.reload();
//             }
//             Err(e) => println!("Error {:?}", e),
//         },
//     )
//     .unwrap();
//     debouncer
//         .watcher()
//         .watch(Path::new("../input"), RecursiveMode::Recursive)
//         .unwrap();
//     loop {}
// }
