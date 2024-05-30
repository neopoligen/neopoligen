use crate::page_v2::PageV2;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for PageV2 {
    fn call_method(
        self: &Arc<PageV2>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "id" => self.id_dev(),
            _ => Ok(Value::from("")),
        }
    }
}

impl Display for PageV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "page")
    }
}
