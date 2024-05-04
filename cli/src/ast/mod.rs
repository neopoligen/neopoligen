use crate::section::*;
use crate::site_sections::SiteSections;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::final_parser::final_parser;
use nom_supreme::final_parser::Location;
use nom_supreme::final_parser::RecreateContext;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;
use std::fmt;

// pub fn ast<'a>(
//     source: &'a str,
//     sections: &'a SiteSections,
// ) -> IResult<&'a str, Vec<Section>, ErrorTree<&'a str>> {
// }

#[derive(Debug)]
pub struct ParserError {
    line: usize,
    column: usize,
    remainder: String,
    source: String,
    message: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("Error: ")?;
        fmt.write_str(&self.message)?;
        fmt.write_str("\n")?;
        fmt.write_str("Line: ")?;
        fmt.write_str(&self.line.to_string())?;
        fmt.write_str(" Column: ")?;
        fmt.write_str(&self.column.to_string())?;
        fmt.write_str(" At: ")?;
        fmt.write_str("\n")?;
        fmt.write_str("\n")?;
        fmt.write_str(&self.remainder)?;

        Ok(())
        //"Error: {} - line {} - column: {}\nAT: {}\nFROM: {}",
        //self.message, self.line, self.column, self.remainder, self.source,
    }
}

pub fn ast<'a>(
    source: &'a str,
    sections: &'a BTreeMap<String, Vec<String>>,
) -> Result<Vec<Section>, ParserError> {
    match final_parser(|src| do_parse(src, &sections))(source) {
        Ok(data) => Ok(data),
        Err(e) => Err(get_error(source, &e)),
    }
}

fn do_parse<'a>(
    source: &'a str,
    sections: &'a BTreeMap<String, Vec<String>>,
) -> IResult<&'a str, Vec<Section>, ErrorTree<&'a str>> {
    let (source, result) = many1(|src| section(src, sections))
        .context("page")
        .parse(source)?;
    Ok((source, result))
}

fn get_error(content: &str, tree: &ErrorTree<&str>) -> ParserError {
    match tree {
        GenericErrorTree::Base { location, kind } => {
            let details = Location::recreate_context(content, location);
            ParserError {
                line: details.line,
                column: details.column,
                source: content.to_string(),
                remainder: location.to_string(),
                message: kind.to_string(),
            }
        }
        GenericErrorTree::Stack { contexts, .. } => {
            let context = contexts[0];
            let details = Location::recreate_context(content, context.0);
            ParserError {
                line: details.line,
                column: details.column,
                source: content.to_string(),
                remainder: context.0.to_string(),
                message: context.1.to_string(),
            }
        }
        GenericErrorTree::Alt(items) => get_error(content, &items[0]),
    }
}
