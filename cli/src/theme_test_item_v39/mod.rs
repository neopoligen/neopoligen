pub mod object;

use regex::Regex;
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
            Some(format_html_for_theme_test_display(&self.parts()[1]).clone())
        } else {
            None
        }
    }

    pub fn got(&self) -> Option<String> {
        if self.parts().len() > 1 {
            Some(format_html_for_theme_test_display(&self.parts()[0]).clone())
        } else {
            None
        }
    }

    pub fn status(&self) -> Option<String> {
        if self.parts().len() == 3 {
            let left = self.parts()[1].replace("\n", "").replace(" ", "");
            let right = self.parts()[1].replace("\n", "").replace(" ", "");
            if left == right {
                Some("passed".to_string())
            } else {
                Some("failed".to_string())
            }
        } else {
            Some("failed".to_string())
        }
    }
}

fn format_html_for_theme_test_display(code: &str) -> String {
    let mut re = Regex::new(r"\n").unwrap();
    let output = re.replace_all(code, " ");
    re = Regex::new(r" \s+").unwrap();
    let output = re.replace_all(&output, " ");
    re = Regex::new(r"\s+<").unwrap();
    let output = re.replace_all(&output, "<");
    re = Regex::new(r">\s+").unwrap();
    let output = re.replace_all(&output, ">");
    let parts: Vec<&str> = output.split("<").collect();
    let mut assembler: Vec<String> = vec![];
    let mut level = 0i8;
    assembler.push(parts[0].to_string());
    parts.iter().skip(1).for_each(|part| {
        if part.starts_with("/") {
            level -= 2;
        }
        for _ in 0..level {
            assembler.push(" ".to_string());
        }
        assembler.push(format!("<{}\n", part));
        if !part.starts_with("/") {
            level += 2;
        }
    });
    assembler.join("").to_string()
}
