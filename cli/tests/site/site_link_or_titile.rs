mod link_or_title {
    use minijinja::Value;
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn get_a_link_back() {
        let file_set = FileSet::set2();
        let config = Config::site2_config();
        let site = Site::new(&file_set, &config);
        let left = Some(
            r#"<a href="/en/link-or-title-target/?link-or-title-target">Link Or Title Target</a>"#
                .to_string(),
        );
        let right = site.link_or_title(&[
            Value::from("link-or-title-start"),
            Value::from("link-or-title-target"),
            Value::from_serializable::<Vec<&str>>(&vec![]),
        ]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn get_text_back_for_self() {
        let file_set = FileSet::set2();
        let config = Config::site2_config();
        let site = Site::new(&file_set, &config);
        let left = Some(r#"Link Or Title Start"#.to_string());
        let right = site.link_or_title(&[
            Value::from("link-or-title-start"),
            Value::from("link-or-title-start"),
            Value::from_serializable::<Vec<&str>>(&vec![]),
        ]);
        assert_eq!(left, right);
    }
}
