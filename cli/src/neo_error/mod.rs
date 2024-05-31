use serde::Serialize;
use std::fmt;
// use std::path::PathBuf;

// Currently Deprecated as of site_v2, but something to
// look back into for better error handling

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NeoError {
    //pub source_path: Option<PathBuf>,
    pub kind: NeoErrorKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum NeoErrorKind {
    MissingIdError {
        source: String,
    },
    NoAst {},
    ParserError {
        line: usize,
        column: usize,
        remainder: String,
        source: String,
        message: String,
    },
    RenderTemplate {
        message: String,
    },
}

impl fmt::Display for NeoError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            NeoErrorKind::NoAst {} => {
                fmt.write_str("No ast")?;
            }
            NeoErrorKind::ParserError {
                line,
                column,
                remainder,
                message,
                ..
            } => {
                fmt.write_str("Error: ")?;
                fmt.write_str(message.as_str())?;
                fmt.write_str("\n")?;
                fmt.write_str("Line: ")?;
                fmt.write_str(line.to_string().as_str())?;
                fmt.write_str(" Column: ")?;
                fmt.write_str(column.to_string().as_str())?;
                fmt.write_str(" At: ")?;
                fmt.write_str("\n")?;
                fmt.write_str("\n")?;
                fmt.write_str(remainder.as_str())?;
            }
            NeoErrorKind::MissingIdError { source } => {
                fmt.write_str("Missing ID\n\n")?;
                fmt.write_str(source.as_str())?;
            }
            NeoErrorKind::RenderTemplate { message } => {
                fmt.write_str("Problem Rending Template\n\n")?;
                fmt.write_str(message.as_str())?;
            }
        }
        Ok(())
    }
}
