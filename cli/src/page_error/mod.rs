use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PageError {
    ParerError {
        line: usize,
        column: usize,
        remainder: String,
        full_source: String,
        message: String,
    },
}
