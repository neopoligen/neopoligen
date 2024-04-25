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
            template_errors: vec![],
            outputs: BTreeMap::new(), // Deprecated. move to outputs_dev
            outputs_dev: vec![],
            build_time: None,
            skipped_template_tests: 0,
        }
    }
}
