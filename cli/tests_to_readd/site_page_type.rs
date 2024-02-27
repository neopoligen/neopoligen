mod site_page_type {
    use minijinja::Value;
    use neopoligen_cli::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn page_type_in_metadata() {
        let site = Site::site1();
        let left = Some("post".to_string());
        let right = site.page_type(&[Value::from("page-alfa")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_type_not_in_metadata() {
        let site = Site::site2();
        let left = Some("post".to_string());
        let right = site.page_type(&[Value::from("id-no-type-or-status-in-metadata")]);
        assert_eq!(left, right);
    }
}
