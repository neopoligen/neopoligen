use itertools::Itertools;

pub fn make_template_list(r#type: &str, kind: &str, bounds: &str, template: &str) -> Vec<String> {
    let templates = vec![
        format!("sections/{}/{}/{}/{}.neoj", r#type, kind, bounds, template),
        format!("sections/{}/{}/{}/default.neoj", r#type, kind, bounds),
        format!("sections/{}/generic/{}/default.neoj", r#type, bounds),
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
            "sections/basic/title/full/custom.neoj".to_string(),
            "sections/basic/title/full/default.neoj".to_string(),
            "sections/basic/generic/full/default.neoj".to_string(),
        ];
        let right = make_template_list("basic", "title", "full", "custom");
        assert_eq!(left, right);
    }
}
