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
            "collection_from_files_and_folders" => Ok(Value::from_serializable(
                &self.collection_from_files_and_folders(args),
            )),
            "collection_from_tags" => {
                Ok(Value::from_serializable(&self.collection_from_tags(args)))
            }
            "does_template_exist" => Ok(Value::from_serializable(&self.does_template_exist(args))),
            "get_subtree" => Ok(Value::from_serializable(&self.get_subtree(args))),
            "ilink" => Ok(Value::from_serializable(&self.ilink(args))),
            "image" => Ok(Value::from_serializable(&self.image(args))),
            "log" => Ok(Value::from_serializable(&self.log_from_template(args))),
            "link_or_title" => Ok(Value::from_serializable(&self.link_or_title(args))),
            "page_head" => Ok(Value::from_serializable(&self.page_head(args))),
            "page_href" => Ok(Value::from_serializable(&self.page_href(args))),
            "page_html_link" => Ok(Value::from_serializable(&self.page_html_link(args))),
            "page_ids" => Ok(Value::from_serializable(&self.page_ids())),
            "page_main_body" => Ok(Value::from_serializable(&self.page_main_body(args))),
            "page_menu_title" => Ok(Value::from_serializable(&self.page_menu_title(args))),
            "page_output_path" => Ok(Value::from_serializable(&self.page_output_path(args))),
            "page_place_section" => Ok(Value::from_serializable(&self.page_place_section(args))),
            "page_scripts" => Ok(Value::from_serializable(&self.page_scripts(args))),
            "page_source" => Ok(Value::from_serializable(&self.page_source(args))),
            "page_source_path" => Ok(Value::from_serializable(&self.page_source_path(args))),
            "page_status" => Ok(Value::from_serializable(&self.page_status(args))),
            "page_stylesheets" => Ok(Value::from_serializable(&self.page_stylesheets(args))),
            "page_template" => Ok(Value::from_serializable(&self.page_template(args))),
            "page_type" => Ok(Value::from_serializable(&self.page_type(args))),
            "page_title" => Ok(Value::from_serializable(&self.page_title(args))),
            "show" => Ok(Value::from_serializable(&self.show(args))),
            _ => Ok(Value::from("")),
        }
    }
}
