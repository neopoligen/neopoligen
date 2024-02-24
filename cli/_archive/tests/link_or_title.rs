#[cfg(test)]
mod test_link_or_title {
    use minijinja::Value;
    use neopoligen_cli::site_v2::SiteV2;
    use pretty_assertions::assert_eq;
    //use std::collections::BTreeMap;

    #[test]
    fn link_or_title_same_page_returns_title() {
        let site = SiteV2::site_with_eight_pages();
        let current_page = Value::from("id12345c");
        let target_page = Value::from("id12345c");
        let exclude_tags = Value::from_serializable::<Vec<&str>>(&vec![]);
        let title_override = Value::from("".to_string());
        let left = Some(r#"Delta Oscar"#.to_string());
        let right = site.link_or_title(&[current_page, target_page, exclude_tags, title_override]);
        assert_eq!(left, right);
    }

    #[test]
    fn link_or_title_target_without_filters() {
        let site = SiteV2::site_with_eight_pages();
        let current_page = Value::from("id12345c");
        let target_page = Value::from("id12345e");
        let exclude_tags = Value::from_serializable::<Vec<&str>>(&vec![]);
        let title_override = Value::from("".to_string());
        let left = Some(r#"<a href="/en/id12345e/?tango-foxtrot">Tango Foxtrot</a>"#.to_string());
        let right = site.link_or_title(&[current_page, target_page, exclude_tags, title_override]);
        assert_eq!(left, right);
    }

    #[test]
    fn link_or_title_target_with_matching_exclude_filter() {
        let site = SiteV2::site_with_eight_pages();
        let current_page = Value::from("id12345c");
        let target_page = Value::from("id12345d");
        let exclude_tags = Value::from_serializable::<Vec<&str>>(&vec!["main-body-test"]);
        let title_override = Value::from("".to_string());
        let left = Some(r#"Bravo Charlie"#.to_string());
        let right = site.link_or_title(&[current_page, target_page, exclude_tags, title_override]);
        assert_eq!(left, right);
    }

    #[test]
    fn link_or_title_target_with_non_matching_exclude_filter() {
        let site = SiteV2::site_with_eight_pages();
        let current_page = Value::from("id12345c");
        let target_page = Value::from("id12345d");
        let exclude_tags = Value::from_serializable::<Vec<&str>>(&vec!["non-matching-filter"]);
        let title_override = Value::from("".to_string());
        let left = Some(r#"<a href="/en/id12345d/?bravo-charlie">Bravo Charlie</a>"#.to_string());
        let right = site.link_or_title(&[current_page, target_page, exclude_tags, title_override]);
        assert_eq!(left, right);
    }

    #[test]
    fn link_or_title_target_with_override_title_and_no_excludes() {
        let site = SiteV2::site_with_eight_pages();
        let current_page = Value::from("id12345c");
        let target_page = Value::from("id778866");
        let exclude_tags = Value::from_serializable::<Vec<&str>>(&vec![]);
        let title_override = Value::from("New Title Here".to_string());
        let left = Some(r#"<a href="/en/id778866/?filter-test">New Title Here</a>"#.to_string());
        let right = site.link_or_title(&[current_page, target_page, exclude_tags, title_override]);
        assert_eq!(left, right);
    }

    #[test]
    fn link_or_title_target_with_matching_exclude_filter_and_title_override() {
        let site = SiteV2::site_with_eight_pages();
        let current_page = Value::from("id12345c");
        let target_page = Value::from("id12345d");
        let exclude_tags = Value::from_serializable::<Vec<&str>>(&vec!["main-body-test"]);
        let title_override = Value::from("Title From Override".to_string());
        let left = Some(r#"Title From Override"#.to_string());
        let right = site.link_or_title(&[current_page, target_page, exclude_tags, title_override]);
        assert_eq!(left, right);
    }

    #[test]
    fn link_or_title_target_with_non_matching_exclude_filter_with_override_title() {
        let site = SiteV2::site_with_eight_pages();
        let current_page = Value::from("id12345c");
        let target_page = Value::from("id12345d");
        let exclude_tags = Value::from_serializable::<Vec<&str>>(&vec!["non-matching-filter"]);
        let title_override = Value::from("Working Code Would Be Great".to_string());
        let left = Some(
            r#"<a href="/en/id12345d/?bravo-charlie">Working Code Would Be Great</a>"#.to_string(),
        );
        let right = site.link_or_title(&[current_page, target_page, exclude_tags, title_override]);
        assert_eq!(left, right);
    }
}
