use super::PageV42;
use crate::ast_v42::parse_ast_v42;
use crate::bound::Bound;
use crate::neo_error::NeoError;
use crate::neo_error::NeoErrorKind;
use crate::section_v42::basic::BasicSectionV42;
use crate::section_v42::block::BlockSectionV42;
use crate::section_v42::helpers::make_template_list::make_template_list;
use crate::section_v42::parsing_error::ParsingErrorSectionV42;
use crate::section_v42::SectionV42;
use crate::site_config::SiteConfigV42;
use crate::span_v42::plaintext::PlainTextSpan;
use crate::span_v42::SpanV42;
use minijinja::Value;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};

impl PageV42 {
    //

    #[instrument(skip(site_config))]
    pub fn new_from_fs_path(
        path: &PathBuf,
        site_config: &SiteConfigV42,
    ) -> Result<PageV42, NeoError> {
        event!(Level::DEBUG, "new_from_fs_path");
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

    #[instrument(skip(source, site_config))]
    pub fn new_from_str(
        source: &str,
        source_path: &PathBuf,
        site_config: &SiteConfigV42,
    ) -> Result<PageV42, NeoError> {
        event!(Level::DEBUG, "new_from_str");
        match parse_ast_v42(&source, site_config) {
            Ok(raw_ast) => {
                let mut page = PageV42 {
                    id: None,
                    html_output_path: None,
                    raw_ast,
                    sections: Value::from_serialize(vec![""]),
                    source: source.to_string(),
                    fs_path: source_path.to_path_buf(),
                };
                // TODO: Handle pages with no ids
                match page.get_pagedata(&[Value::from("id")]) {
                    Ok(id) => {
                        page.id = Some(id.to_string());
                        match page.get_pagedata(&[Value::from("path")]) {
                            Ok(html_output_path) => {
                                page.html_output_path =
                                    Some(html_output_path.trim_start_matches("/").into())
                            }
                            Err(_e) => {
                                page.html_output_path = Some(PathBuf::from(format!(
                                    "{}/{}/index.html",
                                    site_config.default_language().unwrap(),
                                    page.id.as_ref().unwrap()
                                )));
                            }
                        }
                        page.sections = Value::from_serialize(page.clone().raw_ast);
                        Ok(page)
                    }
                    Err(e) => Err(NeoError {
                        kind: NeoErrorKind::ForwardErrorWithSourcePath {
                            source_path: source_path.clone(),
                            msg: e.to_string(),
                        },
                    }),
                }
            }
            Err(e) => {
                let mut page = PageV42 {
                    id: None,
                    html_output_path: None,
                    raw_ast: vec![],
                    sections: Value::from_serialize(vec![""]),
                    source: source.to_string(),
                    fs_path: source_path.to_path_buf(),
                };

                match &e.kind {
                    NeoErrorKind::ParserErrorV42 {
                        line,
                        column,
                        remainder,
                        source,
                        message,
                    } => {
                        page.raw_ast = vec![
                            SectionV42::Basic(BasicSectionV42 {
                                attrs: BTreeMap::new(),
                                bounds: Bound::Full,
                                children: vec![SectionV42::Block(BlockSectionV42 {
                                    children: vec![],
                                    kind: "paragraph".to_string(),
                                    spans: vec![SpanV42::PlainText(PlainTextSpan {
                                        output: "Content Parsing Error".to_string(),
                                    })],
                                    text: "Delta".to_string(),
                                    template_list: vec![
                                        "sections/block/paragraph/full/default.neoj".to_string(),
                                    ],
                                })],
                                flags: vec![],
                                kind: "title".to_string(),
                                template_list: make_template_list(
                                    "basic", "title", "full", "default",
                                ),
                            }),
                            SectionV42::ParsingError(ParsingErrorSectionV42 {
                                attrs: BTreeMap::new(),
                                bounds: Bound::Full,
                                children: vec![],
                                flags: vec![],
                                kind: "parsing_error".to_string(),
                                source: page.source.clone(),
                                template_list: vec![
                                    "sections/built-in/parsing-error/full/default.neoj".to_string(),
                                ],
                                column: *column,
                                line: *line,
                                remainder: remainder.to_string(),
                                message: message.to_string(),
                            }),
                        ];
                    }
                    _ => (),
                };

                // page.raw_ast = vec![
                //     SectionV42::Basic(BasicSectionV42 {
                //         attrs: BTreeMap::new(),
                //         bounds: Bound::Full,
                //         children: vec![SectionV42::Block(BlockSectionV42 {
                //             children: vec![],
                //             kind: "paragraph".to_string(),
                //             spans: vec![SpanV42::PlainText(PlainTextSpan {
                //                 output: "Content Parsing Error".to_string(),
                //             })],
                //             text: "Delta".to_string(),
                //             template_list: vec![
                //                 "sections/block/paragraph/full/default.neoj".to_string()
                //             ],
                //         })],
                //         flags: vec![],
                //         kind: "title".to_string(),
                //         template_list: make_template_list("basic", "title", "full", "default"),
                //     }),
                //     SectionV42::ParsingError(ParsingErrorSectionV42 {
                //         attrs: BTreeMap::new(),
                //         bounds: Bound::Full,
                //         children: vec![],
                //         flags: vec![],
                //         kind: "parsing_error".to_string(),
                //         source: page.source.clone(),
                //         template_list: vec![
                //             "sections/built-in/parsing-error/full/default.neoj".to_string()
                //         ],
                //         column: 0,
                //         line: 1,
                //         remainder: "TODO".to_string(),
                //         message: "TODO".to_string(),
                //     }),
                // ];

                page.sections = Value::from_serialize(page.clone().raw_ast);
                match parse_value_from_pagedata(source, "id") {
                    Ok((_, ids)) => match ids.last() {
                        Some(id) => {
                            page.id = Some(id.to_string());
                            page.html_output_path = Some(PathBuf::from(format!(
                                "{}/{}/index.html",
                                site_config.default_language().unwrap(),
                                page.id.as_ref().unwrap()
                            )));
                            match parse_value_from_pagedata(source, "path") {
                                Ok((_, html_output_paths)) => match html_output_paths.last() {
                                    Some(html_output_path) => {
                                        page.html_output_path =
                                            Some(html_output_path.trim_start_matches("/").into());
                                    }
                                    _ => (),
                                },
                                Err(_) => (),
                            }
                            Ok(page)
                        }
                        None => Err(NeoError {
                            kind: NeoErrorKind::ForwardErrorWithSourcePath {
                                source_path: source_path.clone(),
                                msg: "Could not find id in invalid parsed file".to_string(),
                            },
                        }),
                    },
                    Err(e) => Err(NeoError {
                        kind: NeoErrorKind::ForwardErrorWithSourcePath {
                            source_path: source_path.clone(),
                            msg: e.to_string(),
                        },
                    }),
                }
            }
        }
    }

    //
}

fn parse_value_from_pagedata<'a>(
    source: &'a str,
    target: &'a str,
) -> IResult<&'a str, Vec<&'a str>> {
    let (source, _) =
        many1(|src| move_parser_to_last_instance_of_section(src, "pagedata")).parse(source)?;
    let (source, things) = many1(|src| parse_pagedata_attr(src, target)).parse(source)?;
    Ok((source, things))
}

