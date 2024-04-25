use std::collections::BTreeMap;

use crate::builder::Builder;
use crate::config::Config;
use crate::file_set::FileSet;
use crate::neo_config::NeoEnv;

impl Builder {
    pub fn new(file_set: FileSet, config: &Config, neo_env: &NeoEnv) -> Builder {
        Builder {
            file_set,
            config: config.clone(),
            neo_env: neo_env.clone(),
            outputs: BTreeMap::new(), // Deprecated. move to outputs_dev
            outputs_dev: vec![],
            build_time: None,
            // template_tests_file_count: 0,
            template_tests_found: 0,
            template_tests_run: 0,
            template_tests_skipped: 0,
            template_tests_errors: vec![],
        }
    }
}
