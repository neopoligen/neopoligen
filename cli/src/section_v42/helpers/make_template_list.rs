use std::collections::BTreeSet;

// pub fn old_make_template_list(
//     r#type: &str,
//     kind: &str,
//     bounds: &str,
//     template: &str,
// ) -> Vec<String> {
//     let mut result = vec![];
//     result.push(format!(
//         "sections/{}/{}/{}/{}.neoj",
//         r#type, kind, bounds, template
//     ));
//     result.push(format!(
//         "sections/{}/{}/{}/default.neoj",
//         r#type, kind, bounds
//     ));
//     result.push(format!(
//         "sections/{}/generic/{}/default.neoj",
//         r#type, bounds
//     ));
//     result
// }

pub fn make_template_list(
    r#type: &str,
    kind: &str,
    bounds: &str,
    template: &str,
) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    result.insert(format!(
        "sections/{}/{}/{}/{}.neoj",
        r#type, kind, bounds, template
    ));
    result.insert(format!(
        "sections/{}/{}/{}/default.neoj",
        r#type, kind, bounds
    ));
    result.insert(format!(
        "sections/{}/generic/{}/default.neoj",
        r#type, bounds
    ));
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn solo_check_make_template_list() {
        let mut left = BTreeSet::new();
        left.insert("sections/basic/title/full/custom.neoj".to_string());
        left.insert("sections/basic/title/full/default.neoj".to_string());
        left.insert("sections/basic/generic/full/default.neoj".to_string());
        let right = make_template_list("basic", "title", "full", "custom");
        assert_eq!(left, right);
    }
}
