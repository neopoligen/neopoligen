use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlainTextSpan {
    pub kind: String,
    pub text: String,
}
