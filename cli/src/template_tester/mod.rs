use crate::builder::Builder;
use crate::config::Config;
use crate::file_set::FileSet;
// use crate::helpers::get_file_paths_for_extension::get_file_paths_for_extension;
use crate::neo_config::NeoEnv;
// use std::fs;
// use std::path::PathBuf;

use tracing::{event, instrument, Level};

#[instrument(skip(config, neo_env))]
pub fn test_templates(config: &Config, neo_env: NeoEnv) {
    event!(Level::INFO, "Testing Templates");

    let mut file_set = FileSet::new();

    let mut test_config = config.clone();
    dbg!(test_config);

    // let mut test_content_root = config.folders.theme_tests_root.clone();
    // test_content_root.push("content");
    // file_set.load_content(&test_content_root);

    // let mut test_images_root = config.folders.theme_tests_root.clone();
    // test_images_root.push("images");
    // file_set.load_images(&test_images_root);

    // let mut test_mp3s_root = config.folders.theme_tests_root.clone();
    // test_mp3s_root.push("mp3s");
    // file_set.load_mp3s(&test_mp3s_root);

    //    file_set.load_templates(&config.folders.theme_root);

    //   let builder = Builder::new(file_set, &config, &neo_env);

    //  dbg!(builder.files_to_output());
}
