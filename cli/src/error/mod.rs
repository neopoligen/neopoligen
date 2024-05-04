use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ErrorKind {
    ParserError {
        line: usize,
        column: usize,
        remainder: String,
        source: String,
        message: String,
    },
    MissingIdError {},
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("ERROR")?;
        Ok(())
    }

    // fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    //     fmt.write_str("Error: ")?;
    //     fmt.write_str(&self.message)?;
    //     fmt.write_str("\n")?;
    //     fmt.write_str("Line: ")?;
    //     fmt.write_str(&self.line.to_string())?;
    //     fmt.write_str(" Column: ")?;
    //     fmt.write_str(&self.column.to_string())?;
    //     fmt.write_str(" At: ")?;
    //     fmt.write_str("\n")?;
    //     fmt.write_str("\n")?;
    //     fmt.write_str(&self.remainder)?;
    //     Ok(())
    // }
}
