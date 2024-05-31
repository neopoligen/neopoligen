use crate::site_v39::SiteV39;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for SiteV39 {
    fn call_method(
        self: &Arc<SiteV39>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

impl Display for SiteV39 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "site")
    }
}
