mod site_page_href {
    use neopoligen_cli::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn basic_href() {
        let site = Site::site1();
        let left = Some("/en/id_index/?integration-test-site".to_string());
        let right = site.page_href("id_index");
        assert_eq!(left, right);
    }
}
