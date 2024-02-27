mod site_page_template {
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;
    use pretty_assertions::assert_eq;
    use minijinja::Value;


    #[test]
    pub fn default() {
        let file_set = FileSet::set2();
        let config = Config::site2_config();
        let site = Site::new(&file_set, &config);
        let left = Some("pages/post/published.jinja".to_string());
        let right = site.page_template(&[Value::from("no-type-no-status")]);
        assert_eq!(left, right);
    }



}
