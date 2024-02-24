use axum::Router;
use dirs::config_dir;
use dirs::document_dir;
use neopoligen_cli::config::Config;
use neopoligen_cli::site_builder::SiteBuilder;
use notify_debouncer_mini::new_debouncer;
use notify_debouncer_mini::notify::*;
use notify_debouncer_mini::DebounceEventResult;
use notify_debouncer_mini::DebouncedEventKind;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::path::Path;
// use std::path::PathBuf;
use std::time::Duration;
// use tauri::{api::shell::open, Manager};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;
// use tracing::{event, instrument, Level};
// use tracing_appender;
// use tracing_chrome::ChromeLayerBuilder;
// use tracing_subscriber;
// use tracing_subscriber::prelude::*;

#[derive(Deserialize, Debug)]
pub struct NeoConfig {
    active_site: String,
}

#[tokio::main]
async fn main() {
    println!("Starting process...");
    let mut neo_config_file = config_dir().unwrap();
    neo_config_file.push("Neopoligen/config.json");
    match fs::read_to_string(&neo_config_file) {
        Ok(neo_config_string) => match serde_json::from_str::<NeoConfig>(&neo_config_string) {
            Ok(neo_config) => {
                let mut site_root = document_dir().unwrap();
                site_root.push("Neopoligen");
                site_root.push(neo_config.active_site);
                let site_config = Config::new(site_root);
                build_site(site_config.clone());
                if false {
                    // this is the tmp flag to turn off the watch without getting a warning
                    run_web_server(site_config).await;
                }
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

fn build_site(config: Config) {
    let mut site_builder = SiteBuilder::new(config);
    site_builder.build_site();
}

async fn run_web_server(config: Config) {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .nest_service("/", ServeDir::new(&config.folders.site_output_root))
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
    let watch_path = config.folders.site_project_root.clone();
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
                            if !e.path.starts_with(config.clone().folders.site_output_root)
                                && !e.path.starts_with(template_test_error_dir)
                            {
                                let timestamp = std::time::SystemTime::now()
                                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs();
                                println!("CMD: CLEAR");
                                println!("Caught new change at {}", timestamp);
                                build_site(config.clone());
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

// println!("Starting process...");
// let mut neo_config_file = config_dir().unwrap();
// neo_config_file.push("Neopoligen/config.json");
// if let Ok(neo_config_string) = fs::read_to_string(&neo_config_file) {
//     match serde_json::from_str::<NeoConfig>(&neo_config_string) {
//         Ok(neo_config) => {
//              let mut site_root = document_dir().unwrap();
//              site_root.push("Neopoligen");
//             site_root.push("sites");
//             site_root.push(neo_config.active_site);
//             let config = Config::new(site_root);
//                         build_site(config);
//         }
//         Err(e) => {
//         }
//     };
// } else {
// }

// let mut active_site_file = neopoligen_root.clone();
// active_site_file.push("configuration/active-site.txt");
// if let Ok(active_site_content) = fs::read_to_string(active_site_file) {

// }

// let mut active_site_file = neopoligen_root_folder.clone();
// active_site_file.push("configuration");
// active_site_file.push("active-site.txt");
// if let Ok(active_site_content) = fs::read_to_string(active_site_file) {
//     let active_site_name = active_site_content.lines().nth(0).unwrap().trim();
//     let mut active_site_folder = neopoligen_root_folder.clone();
//     active_site_folder.push("sites");
//     active_site_folder.push(active_site_name);
//     let config = Config::new(active_site_folder);
//     build_site(config.clone());
//     run_web_server(config).await;
// }

// fn run_watcher(reloader: Reloader, folder: PathBuf) {
//     println!("Starting watcher");
//     // let config = Config::default()
//     //     .with_poll_interval(Duration::from_millis(550))
//     //     .with_compare_contents(true);
//     let mut debouncer = new_debouncer(
//         Duration::from_millis(150),
//         move |res: DebounceEventResult| match res {
//             Ok(_events) => {
//                 let timestamp = std::time::SystemTime::now()
//                     .duration_since(std::time::SystemTime::UNIX_EPOCH)
//                     .unwrap()
//                     .as_secs();
//                 format!("{}", timestamp);
//                 println!("{}", timestamp);
//                 println!("got debounced event. building site");
//                 let site_folder = PathBuf::from("/Users/alan/Neopoligen/sites/neopoligen-site");
//                 let config = Config::new(site_folder);
//                 let mut site_builder = SiteBuilder::new(config);
//                 site_builder.build_site();
//                 println!("site build. calling reload");
//                 reloader.reload();
//                 println!("reload request sent");
//             }
//             Err(e) => println!("Error {:?}", e),
//         },
//     )
//     .unwrap();
//     debouncer
//         .watcher()
//         .watch(Path::new(&folder), RecursiveMode::Recursive)
//         .unwrap();
//     // TODO: Figure out how to keep this open without the
//     // loop since clippy says that wastes cpu
//     loop {}
// }

// #[tokio::main]
// async fn main() {
//     println!("Building...");
//     // let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
//     // tracing_subscriber::registry().with(chrome_layer).init();
//     let site_folder = PathBuf::from("/Users/alan/Neopoligen/neopoligen-site");
//     let config = Config::new(site_folder);
//     let mut site_builder = SiteBuilder::new(config);
//     site_builder.build_site();
//     println!("Done.");

//     // println!("Building initial site");
//     // site_builder.build_site();

//     // std::thread::spawn(move || run_web_server());

//     // run_web_server();

//     // println!("Starting up");
//     // let builder = tauri::Builder::default();

//     // builder
//     //     .invoke_handler(tauri::generate_handler![open_browser])
//     //     .run(tauri::generate_context!())
//     //     .expect("error while running tauri application");
// }

// async fn run_web_server() {
//     println!("Kicking off");
//     let site_folder = PathBuf::from("/Users/alan/Neopoligen/neopoligen-site");
//     let config = Config::new(site_folder);
//     let mut site_builder = SiteBuilder::new(config);
//     // println!("Building initial site");
//     // site_builder.build_site();
//     let livereload = LiveReloadLayer::new();
//     let content_reloader = livereload.reloader();
//     let theme_reloader = livereload.reloader();
//     let app = Router::new()
//         .nest_service(
//             "/",
//             ServeDir::new(site_builder.config.folders.site_folder.clone()),
//         )
//         .layer(livereload);
//     std::thread::spawn(move || {
//         run_watcher(
//             content_reloader,
//             PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//         )
//     });
//     std::thread::spawn(move || {
//         run_watcher(
//             theme_reloader,
//             PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//         )
//     });
//     // let _ = run_watcher(
//     //     content_reloader,
//     //     PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//     // );
//     // let _ = run_watcher(
//     //     theme_reloader,
//     //     PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//     // );
//     println!("Starting web server");
//     if let Ok(listener) = tokio::net::TcpListener::bind("localhost:1989").await {
//         if (axum::serve(listener, app).await).is_ok() {
//             // Server is going at this point
//         }
//     }
//     // async fn run_web_server() {
//     // NOTE: This clones site_builder() for the
//     // watchers which isn't
//     // great because it'll lead to problems when
//     // doing file checking for individual file updates,
//     // probably, but that's a problem for the future
//     // println!("Initial site built");
//     // let livereload = LiveReloadLayer::new();
//     // let reloader1 = livereload.reloader();
//     // let reloader2 = livereload.reloader();
//     // tauri::async_runtime::spawn(run_watcher(
//     //     reloader1,
//     //     PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//     // ));
//     // tauri::async_runtime::spawn(run_watcher(
//     //     reloader2,
//     //     PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_configuration/themes"),
//     // ));
//     // let app = Router::new()
//     //     .nest_service(
//     //         "/",
//     //         ServeDir::new(site_builder.config.folders.site_folder.clone()),
//     //     )
//     //     .layer(livereload);
//     // if let Ok(listener) = tokio::net::TcpListener::bind("localhost:1989").await {
//     //     if (axum::serve(listener, app).await).is_ok() {
//     //         // Server is going at this point
//     //     }
//     // }
// }

// let file_appender = tracing_appender::rolling::never("/Users/alan/Desktop", "neopoligen.log");
// let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
// tracing_subscriber::fmt().with_writer(non_blocking).init();

// #[cfg(debug_assertions)] // only enable instrumentation in development builds
// let devtools = devtools::init();

// let site_folder = PathBuf::from("/Users/alan/Neopoligen/neopoligen-site");
// let config = Config::new(site_folder);
// let mut site_builder = SiteBuilder::new(config);
// println!("Building initial site");
// site_builder.build_site();
// println!("Initial site built");

// let livereload = LiveReloadLayer::new();
// let reloader1 = livereload.reloader();
// let reloader2 = livereload.reloader();

// std::thread::spawn(move || {run_web_server(livereload)});

// #[cfg(debug_assertions)]
// let builder = builder.plugin(devtools);

// builder
//     // .setup(|_app| {
//     //     tauri::async_runtime::spawn(run_web_server(livereload));
//     //     tauri::async_runtime::spawn(run_watcher(
//     //         reloader1,
//     //         PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//     //     ));
//     //     tauri::async_runtime::spawn(run_watcher(
//     //         reloader2,
//     //         PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_configuration/themes"),
//     //     ));
//     //     Ok(())
//     // })
//     .invoke_handler(tauri::generate_handler![open_browser])
//     .run(tauri::generate_context!())
//     .expect("error while running tauri application");

// #[cfg(debug_assertions)] // only enable instrumentation in development builds
// let devtools = devtools::init();

// this didn't fix the increasing slowness either
// tokio::spawn(async move {
//     run_web_server().await
//   });

// println!("Starting up");
// let builder = tauri::Builder::default();

// // #[cfg(debug_assertions)]
// // let builder = builder.plugin(devtools);

// builder
//     // .setup(|_app| {
//     //     tauri::async_runtime::spawn(run_web_server());
//     //     Ok(())
//     // })
//     .invoke_handler(tauri::generate_handler![open_browser])
//     .run(tauri::generate_context!())
//     .expect("error while running tauri application");

// tokio::spawn(async move {
//     run_watcher(
//         reloader1,
//         PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//     ).await
// });

// tokio::spawn(async move {
//     run_watcher(
//         reloader2,
//         PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_configuration/themes"),
//     ).await
// });

// run_web_server(livereload).await;

// #[tokio::main]
// async fn web_server_start() {
//     println!("Web server start");

// }

// async fn run_web_server() {
//     println!("Kicking off");
//     let site_folder = PathBuf::from("/Users/alan/Neopoligen/neopoligen-site");
//     let config = Config::new(site_folder);
//     let mut site_builder = SiteBuilder::new(config);
//     // println!("Building initial site");
//     // site_builder.build_site();
//     let livereload = LiveReloadLayer::new();
//     let content_reloader = livereload.reloader();
//     let theme_reloader = livereload.reloader();
//     let app = Router::new()
//         .nest_service(
//             "/",
//             ServeDir::new(site_builder.config.folders.site_folder.clone()),
//         )
//         .layer(livereload);
//     std::thread::spawn(move || {
//         run_watcher(
//             content_reloader,
//             PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//         )
//     });
//     std::thread::spawn(move || {
//         run_watcher(
//             theme_reloader,
//             PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//         )
//     });
//     // let _ = run_watcher(
//     //     content_reloader,
//     //     PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//     // );
//     // let _ = run_watcher(
//     //     theme_reloader,
//     //     PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//     // );
//     println!("Starting web server");
//     if let Ok(listener) = tokio::net::TcpListener::bind("localhost:1989").await {
//         if (axum::serve(listener, app).await).is_ok() {
//             // Server is going at this point
//         }
//     }
//     // async fn run_web_server() {
//     // NOTE: This clones site_builder() for the
//     // watchers which isn't
//     // great because it'll lead to problems when
//     // doing file checking for individual file updates,
//     // probably, but that's a problem for the future
//     // println!("Initial site built");
//     // let livereload = LiveReloadLayer::new();
//     // let reloader1 = livereload.reloader();
//     // let reloader2 = livereload.reloader();
//     // tauri::async_runtime::spawn(run_watcher(
//     //     reloader1,
//     //     PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_content"),
//     // ));
//     // tauri::async_runtime::spawn(run_watcher(
//     //     reloader2,
//     //     PathBuf::from("/Users/alan/Neopoligen/neopoligen-site/_configuration/themes"),
//     // ));
//     // let app = Router::new()
//     //     .nest_service(
//     //         "/",
//     //         ServeDir::new(site_builder.config.folders.site_folder.clone()),
//     //     )
//     //     .layer(livereload);
//     // if let Ok(listener) = tokio::net::TcpListener::bind("localhost:1989").await {
//     //     if (axum::serve(listener, app).await).is_ok() {
//     //         // Server is going at this point
//     //     }
//     // }
// }

// fn run_watcher(reloader: Reloader, folder: PathBuf) {
//     println!("Starting watcher");
//     // let config = Config::default()
//     //     .with_poll_interval(Duration::from_millis(550))
//     //     .with_compare_contents(true);
//     let mut debouncer = new_debouncer(
//         Duration::from_millis(150),
//         move |res: DebounceEventResult| match res {
//             Ok(_events) => {
//                 let timestamp = std::time::SystemTime::now()
//                     .duration_since(std::time::SystemTime::UNIX_EPOCH)
//                     .unwrap()
//                     .as_secs();
//                 format!("{}", timestamp.to_string());
//                 println!("{}", timestamp);
//                 println!("got debounced event. building site");
//                 let site_folder = PathBuf::from("/Users/alan/Neopoligen/neopoligen-site");
//                 let config = Config::new(site_folder);
//                 let mut site_builder = SiteBuilder::new(config);
//                 site_builder.build_site();
//                 println!("site build. calling reload");
//                 reloader.reload();
//                 println!("reload request sent");
//             }
//             Err(e) => println!("Error {:?}", e),
//         },
//     )
//     .unwrap();
//     debouncer
//         .watcher()
//         .watch(Path::new(&folder), RecursiveMode::Recursive)
//         .unwrap();
//     loop {}
// }

// #[tauri::command]
// fn open_browser(app_handle: tauri::AppHandle) {
//     open(&app_handle.shell_scope(), "http://localhost:1989/", None).unwrap();
// }

// #[cfg(test)]
// mod test {
//     // no tests at this point for the output
// }
