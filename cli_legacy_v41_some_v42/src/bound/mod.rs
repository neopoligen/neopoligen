use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Bound {
    End,
    Full,
    Start,
}
