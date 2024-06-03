use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NeoError {
    kind: NeoErrorKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoErrorKind {}
