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
        }
    }
}
