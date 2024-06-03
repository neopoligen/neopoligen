use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PagePayload {
    pub id: String,
}

impl PagePayload {
    pub fn new_from_id(id: &str) -> PagePayload {
        PagePayload { id: id.to_string() }
    }
}
