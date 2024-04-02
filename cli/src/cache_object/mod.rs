use minijinja::Value;
use serde::Serialize;
use crate::collection::Collection;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CacheObject {
    Value(Value),
    Collection(Collection),
    OptionString(Option<String>),
}

