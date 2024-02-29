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
            "page_href" => Ok(Value::from_serializable(&self.page_href(args))),
            "page_ids" => Ok(Value::from_serializable(&self.page_ids())),
            "page_main_body" => Ok(Value::from_serializable(&self.page_main_body(args))),
            "page_output_path" => Ok(Value::from_serializable(&self.page_output_path(args))),
            "page_place_section" => Ok(Value::from_serializable(&self.page_place_section(args))),
            "page_template" => Ok(Value::from_serializable(&self.page_template(args))),
            _ => Ok(Value::from("")),
        }
    }
}
