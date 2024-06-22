// DEPRECATED: Using direct keys instead
// use crate::page_v39::PageV39;
// use minijinja::value::{Object, Value};
// use minijinja::Error;
// use std::fmt::Display;
// use std::sync::Arc;

// impl Object for PageV39 {
//     fn call_method(
//         self: &Arc<PageV39>,
//         _state: &minijinja::State,
//         name: &str,
//         _args: &[Value],
//     ) -> Result<Value, Error> {
//         match name {
//             "all_sections" => self.all_sections(),
//             "status" => Ok(Value::from(self.status())),
//             "type" => Ok(Value::from(self.r#type())),
//             _ => Ok(Value::from("[ERROR: Called non-existant function]")),
//         }
//     }
// }

// impl Display for PageV39 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "page")
//     }
// }
