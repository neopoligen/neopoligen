pub fn make_template_list(r#type: &str, kind: &str, bounds: &str, template: &str) -> Vec<String> {
    let mut result = vec![];
    result.push(format!(
        "sections/{}/{}/{}/{}.neoj",
        r#type, kind, bounds, template
    ));
    result
}

#[cfg(test)]
mod test {
    use super::make_template_list;
    use pretty_assertions::assert_eq;

    #[test]
    fn check_make_template_list() {
        let left = vec!["sections/basic/title/full/default.neoj"];
        let right = make_template_list("basic", "title", "full", "default");
        assert_eq!(left, right);
    }
}
