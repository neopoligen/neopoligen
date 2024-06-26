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
        let left = vec!["page", "folders", "example"];
        let right = site.page_folders(&[Value::from("page-folders-example")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn check_top_level_page_has_no_folders() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left: Vec<String> = vec![];
        let right = site.page_folders(&[Value::from("top-level-file")]);
        assert_eq!(left, right);
    }
}
