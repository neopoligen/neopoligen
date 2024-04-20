// use crate::builder::Builder;
use crate::config::Config;
// use crate::file_set::FileSet;
// use crate::helpers::get_file_paths_for_extension::get_file_paths_for_extension;
use crate::neo_config::NeoEnv;
// use std::fs;
// use std::path::PathBuf;

use tracing::{event, instrument, Level};

#[instrument(skip(_config, _neo_env))]
pub fn test_templates(_config: &Config, _neo_env: NeoEnv) {
    event!(Level::INFO, "Testing Templates");
}
