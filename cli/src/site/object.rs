use crate::site::Site;
use minijinja::value::Object;
use minijinja::{Error, Value};
use tracing::instrument;

impl Object for Site {
    #[instrument]
    fn call_method(
        &self,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            _ => Ok(Value::from("")),
        }
    }
}
