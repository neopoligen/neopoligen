mod site_page_output_path {
    use minijinja::Value;
    use neopoligengine::config::Config;
    use neopoligengine::file_set::FileSet;
    use neopoligengine::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn path_for_page_without_override_path() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some(
            "leading-dir/Neopoligen/set2-test-site/_building/en/page-without-override-path/index.html"
                .to_string(),
        );
        let right = site.page_build_path(&[Value::from("page-without-override-path")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn home_page_override_path() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("leading-dir/Neopoligen/set2-test-site/_building/index.html".to_string());
        let right = site.page_build_path(&[Value::from("site2-home-page")]);
        assert_eq!(left, right);
    }
}
