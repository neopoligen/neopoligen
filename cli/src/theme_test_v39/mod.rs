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
        let tmp_items = vec![ThemeTestItemV39 { content: None }];
        Ok(Value::make_object_iterable(tmp_items.clone(), |item| {
            Box::new(item.iter().cloned().map(Value::from_object))
        }))
    }
}
