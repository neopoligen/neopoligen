use super::PageV42b;
// use crate::ast_v42::parse_ast_v42;
// use crate::bound::Bound;
// use crate::neo_error::NeoError;
// use crate::neo_error::NeoErrorKind;
// use crate::section_v42::basic::BasicSectionV42;
// use crate::section_v42::block::BlockSectionV42;
// use crate::section_v42::helpers::make_template_list::make_template_list;
// use crate::section_v42::parsing_error::ParsingErrorSectionV42;
// use crate::section_v42::SectionV42;
// use crate::site_config::SiteConfigV42;
// use crate::span_v42::plaintext::PlainTextSpan;
// use crate::span_v42::SpanV42;
// use minijinja::Value;
// use nom::bytes::complete::is_a;
// use nom::bytes::complete::is_not;
// use nom::bytes::complete::tag;
// use nom::character::complete::space1;
// use nom::combinator::not;
// use nom::combinator::opt;
// use nom::multi::many0;
// use nom::multi::many1;
// use nom::sequence::preceded;
// use nom::sequence::tuple;
// use nom::IResult;
// use nom::Parser;
// use std::collections::BTreeMap;
// use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};

impl PageV42b {
    #[instrument(skip(source))]
    pub fn new_from_str_v42(
        source_path: &PathBuf,
        source: &str,
        project_dir: &PathBuf,
    ) -> PageV42b {
        event!(Level::DEBUG, "{}", "Making page");
        PageV42b {
            ast: None,
            parsing_issue: None,
            href: None,
            id: None,
            issues: vec![],
            language: None,
            output: None,
            sections: None,
            source: source.to_string(),
            source_absolute_path: source_path.clone(),
            source_relative_path: source_path.clone(),
        }
    }
}

