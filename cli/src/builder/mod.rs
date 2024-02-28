pub mod new;

use crate::config::Config;
use crate::file_set::FileSet;
use crate::helpers::get_file_paths_for_extension::get_file_paths_for_extension;
use crate::site::Site;
use minijinja::context;
use minijinja::Environment;
use minijinja::Value;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many1;
use nom::IResult;
use std::collections::BTreeMap;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub struct Builder {
    file_set: FileSet,
    config: Config,
}

impl Builder {
    pub fn files_to_output(&self) -> BTreeMap<PathBuf, String> {
        let mut env = Environment::new();
        let site = Site::new(&self.file_set, &self.config);
        let mut outputs = BTreeMap::new();

        self.file_set
            .templates
            .iter()
            .for_each(|t| env.add_template_owned(t.0, t.1).unwrap());

        env.add_template_owned(
            "splitter.jinja".to_string(),
            r#"{#- import "includes/macros.jinja" as macros -#}
{#- include "global_vars" -#}
{%- for page_id in site.page_ids() -%}
{{ site.page_output_path(page_id) }}
--- PAGE_DATA_SPLIT ---
{% include site.page_template(page_id) %}
--- PAGE_SEPARATOR ---
{% endfor -%}"#
                .to_string(),
        )
        .unwrap();

        match env.get_template("splitter.jinja") {
            Ok(splitter) => {
                match splitter.render(context!(
                     site => Value::from_object(site),
                )) {
                    Ok(combined_pages) => {
                        combined_pages
                            .split("--- PAGE_SEPARATOR ---")
                            .for_each(|page| {
                                let page_parts: Vec<_> =
                                    page.split("--- PAGE_DATA_SPLIT ---").collect();
                                if page_parts.len() == 2 {
                                    outputs.insert(
                                        PathBuf::from(page_parts[0].trim()),
                                        page_parts[1].trim().to_string(),
                                    );
                                }
                            });
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        };
        outputs
    }

    pub fn test_templates(&self) {
        get_file_paths_for_extension(&self.config.folders.theme_tests_root, "txt")
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

    pub fn write_files(&self) {
        self.files_to_output().iter().for_each(|f| {
            let output_path = PathBuf::from(f.0);
            let parent_dir = output_path.parent().unwrap();
            let _ = create_dir_all(parent_dir);
            let _ = fs::write(output_path, f.1);
        });
    }
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
