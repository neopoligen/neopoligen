use crate::builder::Builder;
use crate::config::Config;
use crate::file_set::FileSet;
use crate::neo_config::NeoEnv;
use crate::template_error::TemplateError;
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
    let mut builder = Builder::new(file_set, &test_config, &neo_env);

    builder.files_to_output().iter().for_each(|output| {
        let body_parts: Vec<&str> = output.1.split("### EXPECTED_OUTPUT ###").collect();
        if body_parts.len() == 2 {
            if body_parts[0] != body_parts[1] {
                builder.template_errors.push(TemplateError {
                    id: "asdf".to_string(),
                    expected: body_parts[0].to_string(),
                    got: body_parts[1].to_string(),
                });
            }
        }
    });
}
