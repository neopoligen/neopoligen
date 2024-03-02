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
            "log" => Ok(Value::from_serializable(&self.log_from_template(args))),
            "folder_menu" => Ok(Value::from_serializable(&self.folder_menu(args))),
            "link_or_title" => Ok(Value::from_serializable(&self.link_or_title(args))),
            "page_href" => Ok(Value::from_serializable(&self.page_href(args))),
            "page_ids" => Ok(Value::from_serializable(&self.page_ids())),
            "page_main_body" => Ok(Value::from_serializable(&self.page_main_body(args))),
            "page_output_path" => Ok(Value::from_serializable(&self.page_output_path(args))),
            "page_place_section" => Ok(Value::from_serializable(&self.page_place_section(args))),
            "page_source_path" => Ok(Value::from_serializable(&self.page_source_path(args))),
            "page_template" => Ok(Value::from_serializable(&self.page_template(args))),
            "page_title" => Ok(Value::from_serializable(&self.page_title(args))),
            _ => Ok(Value::from("")),
        }
    }
}
