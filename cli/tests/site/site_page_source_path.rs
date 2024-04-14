mod site_page_source_path {
    use minijinja::Value;
    use neopoligengine::config::Config;
    use neopoligengine::file_set::FileSet;
    use neopoligengine::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn basic_source_path() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left =
            Some("leading-dir/Neopoligen/set2-test-site/content/source-path-check.neo".to_string());
        let right = site.page_source_path(&[Value::from("source-path-check")]);
        assert_eq!(left, right);
    }
}
