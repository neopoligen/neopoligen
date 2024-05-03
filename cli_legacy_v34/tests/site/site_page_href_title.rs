mod page_href_title {
    use neopoligengine::config::Config;
    use neopoligengine::file_set::FileSet;
    use neopoligengine::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn basic() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("dev-test-site-2-home-page".to_string());
        let right = site.page_href_title("site2-home-page");
        assert_eq!(left, right);
    }

    #[test]
    pub fn url_escape_test() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("url-escape-%2F-title-check".to_string());
        let right = site.page_href_title("url-escape-title-check");
        assert_eq!(left, right);
    }

}
