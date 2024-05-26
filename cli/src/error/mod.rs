use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ErrorKind {
    MissingIdError {
        source: String,
    },
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

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::ParserError {
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
            ErrorKind::MissingIdError { source } => {
                fmt.write_str("Missing ID\n\n")?;
                fmt.write_str(source.as_str())?;
            }
            ErrorKind::RenderTemplate { message } => {
                fmt.write_str("Problem Rending Template\n\n")?;
                fmt.write_str(message.as_str())?;
            }
        }
        Ok(())
    }
}
