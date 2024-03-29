mod site_page_output_path {
    use minijinja::Value;
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn path_for_page_without_override_path() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some(
            "leading-dir/Neopoligen/set2-test-site/docs/en/page-without-override-path/index.html"
                .to_string(),
        );
        let right = site.page_output_path(&[Value::from("page-without-override-path")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn home_page_override_path() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("leading-dir/Neopoligen/set2-test-site/docs/index.html".to_string());
        let right = site.page_output_path(&[Value::from("site2-home-page")]);
        assert_eq!(left, right);
    }
}
