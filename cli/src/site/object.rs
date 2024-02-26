use crate::site::Site;
use minijinja::value::Object;
use minijinja::{Error, Value};

impl Object for Site {
    fn call_method(
        &self,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "page_ids" => Ok(Value::from_serializable(&self.page_ids())),
            "page_href" => Ok(Value::from_serializable(&self.page_href(args))),
            _ => Ok(Value::from("")),
        }
    }
}
