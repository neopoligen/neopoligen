use crate::nav_item::NavItem;
use crate::nav_items::NavItems;
use minijinja::Value;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CacheObject {
    NavItems(NavItems),
    // DEPRECATED: Remove Menu when NavItems is
    // in place (no later than May 2024 if this
    // hasn't been used since then
    Menu(Vec<NavItem>),
    Value(Value),
    OptionString(Option<String>),
}
