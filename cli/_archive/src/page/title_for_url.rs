use crate::child::Child;
use crate::page::Page;
use crate::section_category::SectionCategory;
use crate::span::Span;
use tracing::instrument;

impl Page {
    #[instrument]
    pub fn title_for_url(&self) -> Option<String> {
        self.ast.iter().find_map(|child| match child {
            Child::Section(sec) => {
                if sec.r#type == String::from("title") {
                    match &sec.category {
                        SectionCategory::StandardSectionFull { containers } => {
                            if containers.len() > 0 {
                                match &containers[0] {
                                    Child::Block(thing) => {
                                        if thing.len() > 0 {
                                            Some(
                                                thing
                                                    .iter()
                                                    .map(|el| match el {
                                                        Span::Word { text, .. } => {
                                                            text.to_lowercase().to_string()
                                                        }
                                                        Span::Space { .. } => "-".to_string(),
                                                        _ => "".to_string(),
                                                    })
                                                    .collect::<Vec<String>>()
                                                    .join(""),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn title_for_url_basic() {
        let p = Page::test_with_title_metadata_and_one_p();
        let left = Some("tango-foxtrot".to_string());
        let right = p.title_for_url();
        assert_eq!(left, right);
    }
}
