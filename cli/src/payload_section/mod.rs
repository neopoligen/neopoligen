use crate::helpers::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{
    payload_span::PayloadSpan,
    section::{Section, SectionBounds, SectionKind},
    section_attr::SectionAttrKind,
    site_config::SiteConfig,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSection {
    pub aria: BTreeMap<String, String>,
    pub attr_string: Option<String>,
    pub attrs: BTreeMap<String, String>,
    pub attrs_as_spans: BTreeMap<String, Vec<PayloadSpan>>,
    pub bounds: String,
    pub children: Vec<PayloadSection>,
    pub classes: Option<String>,
    pub created: Option<String>,
    pub data: BTreeMap<String, String>,
    pub flags: Vec<String>,
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
    pub fn new_from_section(section: &Section, config: &SiteConfig) -> PayloadSection {
        let mut aria: BTreeMap<String, String> = BTreeMap::new();
        section.attrs.iter().for_each(|attr| match &attr.kind {
            SectionAttrKind::KeyValueSpans { key, spans } => {
                if key.starts_with("aria-") {
                    let single_key = key.strip_prefix("aria-").unwrap();
                    match aria.get(single_key) {
                        Some(current) => {
                            aria.insert(
                                single_key.to_string(),
                                format!("{} {}", current, flatten_spans(spans)),
                            );
                        }
                        None => {
                            aria.insert(single_key.to_string(), flatten_spans(spans));
                        }
                    }
                }
            }
            _ => {}
        });

        let mut attrs: BTreeMap<String, String> = BTreeMap::new();
        section.attrs.iter().for_each(|attr| match &attr.kind {
            SectionAttrKind::KeyValueSpans { key, spans } => {
                if key.as_str() != "aria" && key.as_str() != "class" && key.as_str() != "data" {
                    match attrs.get(key) {
                        Some(cur) => {
                            attrs.insert(key.clone(), format!("{} {}", cur, flatten_spans(spans)));
                        }
                        None => {
                            attrs.insert(key.clone(), format!("{}", flatten_spans(spans)));
                        }
                    };
                }
            }
            _ => (),
        });

        let mut attrs_as_spans: BTreeMap<String, Vec<PayloadSpan>> = BTreeMap::new();
        section.attrs.iter().for_each(|attr| match &attr.kind {
            SectionAttrKind::KeyValueSpans { key, spans } => {
                if key.as_str() != "aria" && key.as_str() != "class" && key.as_str() != "data" {
                    match attrs_as_spans.get(key) {
                        Some(cur) => {
                            let mut new_attrs = cur.clone();
                            new_attrs.push(PayloadSpan::new_space());
                            spans.iter().for_each(|span| {
                                new_attrs.push(PayloadSpan::new_from_span(span, config))
                            });
                            attrs_as_spans.insert(key.clone(), new_attrs);
                        }
                        None => {
                            attrs_as_spans.insert(
                                key.clone(),
                                spans
                                    .iter()
                                    .map(|span| PayloadSpan::new_from_span(span, config))
                                    .collect(),
                            );
                        }
                    };
                }
            }
            _ => (),
        });

        let bounds = match section.bounds {
            SectionBounds::End => "end".to_string(),
            SectionBounds::Full => "full".to_string(),
            SectionBounds::Start => "start".to_string(),
        };

        let children = match &section.kind {
            SectionKind::Basic { children } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child, config))
                .collect(),
            SectionKind::Block { .. } => vec![],
            SectionKind::Checklist { children } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child, config))
                .collect(),
            SectionKind::ChecklistItem { children } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child, config))
                .collect(),
            SectionKind::List { children } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child, config))
                .collect(),
            SectionKind::Json { children, .. } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child, config))
                .collect(),
            SectionKind::ListItem { children } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child, config))
                .collect(),
            SectionKind::Raw { children, .. } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child, config))
                .collect(),
            SectionKind::Unknown { children } => children
                .iter()
                .map(|child| PayloadSection::new_from_section(child, config))
                .collect(),
            SectionKind::Yaml { .. } => vec![],
        };

        let tmp_classes = section
            .attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SectionAttrKind::KeyValueSpans { key, spans } => {
                    if key.to_lowercase() == "class" {
                        Some(
                            flatten_spans(spans)
                                .clone()
                                .split(" ")
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>(),
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .flatten()
            .collect::<Vec<String>>()
            .join(" ");

        let classes = if tmp_classes.ne("") {
            Some(tmp_classes)
        } else {
            None
        };

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

        let mut data: BTreeMap<String, String> = BTreeMap::new();
        section.attrs.iter().for_each(|attr| match &attr.kind {
            SectionAttrKind::KeyValueSpans { key, spans } => {
                if key.starts_with("data-") {
                    let single_key = key.strip_prefix("data-").unwrap();
                    match data.get(single_key) {
                        Some(current) => {
                            data.insert(
                                single_key.to_string(),
                                format!("{} {}", current, flatten_spans(spans)),
                            );
                        }
                        None => {
                            data.insert(single_key.to_string(), flatten_spans(spans));
                        }
                    }
                }
            }
            _ => {}
        });

        let flags = section
            .attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SectionAttrKind::Flag { flag } => Some(flag.to_string()),
                _ => None,
            })
            .collect::<Vec<String>>();

        let kind = Some(match &section.kind {
            SectionKind::Basic { .. } => "basic".to_string(),
            SectionKind::Block { .. } => "block".to_string(),
            SectionKind::Checklist { .. } => "checklist".to_string(),
            SectionKind::ChecklistItem { .. } => "checklist-item".to_string(),
            SectionKind::Json { .. } => "json".to_string(),
            SectionKind::List { .. } => "list".to_string(),
            SectionKind::ListItem { .. } => "list-item".to_string(),
            SectionKind::Raw { .. } => "raw".to_string(),
            SectionKind::Unknown { .. } => "unknown".to_string(),
            SectionKind::Yaml { .. } => "yaml".to_string(),
        });

        let status = section.attrs.iter().find_map(|attr| match &attr.kind {
            SectionAttrKind::KeyValueSpans { key, spans } => {
                if key.as_str() == "status" {
                    Some(flatten_spans(spans))
                } else {
                    None
                }
            }
            _ => None,
        });

        let spans = match &section.kind {
            SectionKind::Block { spans } => spans
                .iter()
                .map(|span| PayloadSpan::new_from_span(&span, config))
                .collect::<Vec<PayloadSpan>>(),
            _ => vec![],
        };

        let mut template_list = vec![];
        if let Some(template) = attrs.get("template") {
            template_list.push(format!(
                "sections/{}/{}/{}/{}.neoj",
                kind.as_ref().unwrap(),
                section.r#type,
                bounds,
                template
            ));
        }
        template_list.push(format!(
            "sections/{}/{}/{}/default.neoj",
            kind.as_ref().unwrap(),
            section.r#type,
            bounds
        ));
        template_list.push(format!(
            "sections/{}/generic/{}/default.neoj",
            kind.as_ref().unwrap(),
            bounds
        ));
        template_list.push(format!("sections/unknown/generic/{}/default.neoj", bounds));

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

        let mut ps = PayloadSection {
            aria,
            attr_string: None,
            attrs,
            attrs_as_spans,
            bounds,
            children,
            classes,
            created,
            data,
            flags,
            kind,
            spans,
            status,
            tags,
            text,
            r#type: section.r#type.clone(),
            template_list,
            updated,
        };
        ps.make_attr_string();
        ps
    }
}

