use crate::site_v2::SiteV2;
use minijinja::value::Object;
use minijinja::{Error, Value};

impl Object for SiteV2 {
    fn call_method(
        &self,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "domain" => Ok(Value::from_serializable(&self.domain())),
            "domain_no_https" => Ok(Value::from_serializable(&self.domain_no_https())),
            "filtered_pages_alpha" => {
                Ok(Value::from_serializable(&self.filtered_pages_alpha(args)))
            }
            "folder_for_page" => Ok(Value::from_serializable(&self.folder_for_page(args))),
            "folder_menu" => Ok(Value::from_serializable(&self.folder_menu(args))),
            "href_for_page" => Ok(Value::from_serializable(&self.href_for_page(args))),
            "image_path_for" => Ok(Value::from_serializable(&self.image_path_for(args))),
            "include_section_from_page" => Ok(Value::from_serializable(
                &self.include_section_from_page(args),
            )),
            "log_from_template" => Ok(Value::from_serializable(&self.log_from_template(args))),
            "link_or_title" => Ok(Value::from_serializable(&self.link_or_title(args))),
            "main_body_for_page" => Ok(Value::from_serializable(&self.main_body_for_page(args))),
            "output_path_for_page" => {
                Ok(Value::from_serializable(&self.output_path_for_page(args)))
            }
            "page_ids" => Ok(Value::from_serializable(&self.page_ids())),
            "page_source" => Ok(Value::from_serializable(&self.page_source(args))),
            "page_ast" => Ok(Value::from_serializable(&self.page_ast(args))),
            "place_section" => Ok(Value::from_serializable(&self.place_section(args))),
            "template_for_page" => Ok(Value::from_serializable(&self.template_for_page(args))),
            "title_for_page" => Ok(Value::from_serializable(&self.title_for_page(args))),
            _ => Ok(Value::from("")),
        }
    }
}
