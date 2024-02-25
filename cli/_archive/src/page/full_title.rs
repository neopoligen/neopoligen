use crate::child::Child;
use crate::page::Page;
use crate::section::Section;
use crate::section_category::SectionCategory;
use crate::span::Span;
use std::collections::BTreeMap;
use tracing::instrument;

impl Page {
    #[instrument]
    pub fn full_title(&self) -> Option<String> {
        if let Some(title) = get_metadata_title(self.metadata()) {
            Some(title)
        } else if let Some(title) = get_title_section_title(&self.ast) {
            Some(title)
        } else if let Some(title) = get_title_from_any_section(&self.ast) {
            Some(title)
        } else if let Some(title) = get_title_from_first_few_words(&self.ast) {
            Some(title)
        } else if let Some(title) = get_metadata_id(self.metadata()) {
            Some(title)
        } else {
            None
        }
    }
}

fn filter_section(sec: &Section) -> Option<String> {
    // with thanks to togglebit: https://paste.modprog.de/cuE6.rs
    let SectionCategory::StandardSectionFull { containers } = &sec.category else {
        return None;
    };
    let first = containers.first()?;
    let Child::Block(thing) = first else {
        return None;
    };
    let spans = thing
        .iter()
        .flat_map(|span| get_span_words(&span))
        .collect::<String>();
    Some(spans)
}

fn get_metadata_id(metadata: BTreeMap<String, String>) -> Option<String> {
    match metadata.get("id") {
        Some(metadata_id) => Some(format!("Page {}", metadata_id.to_string())),
        None => None,
    }
}

fn get_metadata_title(metadata: BTreeMap<String, String>) -> Option<String> {
    match metadata.get("title") {
        Some(metadata_title) => Some(metadata_title.to_string()),
        None => None,
    }
}

fn get_span_words(span: &Span) -> Vec<String> {
    match span {
        Span::Word { text, .. } => {
            vec![text.to_string()]
        }
        Span::Space { .. } => vec![" ".to_string()],
        Span::StandardSpan { spans, .. } => spans
            .iter()
            .map(|span| get_span_words(&span))
            .collect::<Vec<Vec<String>>>()
            .concat(),
        _ => vec!["".to_string()],
    }
}

fn get_title_from_any_section(ast: &Vec<Child>) -> Option<String> {
    ast.iter().find_map(|child| match child {
        Child::Section(sec) => match sec.key_value_attributes.get("title") {
            Some(title) => Some(title.to_string()),
            None => None,
        },
        _ => None,
    })
}

fn get_title_from_first_few_words(ast: &Vec<Child>) -> Option<String> {
    ast.iter().find_map(|child| match child {
        Child::Section(sec) => {
            let SectionCategory::StandardSectionFull { containers } = &sec.category else {
                return None;
            };
            let first = containers.first()?;
            let Child::Block(thing) = first else {
                return None;
            };
            let spans = thing
                .iter()
                .flat_map(|span| get_span_words(&span))
                .take(11)
                .collect::<String>();
            Some(spans)
        }
        _ => None,
    })
}

fn get_title_section_title(ast: &Vec<Child>) -> Option<String> {
    ast.iter().find_map(|child| match child {
        Child::Section(sec) => {
            if sec.r#type == String::from("title") {
                filter_section(sec)
            } else {
                return None;
            }
        }
        _ => None,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    // MOVED TO SITE_WITH_CACHE
    // #[test]
    // fn full_title_test() {
    //     let p = Page::test_with_title_metadata_and_one_p();
    //     let left = Some("Tango Foxtrot".to_string());
    //     let right = p.full_title();
    //     assert_eq!(left, right);
    // }

    // MOVED TO SITE_WITH_CACHE
    // #[test]
    // fn full_title_test_from_inline_span() {
    //     let p = Page::title_with_inline_span();
    //     let left = Some("Span In Title".to_string());
    //     let right = p.full_title();
    //     assert_eq!(left, right);
    // }

    // MOVED TO SITE_WITH_CACHE
    // #[test]
    // fn full_title_test_from_nested_inilne_spans() {
    //     let p = Page::title_with_nested_inline_spans();
    //     let left = Some("Span Nested In Title".to_string());
    //     let right = p.full_title();
    //     assert_eq!(left, right);
    // }

    // MOVED TO SITE_WITH_CACHE
    // #[test]
    // fn full_title_test_title_in_metadata() {
    //     let p = Page::title_in_metadata();
    //     let left = Some("This should be the title. It's from the metadata".to_string());
    //     let right = p.full_title();
    //     assert_eq!(left, right);
    // }

    #[test]
    fn full_title_test_title_in_bookmark() {
        let p = Page::title_in_bookmark();
        let left = Some("This is the title from a bookmark".to_string());
        let right = p.full_title();
        assert_eq!(left, right);
    }

    #[test]
    fn full_title_test_from_first_few_words() {
        let p = Page::title_via_standard_section();
        let left = Some("Title from first few words of".to_string());
        let right = p.full_title();
        assert_eq!(left, right);
    }

    // MOVED TO SITE_WITH_CACHE
    // #[test]
    // fn full_title_test_with_only_id() {
    //     let p = Page::no_title_or_standard_sections();
    //     let left = Some("Page id996622".to_string());
    //     let right = p.full_title();
    //     assert_eq!(left, right);
    // }
}
