use minijinja::value::{Object, Value};
use serde::Serialize;
// use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone, Debug, Error, Serialize)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum NeoErrorV39 {
    #[error("error: {detail:?}")]
    Generic { detail: String },

    #[error("minijinja error")]
    MiniJinjaError {
        source_path: Option<PathBuf>,
        details: String,
    },
    #[error("unknown data store error")]
    Unknown,
}

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

impl Display for NeoError {
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

impl Object for NeoErrorV39 {
    fn call_method(
        self: &Arc<NeoErrorV39>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, minijinja::Error> {
        match name {
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

impl Object for NeoError {
    fn call_method(
        self: &Arc<NeoError>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, minijinja::Error> {
        match name {
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

// impl Display for NeoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "builder_issue")
//     }
// }
