use minijinja::Value;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CacheObject {
    Value(Value),
    OptionString(Option<String>),
}
