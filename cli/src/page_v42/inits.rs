use super::PageV42;
use crate::ast_v42::parse_ast_v42;
use crate::neo_error::NeoError;
use crate::neo_error::NeoErrorKind;
use crate::site_config::SiteConfigV42;
use minijinja::Value;
use std::fs;
use std::path::PathBuf;
// use tracing::{event, instrument, Level};

impl PageV42 {
    //#[instrument()]
    pub fn new_from_fs_path(
        path: &PathBuf,
        site_config: &SiteConfigV42,
    ) -> Result<PageV42, NeoError> {
        match fs::read_to_string(path) {
            Ok(source) => match parse_ast_v42(&source, site_config) {
                Ok(raw_ast) => Ok(PageV42 {
                    id: None,
                    error: None,
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

    pub fn new_from_str(
        source: &str,
        source_path: &PathBuf,
        site_config: &SiteConfigV42,
    ) -> Result<PageV42, NeoError> {
        match parse_ast_v42(&source, site_config) {
            Ok(raw_ast) => {
                let mut p = PageV42 {
                    id: None,
                    error: None,
                    raw_ast,
                    source: source.to_string(),
                    fs_path: source_path.to_path_buf(),
                };
                // TODO: Handle missing page ids
                p.id = Some(p.get_pagedata(&[Value::from("id")]).unwrap());
                Ok(p)
            }
            Err(e) => Err(NeoError {
                kind: NeoErrorKind::ForwardErrorWithSourcePath {
                    source_path: source_path.to_path_buf(),
                    msg: e.to_string(),
                },
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers::get_include_str_file;
    use crate::site_config::SiteConfigV42;
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLES: Dir = include_dir!("src/page_v42/samples/valid");

    #[test]
    fn solo_basic_load_test() {
        let source = get_include_str_file(SAMPLES, "001-pagedata-only.neo");
        let source_path = PathBuf::from("mock/source/path/file.neo");
        let site_config = SiteConfigV42::mock1_basic();
        let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
        let left = "001-pagedata-only.neo";
        let right = page.id.unwrap();
        assert_eq!(left, right);
    }
}
