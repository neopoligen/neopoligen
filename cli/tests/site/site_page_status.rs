mod site_page_status {
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;
    use pretty_assertions::assert_eq;
    use minijinja::Value;


    #[test]
    pub fn page_status_in_metadata() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("example_status".to_string());
        let right = site.page_status(&[Value::from("page-status-in-metadata")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_status_not_in_metadata() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("published".to_string());
        let right = site.page_status(&[Value::from("page-status-not-in-metadata")]);
        assert_eq!(left, right);
    }

}
