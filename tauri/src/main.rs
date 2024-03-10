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

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
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
                for process in s.processes_by_exact_name("neopoligen_cli") {
                    kill(
                        Pid::from_raw(process.pid().as_u32().try_into().unwrap()),
                        Signal::SIGTERM,
                    )
                    .unwrap();
                }

                let (mut rx, mut _child) = Command::new_sidecar("neopoligen_cli")
                    .expect("failed to setup `neopoligen_cli` sidecar")
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
        .invoke_handler(tauri::generate_handler![open_browser])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_browser(app_handle: tauri::AppHandle) {
    open(&app_handle.shell_scope(), "http://localhost:1989/", None).unwrap();
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
