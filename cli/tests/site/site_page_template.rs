mod site_page_template {
    use neopoligengine::config::Config;
    use neopoligengine::file_set::FileSet;
    use neopoligengine::site::Site;
    use pretty_assertions::assert_eq;
    use minijinja::Value;


    #[test]
    pub fn basic() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("pages/post/published.jinja".to_string());
        let right = site.page_template(&[Value::from("default-template")]);
        assert_eq!(left, right);
    }


    #[test]
    pub fn custom_template_type() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("pages/custom-template-type/published.jinja".to_string());
        let right = site.page_template(&[Value::from("custom-template-type")]);
        assert_eq!(left, right);
    }


    #[test]
    pub fn custom_template_status() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("pages/post/custom-template-status.jinja".to_string());
        let right = site.page_template(&[Value::from("custom-template-status")]);
        assert_eq!(left, right);
    }


    #[test]
    pub fn non_existent_type_template() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("pages/post/published.jinja".to_string());
        let right = site.page_template(&[Value::from("type-for-non-existent-template")]);
        assert_eq!(left, right);
    }

   
    #[test]
    pub fn non_existent_status_template() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("pages/post/published.jinja".to_string());
        let right = site.page_template(&[Value::from("status-for-non-existent-template")]);
        assert_eq!(left, right);
    }


}
