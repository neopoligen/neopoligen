// NOTE: This test is for stubs that call
// directly over to .page_title(). Actual implementaiton
// of the .page_menu_title() functionality is TBD
mod site_page_menu_title {
    use minijinja::Value;
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn basic() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("Dev Test Site 2 Home Page".to_string());
        let right = site.page_menu_title(&[Value::from("site2-home-page")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn missing_page() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("(missing page)".to_string());
        let right = site.page_menu_title(&[Value::from("intentionally-missing-id")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn title_from_content() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("This Is A Title From A Bookmark Attribute".to_string());
        let right = site.page_menu_title(&[Value::from("title-from-section-attribute")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn first_few_words() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("Title from block content example".to_string());
        let right = site.page_menu_title(&[Value::from("title-from-block-content")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn title_from_metadata_id() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("no-title-just-id".to_string());
        let right = site.page_menu_title(&[Value::from("no-title-just-id")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn title_in_metadata() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("Metadata Override Title".to_string());
        let right = site.page_menu_title(&[Value::from("metadata-override-title")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn title_with_inline_span() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("Inline Spans Should Not Show UP".to_string());
        let right = site.page_menu_title(&[Value::from("title-with-inline-span")]);
        assert_eq!(left, right);
    }

    #[test]
    pub fn title_with_nested_inline_span() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let left = Some("Nested Inline Spans Should Not Show UP".to_string());
        let right = site.page_menu_title(&[Value::from("title-with-nested-inline-spans")]);
        assert_eq!(left, right);
    }
}
