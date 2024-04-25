use crate::builder::Builder;
use crate::config::Config;
use crate::file_set::FileSet;
use crate::neo_config::NeoEnv;
use crate::template_error::TemplateError;
use minijinja::{context, Environment, Value};
use std::fs;
use std::path::PathBuf;
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

    builder.outputs.iter().for_each(|output| {
        let tests: Vec<&str> = output
            .1
            .split(r#"<div class="start-template-test-header">"#)
            .collect();
        if tests.len() > 1 {
            tests.iter().skip(1).for_each(|base| {
                let initial_split: Vec<&str> = base
                    .split("</div><!-- /start-tempalte-test-header -->")
                    .collect();
                if initial_split.len() == 2 {
                    let description = initial_split[0].trim().to_string();
                    let expected_parts: Vec<&str> = initial_split[1]
                        .split(r#"<div class="expected-output">"#)
                        .collect();
                    if expected_parts.len() == 2 {
                        let expected = expected_parts[0].trim().to_string();
                        let got_parts: Vec<&str> = expected_parts[1]
                            .split(r#"</div><!-- /expected-output -->"#)
                            .collect();
                        if got_parts.len() == 2 {
                            let got = got_parts[0].trim().to_string();
                            let compare_expected = expected.replace("\n", "").replace(" ", "");
                            let compare_got = got.replace("\n", "").replace(" ", "");

                            if compare_expected != compare_got {
                                event!(
                                    Level::WARN,
                                    "Found mis-aligned template for: {}",
                                    &output.0.display()
                                );

                                let parent_dir = output.0.parent().unwrap();
                                let id = parent_dir
                                    .file_stem()
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string();
                                builder.template_errors.push(TemplateError {
                                    id,
                                    description,
                                    expected,
                                    got,
                                });
                            }
                        }
                    }
                    //dbg!(initial_split[1]);
                }
            })
            //

            //    if initial_split.len() > 1 {
            //        let t = initial_split[1];
            //        let body_parts: Vec<&str> = t
            //            .split(r#"<div class="expected-output-split"></div>"#)
            //            .collect();
            //        let parent_dir = output.0.parent().unwrap();
            //        let id = parent_dir.file_stem().unwrap().to_string_lossy();
            //        if body_parts.len() > 1 {
            //            let compare_start = body_parts[0].replace("\n", "").replace(" ", "");
            //            let compare_end = body_parts[1].replace("\n", "").replace(" ", "");
            //            if compare_start != compare_end {
            //                event!(
            //                    Level::WARN,
            //                    "Found mis-aligned template for: {}",
            //                    &output.0.display()
            //                );
            //                builder.template_errors.push(TemplateError {
            //                    id: id.to_string(),
            //                    expected: body_parts[1].to_string(),
            //                    got: body_parts[0].to_string(),
            //                });
            //            }
            //        }
            //        //})
            //    }
        }
    });

    let mut env = Environment::new();
    env.add_template_owned(
        "template_error_status",
        r#"
    <div>Ran {{ test_page_count }} Template Tests. Found {{ template_error_count }} Errors</div>
    <div>{{ build_time }}</div>
    {% for error in template_errors %}
        <div class="template-error">
        <h2>{{ error.id }}</h2>
        <h3>Description</h3>
        {{ error.description }}
        <h3>Expected</h3>
        <pre>{% autoescape true %}{{ error.expected }}{% endautoescape %}</pre>
        <h3>Got</h3>
        <pre>{% autoescape true %}{{ error.got }}{% endautoescape %}</pre>
        </div>
    {% endfor %}"#
            .to_string(),
    )
    .unwrap();
    let skeleton = env.get_template("template_error_status").unwrap();
    let output = skeleton
        .render(context!(
            test_page_count => &file_set.pages.len(),
            template_error_count => &builder.template_errors.len(),
            template_errors => Value::from_serialize(&builder.template_errors),
            build_time => builder.build_time
        ))
        .unwrap();
    let mut output_path = config.folders.status_root.clone();
    let _ = fs::create_dir_all(&output_path);
    output_path.push("template_errors.htm");
    let _ = fs::write(output_path, output);
}
