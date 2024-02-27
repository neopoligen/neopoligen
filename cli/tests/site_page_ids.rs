mod site_page_ids {
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;

    #[test]
    pub fn basic() {
        let file_set = FileSet::set2();
        let config = Config::site2_config();
        let site = Site::new(&file_set, &config);
        assert!(site.page_ids().contains(&"site2-home-page".to_string()));
    }
}
