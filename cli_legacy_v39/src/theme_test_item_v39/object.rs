use crate::theme_test_item_v39::ThemeTestItemV39;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for ThemeTestItemV39 {
    fn call_method(
        self: &Arc<ThemeTestItemV39>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "expected" => Ok(Value::from(self.expected())),
            "got" => Ok(Value::from(self.got())),
            "status" => Ok(Value::from(self.status())),
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

impl Display for ThemeTestItemV39 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "site")
    }
}
