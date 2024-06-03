pub mod object;

use crate::{page_v39::PageV39, theme_test_item_v39::ThemeTestItemV39};
use minijinja::Value;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ThemeTestV39 {
    pub page: PageV39,
}

impl ThemeTestV39 {
    pub fn output_name(&self) -> Option<String> {
        if let Some(id) = self.page.id() {
            Some(id)
        } else {
            Some("todo-get-name-for-this-file".to_string())
        }
    }

    pub fn items(&self) -> Result<Value, minijinja::Error> {
        let mut tmp_items: Vec<ThemeTestItemV39> = vec![];
        if let Some(content) = &self.page.output_content {
            let tests = content
                .split("<!-- START_THEME_TEST -->")
                .collect::<Vec<&str>>();
            tests.iter().skip(1).for_each(|t| {
                tmp_items.push(ThemeTestItemV39 {
                    content: Some(t.to_string()),
                });
            })
        }
        Ok(Value::make_object_iterable(tmp_items.clone(), |item| {
            Box::new(item.iter().cloned().map(Value::from_object))
        }))
    }
}
