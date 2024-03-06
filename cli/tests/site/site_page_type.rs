// DEPRECATED: Move these tests under the
// page and just forward the site over to it
mod site_page_type {
    use minijinja::Value;
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn page_type_in_metadata() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("example".to_string());
        let right = site.page_type(&[Value::from("page-type-in-metadata")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_type_not_in_metadata() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("post".to_string());
        let right = site.page_type(&[Value::from("page-type-not-in-metadata")]);
        assert_eq!(left, right);
    }
}
