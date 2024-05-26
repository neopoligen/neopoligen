use crate::site_v2::SiteV2;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for SiteV2 {
    fn call_method(
        self: &Arc<SiteV2>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "base_url" => self.base_url(),
            "page_ast" => self.page_ast(args),
            "page_permalink" => self.page_permalink(args),
            "load_config" => Ok(Value::from_serialize(&self.config)),
            _ => Ok(Value::from("")),
        }
    }

    // fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
    //     match key.as_str().unwrap() {
    //         "pages" => Some(Value::from_serialize(&self.pages)),
    //         _ => None,
    //     }
    // }
}

impl Display for SiteV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "site_object")
    }
}
