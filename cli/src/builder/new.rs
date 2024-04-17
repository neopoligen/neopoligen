use crate::builder::Builder;
use crate::config::Config;
use crate::file_set::FileSet;
use crate::neo_config::NeoConfig;

impl Builder {
    pub fn new(file_set: FileSet, config: &Config, _engine_config: &NeoConfig) -> Builder {
        Builder {
            file_set,
            config: config.clone(),
        }
    }
}
