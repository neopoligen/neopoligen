mod site_page_ids {
    use neopoligengine::config::Config;
    use neopoligengine::file_set::FileSet;
    use neopoligengine::site::Site;

    #[test]
    pub fn basic() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        assert!(site.page_ids().contains(&"site2-home-page".to_string()));
    }
}
