use axum::Router;
use dirs::document_dir;
use neopoligen::builder::Builder;
use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::helpers::get_file_paths_for_extension::get_file_paths_for_extension;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many1;
use nom::IResult;
use notify_debouncer_mini::new_debouncer;
use notify_debouncer_mini::notify::*;
use notify_debouncer_mini::DebounceEventResult;
use notify_debouncer_mini::DebouncedEventKind;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;
// use tracing::{event, instrument, Level};

#[derive(Deserialize)]
pub struct EngineConfig {
    settings: EngineConfigSettings,
}

#[derive(Deserialize)]
pub struct EngineConfigSettings {
    active_site: String,
}

#[tokio::main]
async fn main() {
    let format = tracing_subscriber::fmt::format().pretty();
    tracing_subscriber::fmt().event_format(format).init();
    let mut engine_config_file = document_dir().unwrap();
    engine_config_file.push("Neopoligen");
    engine_config_file.push("config.toml");
    match fs::read_to_string(&engine_config_file) {
        Ok(engine_config_string) => match toml::from_str::<EngineConfig>(&engine_config_string) {
            Ok(engine_config) => {
                let mut site_root = document_dir().unwrap();
                site_root.push("Neopoligen");
                site_root.push(engine_config.settings.active_site);
                let config = Config::new(site_root);
                build_site(&config);
                if true {
                    run_web_server(config).await;
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

fn build_site(config: &Config) {
    println!("Starting build run");
    test_templates(&config);
    let mut file_set = FileSet::new();
    file_set.load_content(&config.folders.content_root);
    file_set.load_templates(&config.folders.theme_root);
    // dbg!(&file_set);
    let builder = Builder::new(file_set, &config);
    builder.write_files();
    builder.copy_files();
    builder.copy_theme_assets();
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

// TODO: Move this over to it's own file
#[derive(Debug)]
enum TestSection {
    Description(String),
    Input(String, String),
    Output(String),
    SupportPage(String),
    Template(String, String),
}

fn test_templates(config: &Config) {
    println!("Testing templates");
    get_file_paths_for_extension(&config.folders.theme_tests_root, "neotest")
        .iter()
        .for_each(|tf| {
            let test_setup = fs::read_to_string(tf).unwrap();
            match parse_test_file(&test_setup) {
                Ok(parts) => {
                    let mut test_page_id: String = "".to_string();
                    let mut target_output: String = "".to_string();
                    let mut file_set = FileSet::new();
                    file_set.load_templates(&config.folders.theme_root);
                    parts.1.iter().for_each(|d| {
                        match d {
                            TestSection::Template(name, content) => file_set
                                .templates
                                .insert(name.to_string(), content.to_string()),
                            TestSection::Input(id, content) => {
                                test_page_id = id.to_string();
                                file_set
                                    .pages
                                    .insert(PathBuf::from("page-under-test"), content.to_string())
                            }
                            TestSection::Output(content) => {
                                target_output = content.to_string();
                                None
                            }
                            _ => None,
                        };
                    });
                    let builder = Builder::new(file_set, &config);
                    builder.files_to_output().iter().for_each(|o| {
                        let path_parts: Vec<_> =
                            o.0.components()
                                .map(|p| p.as_os_str().to_string_lossy().to_string())
                                .collect();
                        if path_parts[path_parts.len() - 2].to_string() == test_page_id.to_string()
                        {
                            if o.1.to_string() != target_output.to_string() {
                                println!("Template Error");
                                dbg!(&tf);
                                println!("{}", o.1.to_string());
                                println!("{}", target_output);
                            }
                        }
                    });
                }
                Err(e) => println!("{}", e),
            }
        });
}

fn parse_test_file(source: &str) -> IResult<&str, Vec<TestSection>> {
    let (source, sections) = many1(test_section)(source)?;
    Ok((source, sections))
}

fn test_section(source: &str) -> IResult<&str, TestSection> {
    let (source, string) = alt((
        test_desc,
        test_template,
        test_support_page,
        test_input,
        test_output,
    ))(source)?;
    Ok((source, string))
}

fn test_desc(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("### DESCRIPTION ###")(source)?;
    let (source, _) = multispace0(source)?;
    let (source, desc) = take_until("###")(source)?;
    Ok((source, TestSection::Description(desc.trim().to_string())))
}

fn test_template(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("### TEMPLATE ###")(source)?;
    let (source, _) = multispace0(source)?;
    let (source, name) = take_until("~~~")(source)?;
    let (source, _) = tag("~~~")(source)?;
    let (source, _) = multispace0(source)?;
    let (source, template) = take_until("###")(source)?;
    Ok((
        source,
        TestSection::Template(name.trim().to_string(), template.trim().to_string()),
    ))
}

fn test_support_page(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("### SUPPORT_PAGE ###")(source)?;
    let (source, _) = multispace0(source)?;
    let (source, desc) = take_until("###")(source)?;
    Ok((source, TestSection::SupportPage(desc.trim().to_string())))
}

fn test_input(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("### INPUT ###")(source)?;
    let (source, _) = multispace0(source)?;
    let (source, content) = take_until("###")(source)?;
    let (id_source, _) = take_until("-- id: ")(content)?;
    let (id_source, _) = tag("-- id: ")(id_source)?;
    let (_, id) = is_not(" \n")(id_source)?;
    Ok((
        source,
        TestSection::Input(id.trim().to_string(), content.trim().to_string()),
    ))
}

fn test_output(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("### OUTPUT ###")(source)?;
    let (source, _) = multispace0(source)?;
    let (source, content) = rest(source)?;
    Ok((source, TestSection::Output(content.trim().to_string())))
}
