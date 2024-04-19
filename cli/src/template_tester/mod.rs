use crate::builder::Builder;
use crate::config::Config;
use crate::file_set::FileSet;
use crate::helpers::get_file_paths_for_extension::get_file_paths_for_extension;
use crate::neo_config::NeoEnv;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::rest;
use nom::multi::many1;
use nom::IResult;
use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};

#[derive(Debug)]
pub enum TestSection {
    Description(String),
    Input(String, String, String),
    Output(String),
    SupportPage(String, String, String),
    Template(String, String),
}

#[instrument(skip(config, neo_env))]
pub fn test_templates(config: &Config, neo_env: NeoEnv) {
    let _ = fs::remove_dir_all(&config.folders.theme_errors_root);
    event!(Level::INFO, "{}", "Testing Tempaltes");
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
                            TestSection::Template(name, content) => {
                                // dbg!(&name.to_string());
                                file_set
                                    .templates
                                    .insert(name.to_string(), content.to_string())
                            }
                            TestSection::Input(path, id, content) => {
                                test_page_id = id.to_string();
                                file_set
                                    .pages
                                    .insert(PathBuf::from(path), content.to_string())
                            }
                            TestSection::SupportPage(path, id, content) => {
                                test_page_id = id.to_string();
                                file_set
                                    .pages
                                    .insert(PathBuf::from(path), content.to_string())
                            }
                            TestSection::Output(content) => {
                                target_output = content.to_string();
                                None
                            }
                            _ => None,
                        };
                    });
                    // let _ = file_set.pages.iter().for_each(|p| {
                    //     dbg!(p);
                    //     ()
                    // });
                    let builder = Builder::new(file_set, &config, &neo_env);
                    builder.files_to_output().iter().for_each(|o| {
                        let path_parts: Vec<_> =
                            o.0.components()
                                .map(|p| p.as_os_str().to_string_lossy().to_string())
                                .collect();
                        if path_parts[path_parts.len() - 2].to_string() == test_page_id.to_string()
                        {
                            // dbg!(&test_page_id);
                            if o.1.replace(" ", "").replace("\n", "").to_string()
                                != target_output.replace(" ", "").replace("\n", "").to_string()
                            {
                                println!("Template Error");
                                let mut output_path = config.folders.theme_errors_root.clone();
                                let error_file_sub_path = tf
                                    .strip_prefix(config.folders.theme_tests_root.clone())
                                    .unwrap();
                                output_path.push(error_file_sub_path.with_extension("html"));
                                let _ = fs::create_dir_all(output_path.parent().unwrap());
                                let theme_error_content =
                                    format!("<!DOCTYPE html><html><body><h2>EXPECTED</h2>\n\n\n{}\n\n\n<h2>GOT</h2>\n\n\n{}\n\n\n<!-- spacer --></body></html>", target_output, o.1);
                                let _ = fs::write(output_path, theme_error_content);
                            }
                        }
                    });
                }
                Err(e) => println!("{}", e),
            }
        });
}

pub fn parse_test_file(source: &str) -> IResult<&str, Vec<TestSection>> {
    let (source, sections) = many1(test_section)(source)?;
    Ok((source, sections))
}

pub fn test_section(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = test_spacer_line(source)?;
    let (source, _) = multispace0(source)?;
    let (source, string) = alt((
        test_desc,
        test_template,
        test_support_page,
        test_input,
        test_output,
    ))(source)?;
    Ok((source, string))
}

pub fn test_spacer_line(source: &str) -> IResult<&str, &str> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("##")(source)?;
    let (source, _) = is_a("#")(source)?;
    let (source, _) = line_ending(source)?;
    Ok((source, ""))
}

pub fn test_desc(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = tag("DESCRIPTION")(source)?;
    let (source, _) = test_spacer_line(source)?;
    let (source, desc) = take_until("###")(source)?;
    Ok((source, TestSection::Description(desc.trim().to_string())))
}

pub fn test_template(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = tag("TEMPLATE")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, _) = tag("PATH:")(source)?;
    let (source, _) = space1(source)?;
    let (source, name) = not_line_ending(source)?;
    let (source, _) = line_ending(source)?;
    let (source, _) = test_spacer_line(source)?;
    let (source, template) = take_until("###")(source)?;
    Ok((
        source,
        TestSection::Template(name.trim().to_string(), template.trim().to_string()),
    ))
}

pub fn test_input(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = tag("INPUT")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, _) = tag("PATH:")(source)?;
    let (source, _) = space1(source)?;
    let (source, path) = not_line_ending(source)?;
    let (source, _) = line_ending(source)?;
    let (source, _) = test_spacer_line(source)?;
    let (source, content) = take_until("###")(source)?;
    let (id_source, _) = take_until("-- id: ")(content)?;
    let (id_source, _) = tag("-- id: ")(id_source)?;
    let (_, id) = is_not(" \n")(id_source)?;
    Ok((
        source,
        TestSection::Input(
            path.trim().to_string(),
            id.trim().to_string(),
            content.trim().to_string(),
        ),
    ))
}

pub fn test_support_page(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = tag("SUPPORT_PAGE")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, _) = tag("PATH:")(source)?;
    let (source, _) = space1(source)?;
    let (source, path) = not_line_ending(source)?;
    let (source, _) = line_ending(source)?;
    let (source, _) = test_spacer_line(source)?;
    let (source, content) = take_until("###")(source)?;
    let (id_source, _) = take_until("-- id: ")(content)?;
    let (id_source, _) = tag("-- id: ")(id_source)?;
    let (_, id) = is_not(" \n")(id_source)?;
    Ok((
        source,
        TestSection::Input(
            path.trim().to_string(),
            id.trim().to_string(),
            content.trim().to_string(),
        ),
    ))
}

pub fn test_output(source: &str) -> IResult<&str, TestSection> {
    let (source, _) = tag("EXPECTED_OUTPUT")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, _) = test_spacer_line(source)?;
    let (source, content) = rest(source)?;
    Ok((source, TestSection::Output(content.trim().to_string())))
}
