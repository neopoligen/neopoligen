use crate::builder::Builder;
use crate::config::Config;
use crate::file_set::FileSet;

impl Builder {
    pub fn new(file_set: FileSet, config: &Config) -> Builder {
        Builder {
            file_set,
            config: config.clone(),
        }
    }
}
