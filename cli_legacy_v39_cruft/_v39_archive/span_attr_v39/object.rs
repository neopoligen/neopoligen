use crate::span_attr_v39::SpanAttrV39;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for SpanAttrV39 {
    fn call_method(
        self: &Arc<SpanAttrV39>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "key" => Ok(Value::from(self.key())),
            "value" => Ok(Value::from(self.value())),
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

impl Display for SpanAttrV39 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "site")
    }
}
