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
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "all_site_links" => Ok(Value::from_serializable(&self.all_site_links())),
            "filter_page_links_alpha" => Ok(self.filter_page_links_alpha(args)),
            "link_or_title" => Ok(self.link_or_title(args)),
            "link_or_title_filtered" => Ok(self.link_or_title_filtered(args)),
            "full_title" => Ok(Value::from_serializable(&self.full_title(args))),
            "title_for_url" => Ok(Value::from_serializable(&self.title_for_url(args))),
            "tlink" => Ok(Value::from_serializable(&self.tlink(args))),
            "url_for_page" => Ok(Value::from_serializable(&self.url_for_page(args))),
            _ => Ok(Value::from("")),
        }
    }
}
