use crate::builder_v39::BuilderV39;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for BuilderV39 {
    fn call_method(
        self: &Arc<BuilderV39>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "config" => Ok(Value::from_serialize(self.config())),
            "page_errors" => Ok(Value::from_serialize(self.page_errors())),
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

impl BuilderV39 {
    pub fn issues(&self, _args: &[Value]) -> Result<Value, Error> {
        Ok(Value::from_serialize(
            &self
                .issues
                .iter()
                .map(|i| Value::from_object(i.clone()))
                .collect::<Vec<Value>>(),
        ))
    }
}

impl Display for BuilderV39 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "builder")
    }
}
