use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateError {
    pub id: String,
    pub expected: String,
    pub got: String,
}