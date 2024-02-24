use crate::page::Page;
use minijinja::value::Object;
use minijinja::{Error, Value};
use tracing::instrument;

impl Object for Page {
    #[instrument]
    fn call_method(
        &self,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            // "has_filter" => Ok(self.has_filter(args)),
            // "link_or_title" => Ok(self.link_or_title(args)),
            // "link_or_title_filtered" => Ok(self.link_or_title_filtered(args)),
            // "filter_page_links_alpha" => Ok(self.filter_page_links_alpha(args)),
            // "filters" => Ok(Value::from_serializable(&self.filters())),
            // "folders" => Ok(Value::from_serializable(&self.folders())),
            // "full_title_for_page" => Ok(Value::from_serializable(&self.full_title_for_page(args))),
            // "id" => Ok(Value::from_serializable(&self.id().unwrap())),
            // "place_main_body" => Ok(Value::from_serializable(&self.place_main_body())),
            // "metadata" => Ok(Value::from_serializable(&self.metadata())),
            // "place_everything" => Ok(Value::from_serializable(&self.place_everything())),
            // "place_everything_except" => Ok(Value::from_serializable(
            //     &self.place_everything_except(args),
            // )),
            // "source" => Ok(Value::from_serializable(&self.source())),
            // "type" => Ok(Value::from_serializable(&self.r#type())),
            // "url_for_page" => Ok(Value::from_serializable(&self.url_for_page(args))),
            _ => Ok(Value::from("")),
        }
    }
}
