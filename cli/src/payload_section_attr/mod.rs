use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSectionAttr {
    pub key: String,
    pub value: String,
}