impl PayloadSection {
    pub fn id(&mut self) -> Option<String> {
        if let Some(id_spans) = self.attrs.get("id") {
            Some(id_spans.to_string())
        } else {
            None
        }

        // self.attrs.iter().find_map(|attr| match &attr.kind {
        //     SectionAttrKind::KeyValueSpans { key, spans } => {
        //         if key.as_str() == "id" {
        //             Some(flatten_spans(spans))
        //         } else {
        //             None
        //         }
        //     }
        //     _ => None,
        // })
    }

    pub fn make_attr_string(&mut self) {
        let mut attr_string = String::from("");

        // TODO: Handle ID here
        if let Some(id) = &self.id() {
            attr_string.push_str(format!(r#" id="{}""#, id).as_str());
        }

        let _ = &self.aria.iter().for_each(|(key, value)| {
            attr_string.push_str(format!(r#" aria-{}="{}""#, key, value).as_str());
        });

        let _ = &self.data.iter().for_each(|(key, value)| {
            attr_string.push_str(format!(r#" data-{}="{}""#, key, value).as_str());
        });

        self.attr_string = Some(attr_string);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::{Span, SpanKind};
    use pretty_assertions::assert_eq;

    #[test]
    fn aria_values_basic() {
        let ps = PayloadSection::new_from_section(
            &Section::mock9_aria_data(),
            &SiteConfig::mock1_basic(),
        );
        let left = "alfa bravo charlie delta";
        let r1 = ps.aria;
        let right = r1.get("description").unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn attr_string_with_aria() {
        let ps = PayloadSection::new_from_section(
            &Section::mock9_aria_data(),
            &SiteConfig::mock1_basic(),
        );
        let left = r#" aria-description="alfa bravo charlie delta""#;
        let right = ps.attr_string.unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // fn attr_string_with_classes() {
    //     let ps = PayloadSection::new_from_section(
    //         &Section::mock4_youtube_with_tags_and_classes(),
    //         &SiteConfig::mock1_basic(),
    //     );
    //     let left = r#" class="class1 class2 class3""#;
    //     let right = ps.attr_string.unwrap();
    //     assert_eq!(left, right);
    // }

    #[test]
    fn attr_string_with_data() {
        let ps = PayloadSection::new_from_section(
            &Section::mock9_aria_data(),
            &SiteConfig::mock1_basic(),
        );
        let left = r#" aria-description="alfa bravo charlie delta""#;
        let right = ps.attr_string.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn attr_string_with_id() {
        let ps = PayloadSection::new_from_section(
            &Section::mock5_div_with_id(),
            &SiteConfig::mock1_basic(),
        );
        let left = r#" id="attr-id""#;
        let right = ps.attr_string.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn attrs_basic_check() {
        let ps = PayloadSection::new_from_section(
            &Section::mock3_image_with_flag_and_multiple_attrs_with_same_key(),
            &SiteConfig::mock1_basic(),
        );
        let left = "alfa bravo charlie delta";
        let right = ps.attrs.get("alt").unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn attrs_as_spans_basic_check() {
        let ps = PayloadSection::new_from_section(
            &Section::mock3_image_with_flag_and_multiple_attrs_with_same_key(),
            &SiteConfig::mock1_basic(),
        );
        let left = &vec![
            PayloadSpan {
                aria: None,
                aria_unescaped: None,
                attr_string: None,
                attrs: None,
                attrs_unescaped: None,
                children: vec![],
                classes: None,
                classes_unescaped: None,
                custom_attrs: None,
                custom_attrs_unescaped: None,
                data: None,
                data_unescaped: None,
                first_flag: None,
                first_flag_unescaped: None,
                flags: None,
                flags_unescaped: None,
                id: None,
                id_unescaped: None,
                kind: "basic".to_string(),
                r#type: "wordpart".to_string(),
                parsed_text: "alfa bravo".to_string(),
                template_list: vec![
                    "spans/wordpart.neoj".to_string(),
                    "spans/generic-basic-span.neoj".to_string(),
                ],
            },
            PayloadSpan {
                aria: None,
                aria_unescaped: None,
                attr_string: None,
                attrs: None,
                attrs_unescaped: None,
                children: vec![],
                classes: None,
                classes_unescaped: None,
                custom_attrs: None,
                custom_attrs_unescaped: None,
                data: None,
                data_unescaped: None,
                first_flag: None,
                first_flag_unescaped: None,
                flags: None,
                flags_unescaped: None,
                id: None,
                id_unescaped: None,
                kind: "basic".to_string(),
                r#type: "space".to_string(),
                parsed_text: " ".to_string(),
                template_list: vec![
                    "spans/space.neoj".to_string(),
                    "spans/generic-basic-span.neoj".to_string(),
                ],
            },
            PayloadSpan {
                aria: None,
                aria_unescaped: None,
                attr_string: None,
                attrs: None,
                attrs_unescaped: None,
                children: vec![],
                classes: None,
                classes_unescaped: None,
                custom_attrs: None,
                custom_attrs_unescaped: None,
                data: None,
                data_unescaped: None,
                first_flag: None,
                first_flag_unescaped: None,
                flags: None,
                flags_unescaped: None,
                id: None,
                id_unescaped: None,
                kind: "basic".to_string(),
                r#type: "wordpart".to_string(),
                parsed_text: "charlie delta".to_string(),
                template_list: vec![
                    "spans/wordpart.neoj".to_string(),
                    "spans/generic-basic-span.neoj".to_string(),
                ],
            },
        ];
        let right = ps.attrs_as_spans.get("alt").unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn bounds_check() {
        let config = SiteConfig::mock1_basic();
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = "full";
        let right = &PayloadSection::new_from_section(&section, &config).bounds;
        assert_eq!(left, right);
    }

    #[test]
    fn data_values_basic() {
        let ps = PayloadSection::new_from_section(
            &Section::mock10_data_attrs(),
            &SiteConfig::mock1_basic(),
        );
        let left = "alfa bravo charlie delta";
        let r1 = ps.data;
        let right = r1.get("test").unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn flatten_parsed_text_basic() {
        let span = Span {
            attrs: vec![],
            parsed_text: "some text".to_string(),
            kind: SpanKind::WordPart,
        };
        let left = "some text".to_string();
        let right = flatten_parsed_text(&span);
        assert_eq!(left, right);
    }

    #[test]
    fn flatten_parsed_text_from_named_span() {
        let span = Span {
            attrs: vec![],
            parsed_text: "".to_string(),
            kind: SpanKind::NamedSpan {
                children: vec![Span {
                    attrs: vec![],
                    kind: SpanKind::WordPart,
                    parsed_text: "alfa".to_string(),
                }],
                r#type: "em".to_string(),
            },
        };
        let left = "alfa".to_string();
        let right = flatten_parsed_text(&span);
        assert_eq!(left, right);
    }

    // #[test]
    // #[ignore]
    // fn todo_check_template_is_not_in_attrs() {
    //     // TODO
    // }

    // #[test]
    // #[ignore]
    // fn todo_check_data_from_json() {
    //     // TODO
    // }

    // #[test]
    // #[ignore]
    // fn todo_check_data_from_yaml() {
    //     // TODO
    // }

    // #[test]
    // #[ignore]
    // fn todo_check_text_from_raw_section() {
    //     // TODO
    // }

    #[test]
    fn classes_work() {
        let config = SiteConfig::mock1_basic();
        let payload_section = PayloadSection::new_from_section(
            &Section::mock4_youtube_with_tags_and_classes(),
            &config,
        );
        let left = "class1 class2 class3".to_string();
        let right = payload_section.classes.unwrap();
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

    // #[test]
    // #[ignore]
    // fn created_check() {
    //     let config = SiteConfig::mock1_basic();
    //     let section = Section::mock6_div_with_created_and_updated_and_status();
    //     let left = "2024-01-01T00:00:00-04:00";
    //     let right = &PayloadSection::new_from_section(&section, &config)
    //         .created
    //         .unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn created_dont_show_up_in_attrs() {
    //     let payload_section = PayloadSection::new_from_section(
    //         &Section::mock6_div_with_created_and_updated_and_status(),
    //     );
    //     let left: Vec<PayloadSectionAttr> = vec![];
    //     let right = payload_section.attrs;
    //     assert_eq!(left, right);
    // }

    // #[test]
    // #[ignore]
    // fn flags_work() {
    //     let config = SiteConfig::mock1_basic();
    //     let payload_section = PayloadSection::new_from_section(
    //         &Section::mock4_youtube_with_tags_and_classes(),
    //         &config,
    //     );
    //     let left = vec!["NPJ1qQraMZI".to_string()];
    //     let right = payload_section.flags;
    //     assert_eq!(left, right);
    // }

    // // DEPRECATED: TODO: Remove
    // #[test]
    // fn id_basic_check() {
    //     let config = SiteConfig::mock1_basic();
    //     let payload_section =
    //         PayloadSection::new_from_section(&Section::mock5_div_with_id(), &config);
    //     let left = "attr-id";
    //     let right = payload_section.id.unwrap();
    //     assert_eq!(left, right);
    // }

    // // DEPRECATED: TODO: Remove
    // #[test]
    // fn id_metadata_check() {
    //     let config = SiteConfig::mock1_basic();
    //     let payload_section =
    //         PayloadSection::new_from_section(&Section::mock8_metadata_basic(), &config);
    //     let left = "id_from_metadata";
    //     let right = payload_section.id.unwrap();
    //     assert_eq!(left, right);
    // }

    #[test]
    fn section_template_list_from_attr() {
        let config = SiteConfig::mock1_basic();
        let payload_section = PayloadSection::new_from_section(
            &Section::mock2_div_with_title_and_template_attrs(),
            &config,
        );
        let left = vec![
            "sections/basic/div/full/template-from-attr.neoj".to_string(),
            "sections/basic/div/full/default.neoj".to_string(),
            "sections/basic/generic/full/default.neoj".to_string(),
            "sections/unknown/generic/full/default.neoj".to_string(),
        ];
        let right = payload_section.template_list;
        assert_eq!(left, right);
    }

    #[test]
    fn status_check() {
        let config = SiteConfig::mock1_basic();
        let section = Section::mock6_div_with_created_and_updated_and_status();
        let left = "section-status-example";
        let right = &PayloadSection::new_from_section(&section, &config)
            .status
            .unwrap();
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

    // #[test]
    // #[ignore]
    // fn tags_work() {
    //     let config = SiteConfig::mock1_basic();
    //     let payload_section = PayloadSection::new_from_section(
    //         &Section::mock4_youtube_with_tags_and_classes(),
    //         &config,
    //     );
    //     let left = vec!["minecraft".to_string(), "how-to".to_string()];
    //     let right = payload_section.tags;
    //     assert_eq!(left, right);
    // }

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
        let config = SiteConfig::mock1_basic();
        let payload_section = PayloadSection::new_from_section(
            &Section::mock1_basic_title_section_no_attrs(),
            &config,
        );
        let left = vec![
            "sections/basic/title/full/default.neoj".to_string(),
            "sections/basic/generic/full/default.neoj".to_string(),
            "sections/unknown/generic/full/default.neoj".to_string(),
        ];
        let right = payload_section.template_list;
        assert_eq!(left, right);
    }

    #[test]
    fn type_of_section() {
        let config = SiteConfig::mock1_basic();
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = "title".to_string();
        let right = PayloadSection::new_from_section(&section, &config).r#type;
        assert_eq!(left, right);
    }

    // #[test]
    // #[ignore]
    // fn updated_check() {
    //     let config = SiteConfig::mock1_basic();
    //     let section = Section::mock6_div_with_created_and_updated_and_status();
    //     let left = "2024-01-02T00:00:00-04:00";
    //     let right = &PayloadSection::new_from_section(&section, &config)
    //         .updated
    //         .unwrap();
    //     assert_eq!(left, right);
    // }

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
