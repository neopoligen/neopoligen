use crate::page_v2::PageV2;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for PageV2 {
    fn call_method(
        self: &Arc<PageV2>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "all_sections" => self.all_sections(),
            "all_sections_except" => self.all_sections_except(args),
            "only_sections" => self.only_sections(args),
            "id" => self.id_v2(),
            "type" => self.type_v2(),
            _ => Ok(Value::from("[ERROR: Called non-existant function]")),
        }
    }
}

impl Display for PageV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "page")
    }
}
