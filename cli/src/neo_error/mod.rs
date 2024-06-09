use crate::payload_section::PayloadSection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NeoError {
    pub kind: NeoErrorKind,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoErrorKind {
    GenericErrorWithoutSourcePath {
        msg: String,
    },
    GenericErrorWithSourcePath {
        source_path: PathBuf,
        msg: String,
    },
    FileError {
        source_path: PathBuf,
        msg: String,
    },
    ForwardError {
        msg: String,
    },
    ForwardErrorWithSourcePath {
        source_path: PathBuf,
        msg: String,
    },
    MissingThemeDirectory {
        path: PathBuf,
    },
    ParserError {
        source_path: Option<PathBuf>,
        line: usize,
        column: usize,
        remainder: String,
        source: String,
        message: String,
    },
    ThemeTestError {
        expected: String,
        got: String,
        sections: Vec<PayloadSection>,
        source_path: PathBuf,
    },
}

impl std::fmt::Display for NeoError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            NeoErrorKind::MissingThemeDirectory { path } => {
                fmt.write_str("Missing theme directory: ")?;
                fmt.write_str(path.display().to_string().as_str())?;
            }
            NeoErrorKind::GenericErrorWithoutSourcePath { msg } => {
                fmt.write_str(msg.as_str())?;
            }
            NeoErrorKind::GenericErrorWithSourcePath { source_path, msg } => {
                fmt.write_str("FileError: ")?;
                fmt.write_str(source_path.display().to_string().as_str())?;
                fmt.write_str("\n")?;
                fmt.write_str(msg.as_str())?;
            }
            NeoErrorKind::FileError { source_path, msg } => {
                fmt.write_str("FileError: ")?;
                fmt.write_str(source_path.display().to_string().as_str())?;
                fmt.write_str("\n")?;
                fmt.write_str(msg.as_str())?;
            }
            NeoErrorKind::ForwardError { msg } => {
                fmt.write_str(msg.as_str())?;
            }
            NeoErrorKind::ForwardErrorWithSourcePath { source_path, msg } => {
                fmt.write_str("Path: ")?;
                fmt.write_str(source_path.display().to_string().as_str())?;
                fmt.write_str("\n")?;
                fmt.write_str(msg.as_str())?;
            }
            NeoErrorKind::ThemeTestError { source_path, .. } => {
                fmt.write_str("Path: ")?;
                fmt.write_str(source_path.display().to_string().as_str())?;
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
        }
        Ok(())
    }
}

impl std::error::Error for NeoError {
    fn description(&self) -> &str {
        "todo: figure out how to print errors here"
    }
}
