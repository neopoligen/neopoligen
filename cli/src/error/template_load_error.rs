use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TemplateLoadError {
    pub message: String,
    pub details: String,
}
