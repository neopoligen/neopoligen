use crate::block::Block;
use crate::section_attrs::SectionAttrs;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Section {
    Checklist {
        attrs: SectionAttrs,
        items: Vec<Section>,
    },
    List {
        attrs: SectionAttrs,
        items: Vec<Section>,
    },
    Raw {
        attrs: SectionAttrs,
        text: String,
    },
    Standard {
        attrs: SectionAttrs,
        content: Vec<Block>,
    },
    Unknown,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionBounds {
    End,
    Full,
    Start,
}
