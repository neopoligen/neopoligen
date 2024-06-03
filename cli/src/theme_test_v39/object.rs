use crate::theme_test_v39::ThemeTestV39;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for ThemeTestV39 {
    fn call_method(
        self: &Arc<ThemeTestV39>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "items" => self.items(),
            "ping" => Ok(Value::from("PING")),
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

impl Display for ThemeTestV39 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "site")
    }
}
