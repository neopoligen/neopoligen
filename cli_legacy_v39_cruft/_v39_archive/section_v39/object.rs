use crate::section_v39::SectionV39;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for SectionV39 {
    fn call_method(
        self: &Arc<SectionV39>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "bounds" => Ok(Value::from(self.bounds())),
            "children" => self.children(),
            "ping" => Ok(Value::from(self.ping())),
            "spans" => self.spans(),
            "template" => Ok(Value::from(self.template())),
            "template_list" => Ok(Value::from_serialize(self.template_list())),
            "text" => Ok(Value::from(self.text())),
            "type" => Ok(Value::from(&self.r#type)),
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

impl Display for SectionV39 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "site")
    }
}
