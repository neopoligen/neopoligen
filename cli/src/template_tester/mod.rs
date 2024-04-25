use crate::builder::Builder;
use crate::config::Config;
use crate::file_set::FileSet;
use crate::neo_config::NeoEnv;
use crate::template_error::TemplateError;
use minijinja::{context, Environment, Value};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use tracing::{event, instrument, Level};

#[instrument(skip(config, neo_env))]
pub fn test_templates(config: &Config, neo_env: NeoEnv) {
    event!(Level::INFO, "Testing Templates");
    let mut file_set = FileSet::new();
    let mut test_config = config.clone();
    test_config.folders.content_root = PathBuf::from(format!(
        "{}/{}",
        config.folders.theme_tests_root.display().to_string(),
        "content"
    ));
    test_config.folders.images_root = PathBuf::from(format!(
        "{}/{}",
        config.folders.theme_tests_root.display().to_string(),
        "images"
    ));
    test_config.folders.mp3s_root = PathBuf::from(format!(
        "{}/{}",
        config.folders.theme_tests_root.display().to_string(),
        "mp3s"
    ));
    test_config.folders.files_root = PathBuf::from(format!(
        "{}/{}",
        config.folders.theme_tests_root.display().to_string(),
        "files"
    ));
    file_set.load_content(&test_config.folders.content_root);
    file_set.load_images(&test_config.folders.images_root);
    file_set.load_mp3s(&test_config.folders.mp3s_root);
    file_set.load_templates(&test_config.folders.theme_root);
    let mut builder = Builder::new(file_set.clone(), &test_config, &neo_env);
    builder.generate_files();

    builder.outputs_dev.iter().for_each(|output| {
        let skipped_tests: Vec<&str> = output
            .content
            .split(r#"<div class="skip-template-test-header">"#)
            .collect();
        builder.template_tests_skipped += skipped_tests.len() - 1;
        builder.template_tests_found += skipped_tests.len() - 1;
    });

    builder.outputs_dev.iter().for_each(|output| {
        let tests: Vec<&str> = output
            .content
            .split(r#"<div class="start-template-test-header">"#)
            .collect();
        builder.template_tests_run += tests.len() - 1;
        builder.template_tests_found += tests.len() - 1;
        if tests.len() > 1 {
            tests.iter().skip(1).for_each(|base| {
                let initial_split: Vec<&str> = base
                    .split("</div><!-- /start-template-test-header -->")
                    .collect();
                if initial_split.len() > 1 {
                    let description = initial_split[0].trim().to_string();
                    let expected_parts: Vec<&str> = initial_split[1]
                        .split(r#"<div class="expected-output">"#)
                        .collect();
                    if expected_parts.len() > 1 {
                        let expected = expected_parts[0].trim().to_string();
                        let got_parts: Vec<&str> = expected_parts[1]
                            .split(r#"</div><!-- /expected-output -->"#)
                            .collect();
                        if got_parts.len() > 1 {
                            let got = got_parts[0].trim().to_string();
                            let compare_expected = expected.replace("\n", "").replace(" ", "");
                            let compare_got = got.replace("\n", "").replace(" ", "");
                            if compare_expected != compare_got {
                                event!(
                                    Level::WARN,
                                    "Found mis-aligned template for: {}",
                                    &output.source_path.display()
                                );
                                builder.template_tests_errors.push(TemplateError {
                                    description,
                                    expected,
                                    got,
                                    source_path: output.source_path.display().to_string(),
                                });
                            }
                        }
                    }
                }
            })
        }
    });

    let mut env = Environment::new();
    env.add_function("highlight_code", highlight_code);
    env.add_template_owned(
        "template_error_status",
        r#"
    <div>
        <h2>Build</h2>
        <div>{{ build_time }}</div>
    </div>
    <div>
    <h2>Template Tests</h2>
        <div>Found: {{ template_tests_found }}</div>
        <div>Skipped: {{ template_tests_skipped }}</div>
        <div>Ran: {{ template_tests_run }}</div>
        <div>Error Count: {{ template_tests_error_count }}</div>
    </div>
    {% if template_tests_error_count != 0 %}
        <div class="template_errors flow">
        <h2>Errors</h2>
        {% for error in template_tests_errors %}
            <div class="template-error">
                <h4>Description</h4>
                {{ error.description }}
                <h4>File</h4>
                <div class="test-file-path">{{ error.source_path }}</div>
                <h4>Expected</h4>
                {{ highlight_code(error.expected, "html") }}
                <h4>Got</h4>
                {{ highlight_code(error.got, "html") }}
            </div>
        {% endfor %}
        </div>
    {% endif %}
    "#
        .to_string(),
    )
    .unwrap();
    let skeleton = env.get_template("template_error_status").unwrap();
    let output = skeleton
        .render(context!(
            test_page_count => &file_set.pages.len(),
            build_time => builder.build_time,
            template_tests_errors => Value::from_serialize(&builder.template_tests_errors),
            template_tests_error_count => &builder.template_tests_errors.len(),
            template_tests_found => builder.template_tests_found,
            template_tests_run => builder.template_tests_run,
            template_tests_skipped => builder.template_tests_skipped,
        ))
        .unwrap();
    let mut output_path = config.folders.status_root.clone();
    let _ = fs::create_dir_all(&output_path);
    output_path.push("template_errors.htm");
    let _ = fs::write(output_path, output);
}

fn highlight_code(code: &str, lang: &str) -> String {
    let formatted_html = simple_format_html(code);
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set
        .find_syntax_by_token(&lang)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
    for line in LinesWithEndings::from(&formatted_html) {
        let _ = html_generator.parse_html_for_line_which_includes_newline(line);
    }
    let output_html = html_generator.finalize();
    format!(r#"<pre><code>{}</code></pre>"#, output_html)
}

fn simple_format_html(code: &str) -> String {
    let mut re = Regex::new(r"\n").unwrap();
    let output = re.replace_all(code, " ");
    re = Regex::new(r" \s+").unwrap();
    let output = re.replace_all(&output, " ");
    re = Regex::new(r"\s+<").unwrap();
    let output = re.replace_all(&output, "<");
    re = Regex::new(r">\s+").unwrap();
    let output = re.replace_all(&output, ">");
    let parts: Vec<&str> = output.split("<").collect();
    let mut assembler: Vec<String> = vec![];
    let mut level = 0i8;
    parts.iter().skip(1).for_each(|part| {
        if part.starts_with("/") {
            level -= 2;
        }
        for _ in 0..level {
            assembler.push(" ".to_string());
        }
        assembler.push(format!("<{}\n", part));
        if !part.starts_with("/") {
            level += 2;
        }
    });
    assembler.join("").to_string()
}
