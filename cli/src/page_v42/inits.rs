use super::PageV42;
use crate::ast_v42::parse_ast_v42;
use crate::neo_error::NeoError;
use crate::neo_error::NeoErrorKind;
use crate::site_config::SiteConfigV42;
use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};

impl PageV42 {
    #[instrument()]
    pub fn new_from_fs_path(
        path: &PathBuf,
        site_config: &SiteConfigV42,
    ) -> Result<PageV42, NeoError> {
        match fs::read_to_string(path) {
            Ok(source) => match parse_ast_v42(&source, site_config) {
                Ok(raw_ast) => Ok(PageV42 {
                    raw_ast,
                    source: source.to_string(),
                    fs_path: PathBuf::from("todo/add/fs_path/to/page_v42"),
                }),
                Err(e) => Err(NeoError {
                    kind: NeoErrorKind::ForwardErrorWithSourcePath {
                        source_path: path.to_path_buf(),
                        msg: e.to_string(),
                    },
                }),
            },
            Err(e) => Err(NeoError {
                kind: NeoErrorKind::ForwardErrorWithSourcePath {
                    source_path: path.clone(),
                    msg: e.to_string(),
                },
            }),
        }
    }
}
