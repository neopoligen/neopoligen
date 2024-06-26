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
            "page_ids" => Ok(Value::from_serialize(&self.page_ids())),
            "page_href" => Ok(Value::from_serialize(&self.page_href(args))),
            "page_output_path" => Ok(Value::from_serialize(&self.page_output_path(args))),
            "page_template" => Ok(Value::from_serialize(&self.page_template(args))),
            _ => Ok(Value::from("")),
        }
    }
}
