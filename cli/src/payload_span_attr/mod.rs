use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSpanAttr {
    pub key: String,
    pub value: String,
}
