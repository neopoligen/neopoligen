use super::PageV42;
use crate::ast_v42::parse_ast_v42;
use crate::neo_error::NeoError;
use crate::neo_error::NeoErrorKind;
use crate::site_config::SiteConfigV42;
use minijinja::Value;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
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
            Ok(source) => PageV42::new_from_str(&source, path, site_config),
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
                    html_output_path: None,
                    raw_ast,
                    source: source.to_string(),
                    fs_path: source_path.to_path_buf(),
                };
                // TODO: Handle pages with no ids
                p.id = Some(p.get_pagedata(&[Value::from("id")]).unwrap());
                Ok(p)
            }
            Err(_e) => {
                let mut p = PageV42 {
                    id: None,
                    error: None,
                    html_output_path: None,
                    raw_ast: vec![],
                    source: source.to_string(),
                    fs_path: source_path.to_path_buf(),
                };
                match get_id(&source) {
                    Ok((_, id)) => p.id = Some(id.to_string()),
                    // TODO: Deal with this error
                    Err(_e) => (),
                }
                Ok(p)
                // Err(NeoError {
                //     kind: NeoErrorKind::ForwardErrorWithSourcePath {
                //         source_path: source_path.to_path_buf(),
                //         msg: e.to_string(),
                //     },
                // })
            }
        }
    }
}

fn get_id(source: &str) -> IResult<&str, &str> {
    let (source, _) =
        many0(tuple((opt(is_not("-")), not(tag("-- id:")), is_a("-")))).parse(source)?;
    let (source, id) = preceded(
        tuple((is_not("-"), tag("--"), space1, tag("id:"), space1)),
        is_not("\n\r"),
    )
    .parse(source)?;
    Ok((source, id.trim()))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers::get_include_str_file;
    use crate::site_config::SiteConfigV42;
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const VALID_SAMPLES: Dir = include_dir!("src/page_v42/samples/valid");
    const INVALID_SAMPLES: Dir = include_dir!("src/page_v42/samples/invalid");

    #[test]
    fn basic_load_test() {
        let source = get_include_str_file(VALID_SAMPLES, "001-pagedata-only.neo");
        let source_path = PathBuf::from("mock/source/path/file.neo");
        let site_config = SiteConfigV42::mock1_basic();
        let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
        let left = "001-pagedata-only.neo";
        let right = page.id.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_get_id_when_there_is_an_error() {
        let source = get_include_str_file(INVALID_SAMPLES, "001-unclosed-start-section.neo");
        let source_path = PathBuf::from("mock/source/path/file.neo");
        let site_config = SiteConfigV42::mock1_basic();
        let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
        let left = "001-unclosed-start-section.neo";
        let right = page.id.clone().unwrap();
        assert_eq!(left, right);
    }
}
