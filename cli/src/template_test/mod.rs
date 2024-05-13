use crate::page::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct TemplateTest {
    pub page: Page,
    pub rendered: String,
    pub errors: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TemplateTestStatus {
    Passed,
    Failed,
}

impl TemplateTest {
    pub fn new(page: Page, rendered: String) -> TemplateTest {
        let mut errors = vec![];
        let tests: Vec<&str> = rendered.split("<!-- START_TEMPLATE_TEST -->").collect();
        tests.iter().skip(1).for_each(|t| {
            let parts: Vec<&str> = t.split("<!-- EXPECTED_OUTPUT -->").collect();
            let left = parts[0].replace("\n", "").replace(" ", "");
            let right = parts[1].replace("\n", "").replace(" ", "");
            if left != right {
                errors.push((parts[0].to_string(), parts[1].to_string()));
            }
        });
        TemplateTest {
            page,
            rendered,
            errors,
        }
    }

    // pub fn status(&self) -> TemplateTestStatus {
    //     if self.expected == self.got {
    //         TemplateTestStatus::Passed
    //     } else {
    //         TemplateTestStatus::Failed
    //     }
    // }
}
