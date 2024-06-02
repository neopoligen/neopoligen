pub mod object;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ThemeTestItemV39 {
    pub content: Option<String>,
}

impl ThemeTestItemV39 {
    pub fn expected(&self) -> Option<String> {
        Some("TODO: expected stuff here".to_string())
    }

    pub fn got(&self) -> Option<String> {
        Some("TODO: got stuff here".to_string())
    }

    pub fn status(&self) -> Option<String> {
        Some("TODO: status here".to_string())
    }
}
