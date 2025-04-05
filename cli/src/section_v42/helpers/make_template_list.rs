use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use itertools::Itertools;
use std::collections::BTreeMap;

// TODO: Deprecate this
pub fn make_template_list(r#type: &str, kind: &str, template: &str) -> Vec<String> {
    let templates = vec![
        format!("sections/{}/{}.neoj", kind, template),
        format!("sections/{}/default.neoj", kind),
        format!("sections/{}/default.neoj", r#type),
    ];
    templates.into_iter().unique().collect()
}

// TODO: Add an automated test for this.
pub fn make_template_list_v42(
    r#type: &str,
    kind: &str,
    attrs: BTreeMap<String, Vec<SectionKeyValueAttrPayloadV42>>,
) -> Vec<String> {
    let template_name = match &attrs.get("template") {
        Some(attr) => attr[0].text.clone(),
        None => String::from("default"),
    };
    let templates = vec![
        format!("sections/{}/{}.neoj", kind, template_name),
        format!("sections/{}/default.neoj", kind),
        format!("sections/{}/default.neoj", r#type),
    ];
    templates.into_iter().unique().collect()
    // vec![]
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    // TODO: Update this for the built-in templates
    #[test]
    #[ignore]
    fn check_make_template_list() {
        let left = vec![
            "sections/title/custom.neoj".to_string(),
            "sections/title/default.neoj".to_string(),
        ];
        let right = make_template_list("basic", "title", "custom");
        assert_eq!(left, right);
    }
}
