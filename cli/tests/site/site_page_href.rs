mod site_page_href {
    use minijinja::Value;
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn default_href() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("/en/url-escape-title-check/?url-escape-%2F-title-check".to_string());
        let right = site.page_href(&[Value::from("url-escape-title-check")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn override_path() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("/".to_string());
        let right = site.page_href(&[Value::from("site2-home-page")]);
        assert_eq!(left, right);
    }
}
