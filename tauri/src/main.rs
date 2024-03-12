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
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;
use sysinfo::System;
use tauri::{
    api::process::{Command, CommandEvent},
    api::shell::open,
    Manager,
};

fn main() {
    let builder = tauri::Builder::default();
    builder
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            tauri::async_runtime::spawn(async move {
                // kill the cli if it's already running on mac.
                // this is needed because when the `cargo tauri dev``
                // restarts it doesn't kill the neopoligen_cli sidecar
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
            get_state,
            set_active_site,
            open_browser
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_browser(app_handle: tauri::AppHandle) {
    open(&app_handle.shell_scope(), "http://localhost:1989/", None).unwrap();
}

#[tauri::command]
fn open_finder(app_handle: tauri::AppHandle, site: String) {
    open(&app_handle.shell_scope(), "http://localhost:1989/", None).unwrap();
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
    active_site: Option<String>,
    sites: Vec<Site>,
    status: Option<CurrentStatus>,
    app_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CurrentStatus {
    Ok,
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
            if let Ok(mut state) = serde_json::from_str::<State>(&json_string) {
                state.status = Some(CurrentStatus::Ok);
                state.app_version = Some("0.1.0".to_string());
                serde_json::to_string(&state).unwrap()
            } else {
                r#"{ status: "could not parse engine config file" }"#.to_string()
            }
        } else {
            r#"{ status: "could not read engine config file" }"#.to_string()
        }
    } else {
        r#"{ status: "could not get local engine config dir" }"#.to_string()
    }

    // engine_config_file.push("Neopoligen");

    // let mut neopoligen_path = PathBuf::from(document_dir().unwrap());
    // neopoligen_path.push("Neopoligen");
    // match get_dirs_in_dir(&neopoligen_path) {
    //     Ok(dirs) => {
    //         let site_list = SiteList {
    //             sites: dirs
    //                 .iter()
    //                 .map(|s| Site {
    //                     key: s.file_name().unwrap().to_string_lossy().to_string(),
    //                 })
    //                 .collect(),
    //         };
    //         serde_json::to_string(&site_list).unwrap()
    //     }
    //     Err(_e) => r#"{ "status": "error", "msg": "Could not get neopoligen dir", "sites": [] }"#
    //         .to_string(),
    // }
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
