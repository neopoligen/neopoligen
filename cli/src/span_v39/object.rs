use crate::span_v39::SpanV39;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for SpanV39 {
    fn call_method(
        self: &Arc<SpanV39>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "classes" => Ok(Value::from(self.classes(args))),
            "parsed_text" => Ok(Value::from(self.parsed_text())),
            "template_list" => Ok(Value::from_serialize(self.template_list())),
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

impl Display for SpanV39 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "site")
    }
}