//impl PageV42 {
//    // TODO: Deprecate in favor of new_from_str_v42
//    #[instrument(skip(site_config))]
//    pub fn new_from_fs_path(
//        path: &PathBuf,
//        site_config: &SiteConfigV42,
//    ) -> Result<PageV42, NeoError> {
//        event!(Level::DEBUG, "new_from_fs_path");
//        match fs::read_to_string(path) {
//            Ok(source) => PageV42::new_from_str(&source, path, site_config),
//            Err(e) => Err(NeoError {
//                kind: NeoErrorKind::ForwardErrorWithSourcePath {
//                    source_path: path.clone(),
//                    msg: e.to_string(),
//                },
//            }),
//        }
//    }
//    // TODO: Deprecate in favor of new_from_str_v42
//    #[instrument(skip(source, site_config))]
//    pub fn new_from_str(
//        source: &str,
//        source_path: &PathBuf,
//        site_config: &SiteConfigV42,
//    ) -> Result<PageV42, NeoError> {
//        event!(Level::DEBUG, "new_from_str");
//        match parse_ast_v42(&source, site_config, false) {
//            Ok(raw_ast) => {
//                let mut page = PageV42 {
//                    id: None,
//                    html_output_path: None,
//                    raw_ast,
//                    sections: Value::from_serialize(vec![""]),
//                    source: source.to_string(),
//                    fs_path: source_path.to_path_buf(),
//                    error: None,
//                };
//                // TODO: Handle pages with no ids
//                match page.get_pagedata(&[Value::from("id")]) {
//                    Ok(id) => {
//                        page.id = Some(id.to_string());
//                        match page.get_pagedata(&[Value::from("href")]) {
//                            Ok(html_output_path) => {
//                                page.html_output_path =
//                                    Some(html_output_path.trim_start_matches("/").into())
//                            }
//                            Err(_e) => {
//                                page.html_output_path = Some(PathBuf::from(format!(
//                                    "{}/{}/index.html",
//                                    site_config.default_language().unwrap(),
//                                    page.id.as_ref().unwrap()
//                                )));
//                            }
//                        }
//                        page.sections = Value::from_serialize(page.clone().raw_ast);
//                        Ok(page)
//                    }
//                    Err(e) => {
//                        match parse_value_from_pagedata(source, "id") {
//                            Ok((_, ids)) => match ids.last() {
//                                Some(id) => {
//                                    page.id = Some(id.to_string());
//                                    match page.get_pagedata(&[Value::from("href")]) {
//                                        Ok(html_output_path) => {
//                                            page.html_output_path = Some(
//                                                html_output_path.trim_start_matches("/").into(),
//                                            )
//                                        }
//                                        Err(_e) => {
//                                            page.html_output_path = Some(PathBuf::from(format!(
//                                                "{}/{}/index.html",
//                                                site_config.default_language().unwrap(),
//                                                page.id.as_ref().unwrap()
//                                            )));
//                                        }
//                                    }
//                                    Ok(page.clone())
//                                }
//                                _ => {
//                                    event!(
//                                        Level::ERROR,
//                                        "Could not get ID for:{}",
//                                        page.fs_path.display()
//                                    );
//                                    Err(NeoError {
//                                        kind: NeoErrorKind::ForwardErrorWithSourcePath {
//                                            source_path: source_path.clone(),
//                                            msg: e.to_string(),
//                                        },
//                                    })
//                                }
//                            },
//                            _ => {
//                                event!(
//                                    Level::ERROR,
//                                    "Could not get ID for:{}",
//                                    page.fs_path.display()
//                                );
//                                Err(NeoError {
//                                    kind: NeoErrorKind::ForwardErrorWithSourcePath {
//                                        source_path: source_path.clone(),
//                                        msg: e.to_string(),
//                                    },
//                                })
//                            }
//                        }
//                        // event!(
//                        //     Level::ERROR,
//                        //     "Could not get ID for:{}",
//                        //     page.fs_path.display()
//                        // );
//                        // Err(NeoError {
//                        //     kind: NeoErrorKind::ForwardErrorWithSourcePath {
//                        //         source_path: source_path.clone(),
//                        //         msg: e.to_string(),
//                        //     },
//                        // })
//                    }
//                }
//            }
//            Err(e) => {
//                let mut page = PageV42 {
//                    id: None,
//                    html_output_path: None,
//                    raw_ast: vec![],
//                    sections: Value::from_serialize(vec![""]),
//                    source: source.to_string(),
//                    fs_path: source_path.to_path_buf(),
//                    error: Some(e.clone()),
//                };
//                match &e.kind {
//                    NeoErrorKind::ParserErrorV42 {
//                        line,
//                        column,
//                        remainder,
//                        source,
//                        message,
//                    } => {
//                        let mut output_remainder_with_padding =
//                            format!("{:>width$}", "", width = column - 1);
//                        output_remainder_with_padding.push_str(remainder.as_str());
//                        let output_source = match source.strip_suffix(remainder.as_str()) {
//                            Some(text) => text.to_string(),
//                            None => "Could not properly get initial string".to_string(),
//                        };
//                        page.raw_ast = vec![
//                            SectionV42::Basic(BasicSectionV42 {
//                                attrs: BTreeMap::new(),
//                                bound: Bound::Full,
//                                children: vec![SectionV42::Block(BlockSectionV42 {
//                                    children: vec![],
//                                    kind: "paragraph".to_string(),
//                                    spans: vec![SpanV42::PlainText(PlainTextSpan {
//                                        text: "Content Parsing Error".to_string(),
//                                    })],
//                                    template_list: vec![
//                                        "sections/title/default.neoj".to_string(),
//                                        "sections/generic-basic/default.neoj".to_string(),
//                                    ],
//                                })],
//                                end_section: None,
//                                flags: vec![],
//                                kind: "title".to_string(),
//                                template_list: make_template_list("basic", "title", "default"),
//                            }),
//                            SectionV42::ParsingError(ParsingErrorSectionV42 {
//                                attrs: BTreeMap::new(),
//                                bound: Bound::Full,
//                                children: vec![],
//                                flags: vec![],
//                                kind: "parsing_error".to_string(),
//                                template_list: vec![
//                                    "sections/error-parsing/default.neoj".to_string()
//                                ],
//                                column: *column,
//                                line: *line,
//                                output_source,
//                                output_remainder: remainder.to_string(),
//                                output_remainder_with_padding,
//                                raw_remainder: remainder.to_string(),
//                                raw_source: source.to_string(),
//                                message: message.to_string(),
//                            }),
//                        ];
//                    }
//                    _ => (),
//                };
//                // page.raw_ast = vec![
//                //     SectionV42::Basic(BasicSectionV42 {
//                //         attrs: BTreeMap::new(),
//                //         bounds: Bound::Full,
//                //         children: vec![SectionV42::Block(BlockSectionV42 {
//                //             children: vec![],
//                //             kind: "paragraph".to_string(),
//                //             spans: vec![SpanV42::PlainText(PlainTextSpan {
//                //                 text: "Content Parsing Error".to_string(),
//                //             })],
//                //             text: "Delta".to_string(),
//                //             template_list: vec![
//                //                 "sections/block/paragraph/full/default.neoj".to_string()
//                //             ],
//                //         })],
//                //         flags: vec![],
//                //         kind: "title".to_string(),
//                //         template_list: make_template_list("basic", "title", "full", "default"),
//                //     }),
//                //     SectionV42::ParsingError(ParsingErrorSectionV42 {
//                //         attrs: BTreeMap::new(),
//                //         bounds: Bound::Full,
//                //         children: vec![],
//                //         flags: vec![],
//                //         kind: "parsing_error".to_string(),
//                //         source: page.source.clone(),
//                //         template_list: vec![
//                //             "sections/built-in/parsing-error/full/default.neoj".to_string()
//                //         ],
//                //         column: 0,
//                //         line: 1,
//                //         remainder: "TODO".to_string(),
//                //         message: "TODO".to_string(),
//                //     }),
//                // ];
//                page.sections = Value::from_serialize(page.clone().raw_ast);
//                match parse_value_from_pagedata(source, "id") {
//                    Ok((_, ids)) => match ids.last() {
//                        Some(id) => {
//                            page.id = Some(id.to_string());
//                            page.html_output_path = Some(PathBuf::from(format!(
//                                "{}/{}/index.html",
//                                site_config.default_language().unwrap(),
//                                page.id.as_ref().unwrap()
//                            )));
//                            match parse_value_from_pagedata(source, "href") {
//                                Ok((_, html_output_paths)) => match html_output_paths.last() {
//                                    Some(html_output_path) => {
//                                        page.html_output_path =
//                                            Some(html_output_path.trim_start_matches("/").into());
//                                    }
//                                    _ => (),
//                                },
//                                Err(_) => (),
//                            }
//                            Ok(page)
//                        }
//                        None => Err(NeoError {
//                            kind: NeoErrorKind::ForwardErrorWithSourcePath {
//                                source_path: source_path.clone(),
//                                msg: "Could not find id in invalid parsed file".to_string(),
//                            },
//                        }),
//                    },
//                    Err(e) => Err(NeoError {
//                        kind: NeoErrorKind::ForwardErrorWithSourcePath {
//                            source_path: source_path.clone(),
//                            msg: e.to_string(),
//                        },
//                    }),
//                }
//            }
//        }
//    }
//    //
//}

