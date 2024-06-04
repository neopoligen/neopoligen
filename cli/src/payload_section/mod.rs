use serde::{Deserialize, Serialize};

use crate::{
    payload_section_attr::PayloadSectionAttr,
    section::{Section, SectionBounds, SectionKind},
    section_attr::SectionAttrKind,
    span::Span,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PayloadSection {
    pub attrs: Vec<PayloadSectionAttr>,
    pub bounds: String,
    pub children: Vec<PayloadSection>,
    pub created: Option<String>,
    pub flags: Vec<String>,
    pub spans: Vec<Span>,
    pub tags: Vec<String>,
    pub text: Option<String>,
    pub r#type: String,
    pub template_list: Vec<String>,
    pub updated: Option<String>,
}

impl PayloadSection {
    pub fn new_from_section(section: &Section) -> PayloadSection {
        let attrs = section
            .attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SectionAttrKind::KeyValue { key, value } => Some(PayloadSectionAttr {
                    key: key.to_string(),
                    value: value.to_string(),
                }),
                _ => None,
            })
            .collect::<Vec<PayloadSectionAttr>>();
        let bounds = match section.bounds {
            SectionBounds::End => "end".to_string(),
            SectionBounds::Full => "full".to_string(),
            SectionBounds::Start => "start".to_string(),
        };
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
            _ => vec![],
        };

        let spans = match &section.kind {
            SectionKind::Block { spans } => spans.clone(),
            _ => vec![],
        };

        PayloadSection {
            attrs,
            bounds,
            children,
            created: None,
            flags: vec![],
            spans,
            tags: vec![],
            text: None,
            r#type: "title".to_string(),
            template_list,
            updated: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn bounds_check() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = "full";
        let right = &PayloadSection::new_from_section(&section).bounds;
        assert_eq!(left, right);
    }

    #[test]
    fn get_attrs() {
        let section = Section::mock2_div_with_title_and_template_attrs();
        let left = "Title From Attr";
        let right = &PayloadSection::new_from_section(&section).attrs[0].value;
        assert_eq!(left, right);
    }

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
    fn type_of_seciton() {
        let section = Section::mock1_basic_title_section_no_attrs();
        let left = "title".to_string();
        let right = PayloadSection::new_from_section(&section).r#type;
        assert_eq!(left, right);
    }

    //
}