fn move_parser_to_last_instance_of_section<'a>(
    source: &'a str,
    target: &'a str,
) -> IResult<&'a str, &'a str> {
    let (source, _) = many0(tuple((
        opt(is_not("-")),
        not(tuple((tag("--"), space1, tag(target)))),
        is_a("-"),
    )))
    .parse(source)?;
    let (source, _) = tuple((opt(is_not("-")), tag("--"), space1, tag(target))).parse(source)?;
    Ok((source, ""))
}

fn parse_pagedata_attr<'a>(source: &'a str, target: &'a str) -> IResult<&'a str, &'a str> {
    let (source, _) = many0(tuple((
        opt(is_not("-")),
        not(tuple((tag("--"), space1, tag(target), tag(":")))),
        is_a("-"),
    )))
    .parse(source)?;
    let (source, id) = preceded(
        tuple((
            is_not("-"),
            tag("--"),
            space1,
            tag(target),
            tag(":"),
            space1,
        )),
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
    fn basic_load_test_get_id() {
        let source = get_include_str_file(VALID_SAMPLES, "001-pagedata-only.neo");
        let source_path = PathBuf::from("mock/source/path/file.neo");
        let site_config = SiteConfigV42::mock1_basic();
        let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
        let left = "001-pagedata-only.neo";
        let right = page.id.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn default_get_html_output_path() {
        let source = get_include_str_file(VALID_SAMPLES, "001-pagedata-only.neo");
        let source_path = PathBuf::from("mock/source/path/file.neo");
        let site_config = SiteConfigV42::mock1_basic();
        let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
        let left = PathBuf::from("en/001-pagedata-only.neo/index.html");
        let right = page.html_output_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn custom_html_output_path_from_pagedata_in_valid_file() {
        let source = get_include_str_file(VALID_SAMPLES, "003-custom-path-in-pagedata.neo");
        let source_path = PathBuf::from("mock/source/path/file.neo");
        let site_config = SiteConfigV42::mock1_basic();
        let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
        let left = PathBuf::from("index.html");
        let right = page.html_output_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_custom_html_output_path_from_pagedata_in_invalid_file() {
        let source = get_include_str_file(INVALID_SAMPLES, "002-unclosed-with-path.neo");
        let source_path = PathBuf::from("mock/002-unclosed-with-path.neo");
        let site_config = SiteConfigV42::mock1_basic();
        let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
        let left = PathBuf::from("some/custom/path/index.html");
        let right = page.html_output_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn get_id_when_there_is_an_error() {
        let source = get_include_str_file(INVALID_SAMPLES, "001-unclosed-start-section.neo");
        let source_path = PathBuf::from("mock/source/path/file.neo");
        let site_config = SiteConfigV42::mock1_basic();
        let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
        let left = "001-unclosed-start-section.neo";
        let right = page.id.clone().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_get_id_when_there_is_an_error_and_something_else_also_has_an_id() {
        let id = "003-unclosed-with-other-id.neo";
        let source = get_include_str_file(INVALID_SAMPLES, id);
        let source_path = PathBuf::from(format!("mock/{}", id));
        let site_config = SiteConfigV42::mock1_basic();
        let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
        let left = id;
        let right = page.id.clone().unwrap();
        assert_eq!(left, right);
    }
}
