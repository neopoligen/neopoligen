use crate::neo_error::*;
use crate::section::*;
use crate::site_config::ConfigSections;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::final_parser::final_parser;
use nom_supreme::final_parser::Location;
use nom_supreme::final_parser::RecreateContext;
use nom_supreme::parser_ext::ParserExt;

pub fn parse_ast(source: &str, sections: ConfigSections) -> Result<Vec<Section>, NeoError> {
    match final_parser(|src| do_parse(src, &sections))(source) {
        Ok(data) => Ok(data),
        Err(e) => Err(get_error(source, &e).into()),
    }
}

fn do_parse<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
) -> IResult<&'a str, Vec<Section>, ErrorTree<&'a str>> {
    let (source, result) = many1(|src| start_or_full_section_v39(src, &sections))
        .context("page")
        .parse(source)?;
    Ok((source, result))
}

fn get_error(content: &str, tree: &ErrorTree<&str>) -> NeoError {
    match tree {
        GenericErrorTree::Base { location, kind } => {
            let details = Location::recreate_context(content, location);

            NeoError {
                kind: NeoErrorKind::ParserError {
                    line: details.line,
                    column: details.column,
                    source: content.to_string(),
                    remainder: location.to_string(),
                    message: kind.to_string(),
                },
            }

            //
        }
        GenericErrorTree::Stack { contexts, .. } => {
            let context = contexts[0];
            let details = Location::recreate_context(content, context.0);

            NeoError {
                kind: NeoErrorKind::ParserError {
                    line: details.line,
                    column: details.column,
                    source: content.to_string(),
                    remainder: context.0.to_string(),
                    message: context.1.to_string(),
                },
            }

            //
        }
        GenericErrorTree::Alt(items) => get_error(content, &items[0]),
    }
}
