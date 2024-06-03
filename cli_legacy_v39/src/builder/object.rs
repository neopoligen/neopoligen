use crate::builder::Builder;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for Builder {
    fn call_method(
        self: &Arc<Builder>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "page_errors" => Ok(Value::from_serialize(self.page_errors())),
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

impl Builder {
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

impl Display for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "builder")
    }
}
