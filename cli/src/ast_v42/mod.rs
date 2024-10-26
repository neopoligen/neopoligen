#![allow(unused_imports)]
#![allow(unused_variables)]

use std::path::PathBuf;

use crate::neo_error::*;
use crate::section::*;
use crate::site_config::ConfigSections;
use nom::character::complete::multispace0;
//use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::final_parser::final_parser;
use nom_supreme::final_parser::Location;
use nom_supreme::final_parser::RecreateContext;
//use nom_supreme::parser_ext::ParserExt;

pub fn parse_ast_v42(
    source: &str,
    sections: ConfigSections,
    fs_path: &PathBuf,
    debug: bool,
) -> Result<Vec<Section>, NeoError> {
    match final_parser(|src| do_parse_v42(src, &sections, debug))(source) {
        Ok(data) => Ok(data),
        Err(e) => Err(get_error_v42(source, &e, fs_path).into()),
    }
}

fn do_parse_v42<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    debug: bool,
) -> IResult<&'a str, Vec<Section>, ErrorTree<&'a str>> {
    let (source, result) = multispace0(source)?;
    // let temp_debug_flag = false;
    // // The 0, 0, 0 below are for the nest, list, and checklist levels, they are followed by the
    // // debug flag
    // let (source, result) =
    //     many1(|src| start_or_full_section(src, &sections, 0, 0, 0, temp_debug_flag))
    //         .context("page")
    //         .parse(source)?;
    Ok((source, vec![]))
}

fn get_error_v42(content: &str, tree: &ErrorTree<&str>, fs_path: &PathBuf) -> NeoError {
    match tree {
        GenericErrorTree::Base { location, kind } => {
            let details = Location::recreate_context(content, location);
            NeoError {
                kind: NeoErrorKind::ParserError {
                    source_path: Some(fs_path.clone()),
                    line: details.line,
                    column: details.column,
                    source: content.to_string(),
                    remainder: location.to_string(),
                    message: kind.to_string(),
                },
            }
        }
        GenericErrorTree::Stack { contexts, .. } => {
            let context = contexts[0];
            let details = Location::recreate_context(content, context.0);
            NeoError {
                kind: NeoErrorKind::ParserError {
                    source_path: Some(fs_path.clone()),
                    line: details.line,
                    column: details.column,
                    source: content.to_string(),
                    remainder: context.0.to_string(),
                    message: context.1.to_string(),
                },
            }
        }
        GenericErrorTree::Alt(items) => get_error_v42(content, &items[0], fs_path),
    }
}
