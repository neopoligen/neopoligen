use itertools::Itertools;

pub fn make_template_list(r#type: &str, kind: &str, template: &str) -> Vec<String> {
    let templates = vec![
        format!("sections/{}/{}/{}.neoj", r#type, kind, template),
        format!("sections/{}/{}/default.neoj", r#type, kind),
        format!("sections/{}/generic/default.neoj", r#type),
    ];
    templates.into_iter().unique().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn check_make_template_list() {
        let left = vec![
            "sections/basic/title/custom.neoj".to_string(),
            "sections/basic/title/default.neoj".to_string(),
            "sections/basic/generic/default.neoj".to_string(),
        ];
        let right = make_template_list("basic", "title", "custom");
        assert_eq!(left, right);
    }
}
