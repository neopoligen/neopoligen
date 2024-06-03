use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::source_page;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NeoError {
    pub kind: NeoErrorKind,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoErrorKind {
    FileError {
        source_path: PathBuf,
        msg: String,
    },
    ForwardError {
        msg: String,
    },
    ParserError {
        line: usize,
        column: usize,
        remainder: String,
        source: String,
        message: String,
    },
}

impl std::fmt::Display for NeoError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            NeoErrorKind::FileError { source_path, msg } => {
                fmt.write_str("FileError: ")?;
                fmt.write_str(source_path.display().to_string().as_str())?;
                fmt.write_str("\n")?;
                fmt.write_str(msg.as_str())?;
            }
            NeoErrorKind::ForwardError { msg } => {
                fmt.write_str(msg.as_str())?;
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