// fn parse_value_from_pagedata<'a>(
//     source: &'a str,
//     target: &'a str,
// ) -> IResult<&'a str, Vec<&'a str>> {
//     let (source, _) =
//         many1(|src| move_parser_to_last_instance_of_section(src, "pagedata")).parse(source)?;
//     let (source, things) = many1(|src| parse_pagedata_attr(src, target)).parse(source)?;
//     Ok((source, things))
// }

// fn move_parser_to_last_instance_of_section<'a>(
//     source: &'a str,
//     target: &'a str,
// ) -> IResult<&'a str, &'a str> {
//     let (source, _) = many0(tuple((
//         opt(is_not("-")),
//         not(tuple((tag("--"), space1, tag(target)))),
//         is_a("-"),
//     )))
//     .parse(source)?;
//     let (source, _) = tuple((opt(is_not("-")), tag("--"), space1, tag(target))).parse(source)?;
//     Ok((source, ""))
// }

// fn parse_pagedata_attr<'a>(source: &'a str, target: &'a str) -> IResult<&'a str, &'a str> {
//     let (source, _) = many0(tuple((
//         opt(is_not("-")),
//         not(tuple((tag("--"), space1, tag(target), tag(":")))),
//         is_a("-"),
//     )))
//     .parse(source)?;
//     let (source, id) = preceded(
//         tuple((
//             is_not("-"),
//             tag("--"),
//             space1,
//             tag(target),
//             tag(":"),
//             space1,
//         )),
//         is_not("\n\r"),
//     )
//     .parse(source)?;
//     Ok((source, id.trim()))
// }

#[cfg(test)]
mod test {

    // use super::*;
    // use crate::helpers::get_include_str_file;
    // use crate::site_config::SiteConfigV42;
    // use include_dir::{include_dir, Dir};
    // use pretty_assertions::assert_eq;

    // const SAMPLES: Dir = include_dir!("src/page_v42/samples");

    // #[test]
    // fn basic_load_test_get_id() {
    //     let source = get_include_str_file(SAMPLES, "001-pagedata-only.neo");
    //     let source_path = PathBuf::from("mock/source/path/file.neo");
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
    //     let left = "001-pagedata-only.neo";
    //     let right = page.id.unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn default_get_html_output_path() {
    //     let source = get_include_str_file(SAMPLES, "001-pagedata-only.neo");
    //     let source_path = PathBuf::from("mock/source/path/file.neo");
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
    //     let left = PathBuf::from("en/001-pagedata-only.neo/index.html");
    //     let right = page.html_output_path.unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn custom_html_output_path_from_pagedata_in_valid_file() {
    //     let source = get_include_str_file(SAMPLES, "003-custom-href-in-pagedata.neo");
    //     let source_path = PathBuf::from("mock/source/path/file.neo");
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
    //     let left = PathBuf::from("index.html");
    //     let right = page.html_output_path.unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn custom_html_output_path_from_pagedata_in_invalid_file() {
    //     let source = get_include_str_file(SAMPLES, "007-unclosed-with-path.neo");
    //     let source_path = PathBuf::from("mock/007-unclosed-with-path.neo");
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
    //     let left = PathBuf::from("some/custom/path/index.html");
    //     let right = page.html_output_path.unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn get_id_when_there_is_an_error() {
    //     let source = get_include_str_file(SAMPLES, "006-unclosed-start-section.neo");
    //     let source_path = PathBuf::from("mock/source/path/file.neo");
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
    //     let left = "006-unclosed-start-section.neo";
    //     let right = page.id.clone().unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn get_id_when_there_is_an_error_and_something_else_also_has_an_id() {
    //     let id = "008-unclosed-with-other-id.neo";
    //     let source = get_include_str_file(SAMPLES, id);
    //     let source_path = PathBuf::from(format!("mock/{}", id));
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let page = PageV42::new_from_str(&source, &source_path, &site_config).unwrap();
    //     let left = id;
    //     let right = page.id.clone().unwrap();
    //     assert_eq!(left, right);
    // }

    //
}
