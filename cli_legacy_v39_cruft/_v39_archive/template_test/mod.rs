// DEPRECATED: TODO: Remove in favor of theme_test_v39
//
use crate::page::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct TemplateTest {
    pub page: Page,
    pub render_error: Option<String>,
    pub rendered: Option<String>,
    pub template_errors: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TemplateTestStatus {
    Passed,
    Failed,
}

impl TemplateTest {
    pub fn new(page: Page, rendered: Option<String>, render_error: Option<String>) -> TemplateTest {
        let mut template_errors = vec![];
        match rendered.clone() {
            Some(text) => {
                let tests: Vec<&str> = text.split("<!-- START_TEMPLATE_TEST -->").collect();
                tests.iter().skip(1).for_each(|t| {
                    let parts: Vec<&str> = t.split("<!-- EXPECTED_OUTPUT -->").collect();
                    let left = parts[1].replace("\n", "").replace(" ", "");
                    let right = parts[0].replace("\n", "").replace(" ", "");
                    if left != right {
                        template_errors.push((parts[1].to_string(), parts[0].to_string()));
                    }
                });
                TemplateTest {
                    page,
                    render_error,
                    rendered,
                    template_errors,
                }
            }
            None => TemplateTest {
                page,
                render_error,
                rendered,
                template_errors,
            },
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
