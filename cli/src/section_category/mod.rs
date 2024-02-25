pub mod checklist;
pub mod comment;
pub mod detail;
pub mod json;
pub mod json_plugin;
pub mod list;
pub mod raw;
pub mod standard;
pub mod table;
pub mod text_plugin;
pub mod yaml;

use crate::child::*;
use minijinja::Value;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(content = "content", rename_all = "lowercase", tag = "type")]
pub enum SectionCategory {
    ChecklistSectionStart { containers: Vec<Child> }, // [Done]
    ChecklistSectionFull { containers: Vec<Child> },  // [Done]
    ChecklistSectionEnd { containers: Vec<Child> },   // [Done]

    DetailSectionStart { containers: Vec<Child> }, // []
    DetailSectionFull { containers: Vec<Child> },  // []
    DetailSectionEnd { containers: Vec<Child> },   // []

    CommentSectionStart { containers: Vec<Child> }, // [Done]
    CommentSectionFull { containers: Vec<Child> },  // [Done]
    CommentSectionEnd { containers: Vec<Child> },   // [Done]

    JsonSectionStart { object: Option<Value> }, // [Done]
    JsonSectionFull { object: Option<Value> },  // [Done]
    JsonSectionEnd { containers: Vec<Child> },  // [Done]

    JsonPluginSectionStart { object: Option<Value> }, // []
    JsonPluginSectionFull { object: Option<Value> },  // []
    JsonPluginSectionEnd { containers: Vec<Child> },  // []

    ListSectionStart { containers: Vec<Child> }, // [Done]
    ListSectionFull { containers: Vec<Child> },  // [Done]
    ListSectionEnd { containers: Vec<Child> },   // [Done]

    PreformattedSectionStart { text: Option<String> }, // [Done]
    PreformattedSectionFull { text: Option<String> },  // [Done]
    PreformattedSectionEnd { containers: Vec<Child> }, // [Done]

    StandardSectionStart { containers: Vec<Child> }, // [Done]
    StandardSectionFull { containers: Vec<Child> },  // [Done]
    StandardSectionEnd { containers: Vec<Child> },   // [Done]

    TableSectionStart { object: Value },        // []
    TableSectionFull { object: Value },         // []
    TableSectionEnd { containers: Vec<Child> }, // []

    TextPluginSectionStart { text: Option<String> }, // []
    TextPluginSectionFull { text: Option<String> },  // []
    TextPluginSectionEnd { containers: Vec<Child> }, // []

    YamlSectionStart { object: Option<Value> }, // [Done]
    YamlSectionFull { object: Option<Value> },  // [Done]
    YamlSectionEnd { containers: Vec<Child> },  // [Done]
}
