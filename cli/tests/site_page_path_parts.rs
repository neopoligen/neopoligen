mod site_page_path_parts {
    use minijinja::Value;
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn basic_path_parts() {
        let file_set = FileSet::set2();
        let config = Config::site2_config();
        let site = Site::new(&file_set, &config);
        let left = vec!["path", "parts", "example", "subfile.neo"];
        let right = site.page_path_parts(&[Value::from("page-parts-example")]);
        assert_eq!(left, right);
    }
}