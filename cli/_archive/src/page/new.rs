use crate::config::Config;
use crate::page::parse::parse;
use crate::page::*;
use tracing::instrument;

impl Page {
    #[instrument]
    pub fn new(source_path: PathBuf, source: &str, config: Config) -> Page {
        // TODO: Remove_this and just use .folders()
        let folder_path = match source_path
            .parent()
            .unwrap()
            .strip_prefix(config.folders.site_production_content_root.clone())
        {
            Ok(p) => p.to_path_buf(),
            // TODO: Panic on this error, but you'll need to clean
            // up a bunch of stuff probably
            Err(e) => panic!("{}", e),
        };

        Page {
            ast: parse(source, &config),
            config,
            source_path,
            // folder_path,
            source: source.to_string(),
            site: None,
        }
    }
}
