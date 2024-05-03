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
            "collection_from_files_and_folders" => Ok(Value::from_serialize(
                &self.collection_from_files_and_folders(args),
            )),
            "collection_from_tags" => {
                Ok(Value::from_serialize(&self.collection_from_tags(args)))
            }
            "does_template_exist" => Ok(Value::from_serialize(&self.does_template_exist(args))),
            "error" => Ok(Value::from_serialize(&self.error_from_template(args))),
            "get_subtree" => Ok(Value::from_serialize(&self.get_subtree(args))),
            "highlight_code" => Ok(Value::from_serialize(&self.highlight_code(args))),
            "ilink" => Ok(Value::from_serialize(&self.ilink(args))),
            "image" => Ok(Value::from_serialize(&self.image(args))),
            "log" => Ok(Value::from_serialize(&self.log_from_template(args))),
            "link_or_title" => Ok(Value::from_serialize(&self.link_or_title(args))),
            "mp3" => Ok(Value::from_serialize(&self.mp3(args))),
            "page_ast" => Ok(Value::from_serialize(&self.page_ast(args))),
            "page_ast_pretty" => Ok(Value::from_serialize(&self.page_ast_pretty(args))),
            "page_build_path" => Ok(Value::from_serialize(&self.page_build_path(args))),
            "page_head" => Ok(Value::from_serialize(&self.page_head(args))),
            "page_href" => Ok(Value::from_serialize(&self.page_href(args))),
            "page_html_link" => Ok(Value::from_serialize(&self.page_html_link(args))),
            "page_ids" => Ok(Value::from_serialize(&self.page_ids())),
            "page_main_body" => Ok(Value::from_serialize(&self.page_main_body(args))),
            "page_menu_title" => Ok(Value::from_serialize(&self.page_menu_title(args))),
            "page_place_section" => Ok(Value::from_serialize(&self.page_place_section(args))),
            "page_scripts" => Ok(Value::from_serialize(&self.page_scripts(args))),
            "page_source" => Ok(Value::from_serialize(&self.page_source(args))),
            "page_source_path" => Ok(Value::from_serialize(&self.page_source_path(args))),
            "page_status" => Ok(Value::from_serialize(&self.page_status(args))),
            "page_stylesheets" => Ok(Value::from_serialize(&self.page_stylesheets(args))),
            "page_template" => Ok(Value::from_serialize(&self.page_template(args))),
            "page_type" => Ok(Value::from_serialize(&self.page_type(args))),
            "page_title" => Ok(Value::from_serialize(&self.page_title(args))),
            "show" => Ok(Value::from_serialize(&self.show(args))),
            _ => Ok(Value::from("")),
        }
    }
}
