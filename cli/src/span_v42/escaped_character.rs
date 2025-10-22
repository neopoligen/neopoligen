use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EscapedCharacterSpanV42 {
    pub kind: String,
    pub text: String,
}
