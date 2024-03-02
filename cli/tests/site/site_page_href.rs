mod site_page_href {
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;
    use pretty_assertions::assert_eq;
    use minijinja::Value;

    #[test]
    pub fn basic() {
        let file_set = FileSet::set2();
        let config = Config::site2_config();
        let site = Site::new(&file_set, &config);
        let left = Some("/en/url-escape-title-check/?url-escape-%2F-title-check".to_string());
        let right = site.page_href(&[Value::from("url-escape-title-check")]);
        assert_eq!(left, right);
    }

}
