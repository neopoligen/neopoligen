pub mod object;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ThemeTestItemV39 {
    pub content: Option<String>,
}

impl ThemeTestItemV39 {
    pub fn parts(&self) -> Vec<String> {
        self.content
            .clone()
            .unwrap()
            .split("<!-- EXPECTED_OUTPUT -->")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn expected(&self) -> Option<String> {
        if self.parts().len() > 1 {
            Some(self.parts()[0].clone())
        } else {
            Some("NO EXPECTED VALUE WAS FOUND".to_string())
        }
    }

    pub fn got(&self) -> Option<String> {
        if self.parts().len() > 1 {
            Some(self.parts()[1].clone())
        } else {
            Some("NO GOT VALUE WAS FOUND".to_string())
        }
    }

    pub fn status(&self) -> Option<String> {
        Some("TODO: status here".to_string())
    }
}
