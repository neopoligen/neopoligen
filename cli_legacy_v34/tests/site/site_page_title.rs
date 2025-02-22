mod site_page_title {
    use minijinja::Value;
    use neopoligengine::config::Config;
    use neopoligengine::file_set::FileSet;
    use neopoligengine::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn title_from_content() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("This Is A Title From A Bookmark Attribute".to_string());
        let right = site.page_title(&[Value::from("title-from-section-attribute")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn first_few_words() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("Title from block content example".to_string());
        let right = site.page_title(&[Value::from("title-from-block-content")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn title_from_metadata_id() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("no-title-just-id".to_string());
        let right = site.page_title(&[Value::from("no-title-just-id")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn title_in_metadata() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("Metadata Override Title".to_string());
        let right = site.page_title(&[Value::from("metadata-override-title")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn title_with_inline_span() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("Inline Spans Should Not Show UP".to_string());
        let right = site.page_title(&[Value::from("title-with-inline-span")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn title_with_nested_inline_span() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("Nested Inline Spans Should Not Show UP".to_string());
        let right = site.page_title(&[Value::from("title-with-nested-inline-spans")]);
        assert_eq!(left, right);
    }
}
