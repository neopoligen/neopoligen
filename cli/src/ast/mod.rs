use crate::section::Section;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Ast {
    secitons: Vec<Section>,
}
