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
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct EngineConfig {
    settings: EngineConfigSettings,
}

#[derive(Deserialize)]
pub struct EngineConfigSettings {
    active_site: String,
}

fn main() {
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

                // Testing (to move to a better location later)
                test_templates(&config);

                let mut file_set = FileSet::new();
                file_set.load_content(&config.folders.content_root);
                file_set.load_templates(&config.folders.theme_root);
                let builder = Builder::new(file_set, &config);
                builder.write_files();
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

fn test_templates(config: &Config) {
    let mut file_set = FileSet::new();
    file_set.load_templates(&config.folders.theme_root);
    get_file_paths_for_extension(&config.folders.theme_tests_root, "txt")
        .iter()
        .for_each(|tf| {
            let test_setup = fs::read_to_string(tf).unwrap();
            match parse_test_file(&test_setup) {
                Ok(parts) => {
                    dbg!(parts);
                    ()
                }
                Err(e) => println!("{}", e),
            }
        });
}

#[derive(Debug)]
enum TestSection {
    Description(String),
    Input(String, String),
    Output(String),
    SupportPage(String),
    Template(String, String),
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
    Ok((source, TestSection::SupportPage(content.trim().to_string())))
}
