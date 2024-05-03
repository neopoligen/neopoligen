mod site_page_path_parts {
    use minijinja::Value;
    use neopoligengine::config::Config;
    use neopoligengine::file_set::FileSet;
    use neopoligengine::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn basic_path_parts() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = vec!["path", "parts", "example", "subfile.neo"];
        let right = site.page_path_parts(&[Value::from("page-parts-example")]);
        assert_eq!(left, right);
    }
}
