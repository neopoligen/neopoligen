use itertools::Itertools;

pub fn make_template_list(r#type: &str, kind: &str, template: &str) -> Vec<String> {
    let templates = vec![
        format!("sections/{}/{}.neoj", kind, template),
        format!("sections/{}/default.neoj", kind),
        format!("built-in/sections/{}/default.neoj", kind),
        format!("built-in/sections/generic-{}/default.neoj", r#type),
    ];
    templates.into_iter().unique().collect()
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
            "sections/generic-basic/custom.neoj".to_string(),
            "sections/generic-basic/default.neoj".to_string(),
        ];
        let right = make_template_list("basic", "title", "custom");
        assert_eq!(left, right);
    }
}
