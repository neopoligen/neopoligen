use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{
    payload_span::PayloadSpan,
    section::{Section, SectionBounds, SectionKind},
    section_attr::SectionAttrKind,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSection {
    pub attrs: BTreeMap<String, String>,
    pub bounds: String,
    pub children: Vec<PayloadSection>,
    pub classes: Vec<String>,
    pub created: Option<String>,
    pub data: Option<Value>,
    pub flags: Vec<String>,
    pub id: Option<String>,
    pub kind: Option<String>,
    pub spans: Vec<PayloadSpan>,
    pub status: Option<String>,
    pub tags: Vec<String>,
    pub text: Option<String>,
    pub r#type: String,
    pub template_list: Vec<String>,
    pub updated: Option<String>,
}

impl PayloadSection {
    pub fn new_from_section(section: &Section) -> PayloadSection {
        // TODO: Convert to BTreeMap
        // let attrs = section
        //     .attrs
        //     .iter()
        //     .filter_map(|attr| match &attr.kind {
        //         SectionAttrKind::KeyValue { key, value } => {
        //             if key.as_str() != "tag"
        //                 && key.as_str() != "class"
        //                 && key.as_str() != "created"
        //                 && key.as_str() != "status"
        //                 && key.as_str() != "updated"
        //             {
        //                 Some(PayloadSectionAttr {
        //                     key: key.to_string(),
        //                     value: value.to_string(),
        //                 })
        //             } else {
        //                 None
        //             }
        //         }
        //         _ => None,
        //     })
        //     .collect::<Vec<PayloadSectionAttr>>();

        let bounds = match section.bounds {
            SectionBounds::End => "end".to_string(),
            SectionBounds::Full => "full".to_string(),
            SectionBounds::Start => "start".to_string(),
        };
        let classes = section
            .attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SectionAttrKind::KeyValue { key, value } => {
                    if key.as_str() == "class" {
                        Some(value.split(" ").map(|s| s.to_string()))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .flatten()
            .collect::<Vec<String>>();
        let created = section.attrs.iter().find_map(|attr| match &attr.kind {
            SectionAttrKind::KeyValue { key, value } => {
                if key.as_str() == "created" {
                    Some(value.clone())
                } else {
                    None
                }
            }
            _ => None,
        });
        let flags = section
            .attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SectionAttrKind::Flag { flag } => Some(flag.to_string()),
                _ => None,
            })
            .collect::<Vec<String>>();
        let id = section.attrs.iter().find_map(|attr| match &attr.kind {
            SectionAttrKind::KeyValue { key, value } => {
                if key.as_str() == "id" {
                    Some(value.clone())
                } else {
                    None
                }
            }
            _ => None,
        });
        let kind = Some(match &section.kind {
            SectionKind::Basic { .. } => "basic".to_string(),
            SectionKind::Block { .. } => "block".to_string(),
            SectionKind::List { .. } => "list".to_string(),
            SectionKind::ListItem { .. } => "listitem".to_string(),
            SectionKind::Raw { .. } => "raw".to_string(),
            SectionKind::Unknown { .. } => "unknown".to_string(),
            SectionKind::Yaml { .. } => "yaml".to_string(),
        });
        let status = section.attrs.iter().find_map(|attr| match &attr.kind {
            SectionAttrKind::KeyValue { key, value } => {
                if key.as_str() == "status" {
                    Some(value.clone())
                } else {
                    None
                }
            }
            _ => None,
        });
        let mut template_list = vec![];
        if let Some(template) = section.get_attr("template") {
            template_list.push(format!(
                "sections/{}/{}/{}.neoj",
                section.r#type, bounds, template
            ));
        }
        template_list.push(format!(
            "sections/{}/{}/default.neoj",
            section.r#type, bounds
        ));
        template_list.push(format!("sections/generic/{}/default.neoj", bounds));
        let children = match &section.kind {
            SectionKind::Basic { children } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child))
                .collect(),
            SectionKind::Block { .. } => vec![],
            SectionKind::List { children } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child))
                .collect(),
            SectionKind::ListItem { children } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child))
                .collect(),
            SectionKind::Raw { children, .. } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child))
                .collect(),
            SectionKind::Unknown { children } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child))
                .collect(),
            SectionKind::Yaml { .. } => vec![],
        };

        let tags = section
            .attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SectionAttrKind::KeyValue { key, value } => {
                    if key.as_str() == "tag" {
                        Some(value.to_string())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<String>>();
        let spans = match &section.kind {
            SectionKind::Block { spans } => spans
                .iter()
                .map(|span| PayloadSpan::new_from_span(&span))
                .collect::<Vec<PayloadSpan>>(),
            _ => vec![],
        };
        let text = match &section.kind {
            SectionKind::Raw { text, .. } => text.clone(),
            _ => None,
        };
        let updated = section.attrs.iter().find_map(|attr| match &attr.kind {
            SectionAttrKind::KeyValue { key, value } => {
                if key.as_str() == "updated" {
                    Some(value.clone())
                } else {
                    None
                }
            }
            _ => None,
        });
        PayloadSection {
            attrs: BTreeMap::new(),
            bounds,
            children,
            classes,
            created,
            data: None,
            flags,
            id,
            kind,
            spans,
            status,
            tags,
            text,
            r#type: section.r#type.clone(),
            template_list,
            updated,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    // #[test]
    // fn attrs_check() {
    //     let section = Section::mock2_div_with_title_and_template_attrs();
    //     let left = "Title From Attr";
    //     let right = &PayloadSection::new_from_section(&section).attrs[0].value;
    //     assert_eq!(left, right);
    // }

    #[test]
    #[ignore]
    fn todo_check_data_from_json() {
        // TODO
    }

    #[test]
    #[ignore]
    fn todo_check_data_from_yaml() {
        // TODO
    }

    #[test]
    #[ignore]
    fn todo_check_text_from_raw_section() {
        // TODO
    }

    #[test]
    fn bounds_check() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = "full";
        let right = &PayloadSection::new_from_section(&section).bounds;
        assert_eq!(left, right);
    }

    #[test]
    fn classes_work() {
        let payload_section =
            PayloadSection::new_from_section(&Section::mock4_youtube_with_tags_and_classes());
        let left: Vec<String> = vec![
            "class1".to_string(),
            "class2".to_string(),
            "class3".to_string(),
        ];
        let right = payload_section.classes;
        assert_eq!(left, right);
    }

    // #[test]
    // fn classes_dont_show_up_in_attrs() {
    //     let payload_section =
    //         PayloadSection::new_from_section(&Section::mock4_youtube_with_tags_and_classes());
    //     let left: Vec<PayloadSectionAttr> = vec![];
    //     let right = payload_section.attrs;
    //     assert_eq!(left, right);
    // }

    #[test]
    fn created_check() {
        let section = Section::mock6_div_with_created_and_updated_and_status();
        let left = "2024-01-01T00:00:00-04:00";
        let right = &PayloadSection::new_from_section(&section).created.unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // fn created_dont_show_up_in_attrs() {
    //     let payload_section = PayloadSection::new_from_section(
    //         &Section::mock6_div_with_created_and_updated_and_status(),
    //     );
    //     let left: Vec<PayloadSectionAttr> = vec![];
    //     let right = payload_section.attrs;
    //     assert_eq!(left, right);
    // }

    #[test]
    fn flags_work() {
        let payload_section =
            PayloadSection::new_from_section(&Section::mock4_youtube_with_tags_and_classes());
        let left = vec!["NPJ1qQraMZI".to_string()];
        let right = payload_section.flags;
        assert_eq!(left, right);
    }

    #[test]
    fn id_check() {
        let payload_section = PayloadSection::new_from_section(&Section::mock5_div_with_id());
        let left = "attr-id";
        let right = payload_section.id.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn section_template_list_from_attr() {
        let payload_section =
            PayloadSection::new_from_section(&Section::mock2_div_with_title_and_template_attrs());
        let left = vec![
            "sections/div/full/template-from-attr.neoj".to_string(),
            "sections/div/full/default.neoj".to_string(),
            "sections/generic/full/default.neoj".to_string(),
        ];
        let right = payload_section.template_list;
        assert_eq!(left, right);
    }

    #[test]
    fn status_check() {
        let section = Section::mock6_div_with_created_and_updated_and_status();
        let left = "section-status-example";
        let right = &PayloadSection::new_from_section(&section).status.unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // fn status_does_not_show_up_in_attrs() {
    //     let payload_section = PayloadSection::new_from_section(
    //         &Section::mock6_div_with_created_and_updated_and_status(),
    //     );
    //     let left: Vec<PayloadSectionAttr> = vec![];
    //     let right = payload_section.attrs;
    //     assert_eq!(left, right);
    // }

    #[test]
    fn tags_work() {
        let payload_section =
            PayloadSection::new_from_section(&Section::mock4_youtube_with_tags_and_classes());
        let left = vec!["minecraft".to_string(), "how-to".to_string()];
        let right = payload_section.tags;
        assert_eq!(left, right);
    }

    // #[test]
    // fn tags_dont_show_up_in_attrs() {
    //     let payload_section =
    //         PayloadSection::new_from_section(&Section::mock4_youtube_with_tags_and_classes());
    //     let left: Vec<PayloadSectionAttr> = vec![];
    //     let right = payload_section.attrs;
    //     assert_eq!(left, right);
    // }

    #[test]
    fn template_list_check() {
        let payload_section =
            PayloadSection::new_from_section(&Section::mock1_basic_title_section_no_attrs());
        let left = vec![
            "sections/title/full/default.neoj".to_string(),
            "sections/generic/full/default.neoj".to_string(),
        ];
        let right = payload_section.template_list;
        assert_eq!(left, right);
    }

    #[test]
    fn type_of_seciton() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = "title".to_string();
        let right = PayloadSection::new_from_section(&section).r#type;
        assert_eq!(left, right);
    }

    #[test]
    fn updated_check() {
        let section = Section::mock6_div_with_created_and_updated_and_status();
        let left = "2024-01-02T00:00:00-04:00";
        let right = &PayloadSection::new_from_section(&section).updated.unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // fn updated_dont_show_up_in_attrs() {
    //     let payload_section = PayloadSection::new_from_section(
    //         &Section::mock6_div_with_created_and_updated_and_status(),
    //     );
    //     let left: Vec<PayloadSectionAttr> = vec![];
    //     let right = payload_section.attrs;
    //     assert_eq!(left, right);
    // }

    //
}
