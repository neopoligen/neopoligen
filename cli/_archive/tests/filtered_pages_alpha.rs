#[cfg(test)]
mod filtered_pages_alpha_test {
    use minijinja::value::Value;
    use neopoligen_cli::site_v2::SiteV2;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn filtered_pages_alpha_includes_only() {
        let site = SiteV2::site_with_eight_pages();
        let include_tags = Value::from_serializable(&vec![vec!["published"]]);
        let exclude_tags = Value::from_serializable::<Vec<&str>>(&vec![]);
        let left: Vec<String> = vec!["id003333".to_string(), "id778866".to_string()];
        let right = site.filtered_pages_alpha(&[include_tags, exclude_tags]);
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn filtered_pages_alpha_includes_and_exclude() {
        let site = SiteV2::site_with_eight_pages();
        let arg1 = Value::from("id12345c");
        let mut arg2_data: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        arg2_data.insert("include_tags", vec!["published"]);
        arg2_data.insert("exclude_tags", vec!["id003333"]);
        let arg2 = Value::from(arg2_data);
        let left: Vec<String> = vec!["id778866".to_string()];
        let right = site.filtered_pages_alpha(&[arg1, arg2]);
        assert_eq!(left, right);
    }
}
