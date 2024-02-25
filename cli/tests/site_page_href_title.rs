mod page_href_title {
    use neopoligen_cli::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn basic_href_title() {
        let site = Site::site2();
        let left = Some("site-2-home-page".to_string());
        let right = site.page_href_title("id_index");
        assert_eq!(left, right);
    }
}
